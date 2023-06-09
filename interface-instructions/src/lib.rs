//! Crate for defining and implementing Solana program interfaces
//! for instructions
extern crate self as spl_interface_instructions;

mod error;
mod instructions;

pub use spl_interface_instructions_derive::interface_instruction;

use error::SplInterfaceError;
use std::collections::{HashMap, HashSet};
use syn::{parse_quote, ItemFn, Type};

/// Trait defining a Solana program interface
pub trait Interface {
    /// The interface's namespace
    const NAMESPACE: &'static str;
    /// The instructions required by the interface
    fn instructions() -> Vec<InterfaceInstruction>;
    /// Returns the instructions required by the interface
    /// as a set for evaluation
    fn instruction_set() -> HashSet<InterfaceInstruction> {
        let mut set = HashSet::new();
        for instruction in Self::instructions() {
            set.insert(instruction);
        }
        set
    }
}

/// Trait defining a Solana program interface instruction
#[derive(PartialEq, Eq, Hash)]
pub struct InterfaceInstruction {
    /// The instruction's namespace
    pub namespace: String,
    /// The instruction's required arguments
    pub required_args: Vec<RequiredArg>,
}
impl InterfaceInstruction {
    /// Returns the 8-byte discriminator for the instruction
    pub fn discriminator(&self, interface_namespace: &str) -> [u8; 8] {
        let mut disc = [0u8; 8];
        disc.copy_from_slice(
            &solana_program::hash::hash(
                (interface_namespace.to_string() + ":" + &self.namespace).as_bytes(),
            )
            .to_bytes()[..8],
        );
        disc
    }
    /// Converts an instruction namespace and `&ItemFn` to an
    /// `InterfaceInstruction` for evaluation
    fn from_item_fn(instruction_namespace: &str, function: &ItemFn) -> Self {
        let mut required_args = vec![];
        for arg in &function.sig.inputs {
            if let syn::FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(ident) = &*pat_type.pat {
                    required_args.push((
                        ident.ident.to_string(),
                        RequiredArgType::from(&*pat_type.ty),
                    ));
                }
            }
        }
        Self {
            namespace: instruction_namespace.to_string(),
            required_args,
        }
    }
}

/// A required argument for an instruction
type RequiredArg = (String, RequiredArgType);

/// The type of a required argument
#[derive(PartialEq, Eq, Hash)]
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

impl From<&Type> for RequiredArgType {
    fn from(value: &Type) -> Self {
        if value == &parse_quote! { u8 } {
            RequiredArgType::U8
        } else if value == &parse_quote! { u16 } {
            RequiredArgType::U16
        } else if value == &parse_quote! { u32 } {
            RequiredArgType::U32
        } else if value == &parse_quote! { u64 } {
            RequiredArgType::U64
        } else if value == &parse_quote! { u128 } {
            RequiredArgType::U128
        } else if value == &parse_quote! { String } {
            RequiredArgType::String
        } else if value == &parse_quote! { solana_program::pubkey::Pubkey } {
            RequiredArgType::Pubkey
        } else {
            panic!("Invalid type")
        }
    }
}

/// A declared instruction in a program
type DeclaredInstruction<'i> = (&'i str, &'i str, &'i ItemFn);

/// Evaluates a program's interface instructions against any declared interfaces
pub fn evaluate_interface_instructions<'i>(
    declared_instructions: Vec<DeclaredInstruction>,
) -> Result<(), SplInterfaceError> {
    // Initialize a HashMap to keep track of all declared interfaces
    let mut declared_interfaces: HashMap<String, HashSet<InterfaceInstruction>> = HashMap::new();
    // Iterate through all declared instructions and
    // evaluate them against the declared interfaces
    for declared_ix in declared_instructions {
        if declared_ix.1 == instructions::SRFC20::NAMESPACE {
            process_declared_instruction::<instructions::SRFC20>(
                &mut declared_interfaces,
                declared_ix,
            )?
        } else if declared_ix.1 == instructions::SRFC21::NAMESPACE {
            process_declared_instruction::<instructions::SRFC21>(
                &mut declared_interfaces,
                declared_ix,
            )?
        } else if declared_ix.1 == instructions::SRFC22::NAMESPACE {
            process_declared_instruction::<instructions::SRFC22>(
                &mut declared_interfaces,
                declared_ix,
            )?
        } else if declared_ix.1 == instructions::SRFC23::NAMESPACE {
            process_declared_instruction::<instructions::SRFC23>(
                &mut declared_interfaces,
                declared_ix,
            )?
        } else {
            return Err(SplInterfaceError::InvalidInterfaceNamespace);
        }
    }
    // Make sure all declared interfaces have no remaining unmatched instructions
    for x in declared_interfaces.values() {
        if x.len() > 0 {
            return Err(SplInterfaceError::InstructionMissing);
        }
    }
    Ok(())
}

/// Processed a declared instruction by checking to see if it exists in the `HashMap`
/// and if it does, removing the instruction from the `HashSet`
fn process_declared_instruction<I: Interface>(
    declared_interfaces: &mut HashMap<String, HashSet<InterfaceInstruction>>,
    declared_ix: DeclaredInstruction,
) -> Result<(), SplInterfaceError> {
    match declared_interfaces.get_mut(declared_ix.1) {
        Some(set) => {
            if set.remove(&InterfaceInstruction::from_item_fn(
                declared_ix.1,
                declared_ix.2,
            )) == false
            {
                return Err(SplInterfaceError::InstructionNotFound);
            }
        }
        None => {
            let mut set = I::instruction_set();
            if set.remove(&InterfaceInstruction::from_item_fn(
                declared_ix.1,
                declared_ix.2,
            )) == false
            {
                return Err(SplInterfaceError::InstructionNotFound);
            }
            declared_interfaces.insert(declared_ix.0.to_string(), set);
        }
    }
    Ok(())
}
