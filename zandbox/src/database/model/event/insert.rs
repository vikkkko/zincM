#[derive(Debug)]
pub struct Input {
    pub account_id: zksync_types::AccountId,
    pub index: i16,
    pub sender: zksync_types::Address,
    pub name: String,
    pub value: serde_json::Value,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        account_id: zksync_types::AccountId,
        index: i16,
        sender: zksync_types::Address,
        name: String,
        value: serde_json::Value,
    ) -> Self {
        Self {
            account_id,
            index,
            sender,
            name,
            value,
        }
    }
}
