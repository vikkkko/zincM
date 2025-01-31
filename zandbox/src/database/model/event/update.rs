//!
//! The database contract storage event UPDATE model.
//!

///
/// The database contract storage event UPDATE input model.
///
#[derive(Debug)]
pub struct Input {
    /// The contract account ID referencing `contracts.account_id`.
    pub account_id: zksync_types::AccountId,
    /// The event index in the contract storage.
    pub index: i16,
    /// The event value in JSON representation.
    pub value: serde_json::Value,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(account_id: zksync_types::AccountId, index: i16, value: serde_json::Value) -> Self {
        Self {
            account_id,
            index,
            value,
        }
    }
}
