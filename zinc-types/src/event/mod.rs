//!
//! The contract value storage field.
//!

use crate::data::value::Value;
use serde::Deserialize;
use serde::Serialize;
use zksync_types::Address;
///
/// The contract value storage field representation.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractEvent {
    /// The field name.
    pub name: String,
    /// The field value.
    pub value: Value,
}

impl ContractEvent {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(name: String, value: Value) -> Self {
        Self { name, value }
    }
}
