//!
//! The semantic analyzer `<Contract>::event` intrinsic function element.
//!

use std::fmt;

use zinc_lexical::Location;
use zinc_types::LibraryFunctionIdentifier;

use crate::semantic::element::argument_list::ArgumentList;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error;

///
/// The semantic analyzer `<Contract>::event` intrinsic function element.
///
#[derive(Debug, Clone)]
pub struct Function {
    /// The location where the function is called.
    pub location: Option<Location>,
    /// The unique intrinsic function identifier.
    pub library_identifier: LibraryFunctionIdentifier,
    /// The function identifier.
    pub identifier: &'static str,
}

impl Default for Function {
    fn default() -> Self {
        Self {
            location: None,
            library_identifier: LibraryFunctionIdentifier::ContractEvent,
            identifier: Self::IDENTIFIER,
        }
    }
}

impl Function {
    /// The function identifier.
    pub const IDENTIFIER: &'static str = "event";

    /// The position of the `contract` argument in the function argument list.
    pub const ARGUMENT_INDEX_CONTRACT: usize = 0;

    /// The position of the `name` argument in the function argument list.
    pub const ARGUMENT_INDEX_NAME: usize = 1;

    /// The expected number of the function arguments.
    pub const ARGUMENT_COUNT: usize = 2;

    ///
    /// Calls the function with the `argument_list`, validating the call.
    ///
    pub fn call(self, location: Location, argument_list: ArgumentList) -> Result<Type, Error> {
        let mut actual_params = Vec::with_capacity(argument_list.arguments.len());
        for (index, element) in argument_list.arguments.into_iter().enumerate() {
            let location = element.location();

            let r#type = match element {
                Element::Value(value) => value.r#type(),
                Element::Constant(constant) => constant.r#type(),
                element => {
                    return Err(Error::FunctionArgumentNotEvaluable {
                        location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                        function: self.identifier.to_owned(),
                        position: index + 1,
                        found: element.to_string(),
                    })
                }
            };

            actual_params.push((r#type, location));
        }

        match actual_params.get(Self::ARGUMENT_INDEX_CONTRACT) {
            Some((Type::Contract(_), _location)) => {
                log::debug!("((Type::Contract(_), _location):{:?}", _location);
            }
            Some((r#type, location)) => {
                log::debug!("1");
                return Err(Error::FunctionArgumentType {
                    location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "contract".to_owned(),
                    position: Self::ARGUMENT_INDEX_NAME + 1,
                    expected: "{contract}".to_owned(),
                    found: r#type.to_string(),
                });
            }
            None => {
                return Err(Error::FunctionArgumentCount {
                    location,
                    function: self.identifier.to_owned(),
                    expected: Self::ARGUMENT_COUNT,
                    found: actual_params.len(),
                    reference: None,
                })
            }
        }

        match actual_params.get(Self::ARGUMENT_INDEX_NAME) {
            Some((Type::String(_), _location)) => {
                // string.to_owned()
            }
            Some((r#type, location)) => {
                return Err(Error::FunctionArgumentType {
                    location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "name".to_owned(),
                    position: Self::ARGUMENT_INDEX_NAME + 1,
                    expected: Type::string(None).to_string(),
                    found: r#type.to_string(),
                })
            }
            None => {
                return Err(Error::FunctionArgumentCount {
                    location,
                    function: self.identifier.to_owned(),
                    expected: Self::ARGUMENT_COUNT,
                    found: actual_params.len(),
                    reference: None,
                })
            }
        }

        // if actual_params.len() > Self::ARGUMENT_COUNT {
        //     return Err(Error::FunctionArgumentCount {
        //         location,
        //         function: self.identifier.to_owned(),
        //         expected: Self::ARGUMENT_COUNT,
        //         found: actual_params.len(),
        //         reference: None,
        //     });
        // }

        Ok(Type::unit(self.location))
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(contract: u160, name: string)", self.identifier)
    }
}
