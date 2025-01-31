//!
//! The virtual machine contract input.
//!

use std::collections::HashMap;

use zinc_types::TransactionMsg;
use zksync_types::Address;

///
/// The virtual machine contract input.
///
pub struct Input {
    /// The contract method arguments, which is witness for now.
    pub arguments: zinc_types::Value,
    /// The contract storages after executing a method.
    pub storages: HashMap<Address, zinc_types::Value>,
    /// The contract method name which is called.
    pub method_name: String,
    /// The contract input transaction.
    pub transactions: Vec<TransactionMsg>,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        arguments: zinc_types::Value,
        storages: HashMap<Address, zinc_types::Value>,
        method_name: String,
        mut transactions: Vec<TransactionMsg>,
    ) -> Self {
        if transactions.len() == 1 {
            transactions.push(TransactionMsg::default())
        }
        Self {
            arguments,
            storages,
            method_name,
            transactions,
        }
    }
}
