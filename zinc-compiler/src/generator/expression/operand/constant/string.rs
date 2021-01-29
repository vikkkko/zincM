//!
//! The generator expression string constant operand.
//!

use std::cell::RefCell;
use std::rc::Rc;

use num::BigInt;
use num::One;
use num::Zero;

use zinc_types::Instruction;
use zinc_types::Push;

use crate::generator::zinc_vm::State as ZincVMState;
use crate::generator::IBytecodeWritable;
use crate::semantic::element::constant::string::String as SemanticStringConstant;

///
/// The generator expression string constant operand.
///
#[derive(Debug, Clone)]
pub struct String {
    /// The inner value.
    /// The inner string value.
    pub inner: ::std::string::String,
}

impl String {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(inner: ::std::string::String) -> Self {
        Self { inner }
    }

    ///
    /// Converts from the semantic boolean constant.
    ///
    pub fn from_semantic(string: &SemanticStringConstant) -> Self {
        Self::new(string.inner.clone())
    }
}

impl IBytecodeWritable for String {
    fn write_to_zinc_vm(self, state: Rc<RefCell<ZincVMState>>) {
        log::debug!("self.inner:{:?}", self.inner);
        let array: &[u8] = self.inner.as_bytes();
        log::debug!("array:{:?}", array);
        let bi = BigInt::from_signed_bytes_be(array);
        log::debug!("bi:{:?}", bi);
        let array2 = bi.to_signed_bytes_be();
        log::debug!("array2:{:?}", array2);
        let s = std::string::String::from_utf8(array2).expect("Found invalid UTF-8");
        log::debug!("s:{:?}", s);
        state.borrow_mut().push_instruction(
            Instruction::Push(Push::new(bi, zinc_types::ScalarType::String)),
            None,
        );
    }
}
