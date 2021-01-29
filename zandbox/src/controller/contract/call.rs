//!
//! The contract resource POST method `call` module.
//!

use std::collections::HashMap;

use actix_web::http::StatusCode;
use actix_web::web;
use num::BigInt;

use crate::contract::Contract;
use crate::database::model;
use crate::error::Error;
use crate::response::Response;
use crate::storage::Storage;
use zksync_types::Address;
///
/// The HTTP request handler.
///
/// Sequence:
/// 1. Get the contract and its data from the database.
/// 2. Extract the called method from its metadata and check if it is mutable.
/// 3. Parse the method input arguments.
/// 4. Run the method on the VM.
/// 5. Create a transactions array from the client and contract transfers.
/// 6. Send the transactions to zkSync and store its handles.
/// 7. Wait for all transactions to be committed.
/// 8. Update the contract storage state in the database.
/// 9. Send the contract method execution result back to the client.
///
pub async fn handle(
    app_data: crate::WebData,
    query: web::Query<zinc_types::CallRequestQuery>,
    body: web::Json<zinc_types::CallRequestBody>,
) -> crate::Result<serde_json::Value, Error> {
    let query = query.into_inner();
    let body = body.into_inner();
    let log_id = serde_json::to_string(&query.address).expect(zinc_const::panic::DATA_CONVERSION);

    let postgresql = app_data
        .read()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .postgresql
        .clone();
    let network = app_data
        .read()
        .expect(zinc_const::panic::SYNCHRONIZATION)
        .network;

    log::info!("[{}] Calling method `{}`", log_id, query.method);

    let contract = Contract::new(network, postgresql.clone(), query.address).await?;

    let method = match contract.build.methods.get(query.method.as_str()).cloned() {
        Some(method) => method,
        None => return Err(Error::MethodNotFound(query.method)),
    };
    if !method.is_mutable {
        return Err(Error::MethodIsImmutable(query.method));
    }

    let eth_address_bigint =
        BigInt::from_bytes_be(num::bigint::Sign::Plus, contract.eth_address.as_bytes());
    let mut arguments = zinc_types::Value::try_from_typed_json(body.arguments, method.input)
        .map_err(Error::InvalidInput)?;
    arguments.insert_contract_instance(eth_address_bigint.clone());

    let mut transaction_msgs: Vec<zinc_types::TransactionMsg> = Vec::new();
    for transaction in (&body.transaction).iter() {
        let transaction_msg = transaction.try_to_msg(&contract.wallet)?;
        log::debug!("transactionMsg:{:?}", transaction_msg);
        transaction_msgs.push(transaction_msg);
    }
    let sender = transaction_msgs.get(0).unwrap().sender;

    let output = contract
        .run_method(
            query.method,
            transaction_msgs,
            arguments,
            postgresql.clone(),
        )
        .await?;
    let mut transactions = body.transaction;

    let mut nonces = HashMap::with_capacity(output.storages.len());
    let mut created_instances = contract
        .execute_initial_deposits(output.initializers, &mut nonces, &mut transactions)
        .await?;
    let eth_private_keys: HashMap<zksync_types::Address, zksync_types::H256> = created_instances
        .iter()
        .map(|(address, instance)| (*address, instance.eth_private_key))
        .collect();
    contract
        .execute_main_batch(
            postgresql.clone(),
            output.transfers,
            transactions,
            nonces,
            eth_private_keys,
        )
        .await?;

    let mut transaction = postgresql.new_transaction().await?;
    for (address, storage) in output.storages.into_iter() {
        let address = zinc_types::address_from_slice(address.to_bytes_be().1.as_slice());

        if let Some(instance) = created_instances.remove(&address) {
            let account_id = instance.account_id;
            let storage = Storage::from_build(storage).into_database_insert(account_id);

            postgresql
                .insert_contract(instance, Some(&mut transaction))
                .await?;
            postgresql
                .insert_fields(storage, Some(&mut transaction))
                .await?;
        } else {
            let contract = postgresql
                .select_contract(
                    model::contract::select_one::Input::new(address),
                    Some(&mut transaction),
                )
                .await?;
            let storage = Storage::from_build(storage)
                .into_database_update(contract.account_id as zksync_types::AccountId);
            postgresql
                .update_fields(storage, Some(&mut transaction))
                .await?;
        }
    }
    for (address, events) in output.events.into_iter() {
        if let Some(instance) = created_instances.remove(&address) {
            let account_id = instance.account_id;
            let mut es = Vec::new();
            let mut index = 0i16;
            for event in events {
                log::debug!("event:{:?}", event);
                let event = model::event::insert::Input::new(
                    account_id,
                    index,
                    sender,
                    event.name,
                    event.value.into_json(),
                );
                index = index + 1;
                log::debug!(
                    "account_id{:?}--address:{:?}--event:{:?}",
                    account_id,
                    address,
                    event
                );
                es.push(event);
            }
            postgresql.insert_events(es, Some(&mut transaction)).await?;
        } else {
            let contract = postgresql
                .select_contract(
                    model::contract::select_one::Input::new(address),
                    Some(&mut transaction),
                )
                .await?;
            let mut es = Vec::new();
            let account_id = contract.account_id;
            let mut index = 0i16;
            for event in events {
                let value: zinc_types::Value = event.value;
                let a = value.into_json();
                log::debug!("a:{:?}", a);
                let event = model::event::insert::Input::new(
                    account_id as zksync_types::AccountId,
                    index,
                    sender,
                    event.name,
                    a,
                );
                index = index + 1;
                log::debug!(
                    "contract.account_id:{:?}--address:{:?}--event:{:?}",
                    contract.account_id,
                    address,
                    event
                );
                es.push(event);
            }
            postgresql.insert_events(es, Some(&mut transaction)).await?;
        }
    }

    transaction.commit().await?;

    let response = serde_json::json!({
        "output": output.result.into_json(),
    });

    log::info!("[{}] Call finished", log_id);
    Ok(Response::new_with_data(StatusCode::OK, response))
}
