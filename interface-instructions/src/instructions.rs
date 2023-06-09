use crate::{Interface, InterfaceInstruction, RequiredArgType};

pub struct SRFC20 {}
impl Interface for SRFC20 {
    const NAMESPACE: &'static str = "srfc20_token";

    fn instructions() -> Vec<InterfaceInstruction> {
        vec![
            InterfaceInstruction {
                namespace: "mint_to".to_string(),
                required_args: vec![("amount".to_string(), RequiredArgType::U64)],
            },
            InterfaceInstruction {
                namespace: "transfer".to_string(),
                required_args: vec![("amount".to_string(), RequiredArgType::U64)],
            },
        ]
    }
}

pub struct SRFC21 {}
impl Interface for SRFC21 {
    const NAMESPACE: &'static str = "srfc21_token";

    fn instructions() -> Vec<InterfaceInstruction> {
        vec![InterfaceInstruction {
            namespace: "burn".to_string(),
            required_args: vec![("amount".to_string(), RequiredArgType::U64)],
        }]
    }
}

pub struct SRFC22 {}
impl Interface for SRFC22 {
    const NAMESPACE: &'static str = "srfc22_associated_token";

    fn instructions() -> Vec<InterfaceInstruction> {
        vec![
            InterfaceInstruction {
                namespace: "freeze".to_string(),
                required_args: vec![],
            },
            InterfaceInstruction {
                namespace: "thaw".to_string(),
                required_args: vec![],
            },
        ]
    }
}

pub struct SRFC23 {}
impl Interface for SRFC23 {
    const NAMESPACE: &'static str = "srfc23_token_metadata";

    fn instructions() -> Vec<InterfaceInstruction> {
        vec![
            InterfaceInstruction {
                namespace: "create_metadata".to_string(),
                required_args: vec![
                    ("name".to_string(), RequiredArgType::String),
                    ("symbol".to_string(), RequiredArgType::String),
                    ("uri".to_string(), RequiredArgType::String),
                ],
            },
            InterfaceInstruction {
                namespace: "update_metadata".to_string(),
                required_args: vec![
                    ("name".to_string(), RequiredArgType::String),
                    ("symbol".to_string(), RequiredArgType::String),
                    ("uri".to_string(), RequiredArgType::String),
                ],
            },
        ]
    }
}
