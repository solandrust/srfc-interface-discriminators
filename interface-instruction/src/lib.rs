extern crate self as spl_interface_instruction;

pub use spl_interface_instruction_derive::interface_instruction;
use syn::{parse_quote, Type};

pub trait Interface {
    const NAMESPACE: &'static str;
    fn instructions() -> Vec<InterfaceInstruction>;
    fn validate<I: Interface>(defined_interface: I) -> Result<(), SplInterfaceError> {
        todo!()
    }
}

pub struct InterfaceInstruction {
    pub namespace: &'static str,
    pub required_args: Vec<RequiredArg>,
}
impl InterfaceInstruction {
    fn discriminator(&self, interface_namespace: &str) -> [u8; 8] {
        discriminator(interface_namespace, self.namespace)
    }
}

type RequiredArg = (String, RequiredArgType);

pub enum RequiredArgType {
    /// A `u8` arg
    U8,
    /// A `u16` arg
    U16,
    /// A `u8` arg
    U32,
    /// A `u64` arg
    U64,
    /// A `u128` arg
    U128,
    /// A `String` arg
    String,
    /// A `Pubkey` arg
    Pubkey,
}

impl From<&RequiredArgType> for Type {
    fn from(value: &RequiredArgType) -> Self {
        match value {
            RequiredArgType::U8 => parse_quote! { u8 },
            RequiredArgType::U16 => parse_quote! { u16 },
            RequiredArgType::U32 => parse_quote! { u32 },
            RequiredArgType::U64 => parse_quote! { u64 },
            RequiredArgType::U128 => parse_quote! { u128 },
            RequiredArgType::String => parse_quote! { String },
            RequiredArgType::Pubkey => parse_quote! { solana_program::pubkey::Pubkey },
        }
    }
}

fn discriminator(interface_namespace: &str, instruction_namespace: &str) -> [u8; 8] {
    let mut disc = [0u8; 8];
    disc.copy_from_slice(
        &solana_program::hash::hash(
            (interface_namespace.to_string() + ":" + instruction_namespace).as_bytes(),
        )
        .to_bytes()[..8],
    );
    disc
}

#[derive(Clone, Debug, Eq, thiserror::Error, PartialEq)]
pub enum SplInterfaceError {
    #[error("Instruction not found")]
    InstructionNotFound,
    #[error("Instruction has incorrect number of arguments")]
    InstructionIncorrectNumberOfArgs,
    #[error("Instruction has incorrect argument type")]
    InstructionIncorrectArgType,
}

pub struct TokenInterface {}
impl Interface for TokenInterface {
    const NAMESPACE: &'static str = "token";

    fn instructions() -> Vec<InterfaceInstruction> {
        vec![
            InterfaceInstruction {
                namespace: "mint_to",
                required_args: vec![("amount".to_string(), RequiredArgType::U64)],
            },
            InterfaceInstruction {
                namespace: "transfer",
                required_args: vec![("amount".to_string(), RequiredArgType::U64)],
            },
        ]
    }
}

pub struct TestInterface {}
impl Interface for TestInterface {
    const NAMESPACE: &'static str = "test";

    fn instructions() -> Vec<InterfaceInstruction> {
        vec![
            InterfaceInstruction {
                namespace: "test",
                required_args: vec![("amount".to_string(), RequiredArgType::U64)],
            },
            InterfaceInstruction {
                namespace: "test2",
                required_args: vec![("amount".to_string(), RequiredArgType::U64)],
            },
        ]
    }
}

fn test() {
    let x = TestInterface {};
    match TokenInterface::validate(x) {
        Ok(_) => println!("ok"),
        Err(e) => println!("err: {}", e),
    }
}
