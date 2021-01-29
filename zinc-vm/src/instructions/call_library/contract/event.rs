//!
//! The `<Contract>::event` function call.
//!

use std::collections::HashMap;

use crate::core::execution_state::ExecutionState;
use crate::error::Error;
use crate::error::MalformedBytecode;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::instructions::call_library::INativeCallable;
use crate::IEngine;
use franklin_crypto::bellman::pairing::ff::Field;
use franklin_crypto::bellman::ConstraintSystem;
use num::bigint::ToBigInt;
use num::BigInt;
use zinc_types::ScalarValue;
use zksync::web3::types::Address;

pub struct Event {
    input_size: usize,
}

impl Event {
    pub fn new(inputs_count: usize) -> Result<Self, Error> {
        inputs_count
            .checked_sub(1)
            .map(|input_size| Self { input_size })
            .ok_or_else(|| {
                MalformedBytecode::InvalidArguments(
                    "array::Event expects at least 2 arguments".into(),
                )
                .into()
            })
    }
}

impl<E: IEngine, S: IMerkleTree<E>> INativeCallable<E, S> for Event {
    fn call<CS>(
        &self,
        _cs: CS,
        state: &mut ExecutionState<E>,
        _storages: Option<HashMap<BigInt, &mut S>>,
    ) -> Result<(), Error>
    where
        CS: ConstraintSystem<E>,
    {
        log::debug!("input_size:{}", self.input_size);

        let mut argsCount = self.input_size - 1;
        let mut args = Vec::with_capacity(argsCount);
        while argsCount != 0 {
            argsCount = argsCount - 1;
            let arg = state.evaluation_stack.pop()?.try_into_value()?;
            let arg = zinc_types::ScalarValue::Integer(
                arg.to_bigint().expect(zinc_const::panic::DATA_CONVERSION),
                zinc_types::IntegerType::ETH_ADDRESS,
            );
            log::debug!("arg:{:?}", arg);
            args.push(zinc_types::Value::Scalar(arg));
        }
        args.reverse();
        let name = state.evaluation_stack.pop()?.try_into_value()?;
        let contract = state.evaluation_stack.pop()?.try_into_value()?;
        log::debug!("contract:{:?}----name:{:?}", contract, name);
        let contract = zinc_types::address_from_slice(
            contract
                .to_bigint()
                .expect(zinc_const::panic::DATA_CONVERSION)
                .to_bytes_be()
                .1
                .as_slice(),
        );
        let name = zinc_types::string_from_slice(
            name.to_bigint()
                .expect(zinc_const::panic::DATA_CONVERSION)
                .to_bytes_be()
                .1
                .as_slice(),
        );

        log::debug!("1contract:{:?}----name:{:?}", contract, name);
        let event = zinc_types::ContractEventType::new(name, zinc_types::Value::Array(args));
        if state.events.contains_key(&contract) {
            state.events.get_mut(&contract).unwrap().push(event);
        } else {
            state.events.insert(contract, vec![event]);
        }

        if state
            .conditions_stack
            .iter()
            .map(|value| value.get_value().expect(zinc_const::panic::DATA_CONVERSION))
            .all(|value| !value.is_zero())
        {}

        Ok(())
    }
}
