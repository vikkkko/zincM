//!
//! The database contract storage event SELECT model.
//!

///
/// The database contract storage event SELECT input model.
///
#[derive(Debug)]
pub struct Input {
    /// The contract account ID referencing `contracts.account_id`.
    pub account_id: zksync_types::AccountId,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(account_id: zksync_types::AccountId) -> Self {
        Self { account_id }
    }
}

///
/// The database contract storage event SELECT output model.
///
#[derive(Debug, sqlx::FromRow)]
pub struct Output {
    /// The event name.
    pub name: String,
    /// The event value in JSON representation.
    pub value: serde_json::Value,
}
