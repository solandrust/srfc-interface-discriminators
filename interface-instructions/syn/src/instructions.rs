//! Community-defined interfaces from sRFC workflow

use crate::interface::{Interface, InterfaceInstruction, RequiredArgType};

/// The sRFC 20 Token Interface
pub struct SRFC20 {}
impl Interface for SRFC20 {
    const NAMESPACE: &'static str = "srfc20_token";

    fn instructions() -> Vec<InterfaceInstruction> {
        vec![
            InterfaceInstruction {
                interface_namespace: Self::NAMESPACE.to_string(),
                instruction_namespace: "mint_to".to_string(),
                required_args: vec![("amount".to_string(), RequiredArgType::U64)],
            },
            InterfaceInstruction {
                interface_namespace: Self::NAMESPACE.to_string(),
                instruction_namespace: "transfer".to_string(),
                required_args: vec![("amount".to_string(), RequiredArgType::U64)],
            },
        ]
    }
}

/// The sRFC 21 Token Interface
pub struct SRFC21 {}
impl Interface for SRFC21 {
    const NAMESPACE: &'static str = "srfc21_token";

    fn instructions() -> Vec<InterfaceInstruction> {
        vec![InterfaceInstruction {
            interface_namespace: Self::NAMESPACE.to_string(),
            instruction_namespace: "burn".to_string(),
            required_args: vec![("amount".to_string(), RequiredArgType::U64)],
        }]
    }
}

/// The sRFC 22 Associated Token Interface
pub struct SRFC22 {}
impl Interface for SRFC22 {
    const NAMESPACE: &'static str = "srfc22_associated_token";

    fn instructions() -> Vec<InterfaceInstruction> {
        vec![
            InterfaceInstruction {
                interface_namespace: Self::NAMESPACE.to_string(),
                instruction_namespace: "freeze".to_string(),
                required_args: vec![],
            },
            InterfaceInstruction {
                interface_namespace: Self::NAMESPACE.to_string(),
                instruction_namespace: "thaw".to_string(),
                required_args: vec![],
            },
        ]
    }
}

/// The sRFC 23 Token Metadata Interface
pub struct SRFC23 {}
impl Interface for SRFC23 {
    const NAMESPACE: &'static str = "srfc23_token_metadata";

    fn instructions() -> Vec<InterfaceInstruction> {
        vec![
            InterfaceInstruction {
                interface_namespace: Self::NAMESPACE.to_string(),
                instruction_namespace: "create_metadata".to_string(),
                required_args: vec![
                    ("name".to_string(), RequiredArgType::String),
                    ("symbol".to_string(), RequiredArgType::String),
                    ("uri".to_string(), RequiredArgType::String),
                ],
            },
            InterfaceInstruction {
                interface_namespace: Self::NAMESPACE.to_string(),
                instruction_namespace: "update_metadata".to_string(),
                required_args: vec![
                    ("name".to_string(), RequiredArgType::String),
                    ("symbol".to_string(), RequiredArgType::String),
                    ("uri".to_string(), RequiredArgType::String),
                ],
            },
        ]
    }
}
