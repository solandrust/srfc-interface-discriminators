//! Defines the `Interface` trait and related types and checks,
//! including the implementation of `pack(..)` and `unpack(..)`
//! for native and Shank programs

use std::collections::{HashMap, HashSet};
use syn::{parse_quote, ItemFn, Type, Variant};

use crate::error::SplInterfaceError;
use crate::instructions::*;

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
    /// The interface's namespace
    pub interface_namespace: String,
    /// The instruction's namespace
    pub instruction_namespace: String,
    /// The instruction's required arguments
    pub required_args: Vec<RequiredArg>,
}
impl InterfaceInstruction {
    /// Returns the 8-byte discriminator for the instruction
    pub fn discriminator(&self) -> [u8; 8] {
        let mut disc = [0u8; 8];
        disc.copy_from_slice(
            &solana_program::hash::hash(
                (self.interface_namespace.to_string() + ":" + &self.instruction_namespace)
                    .as_bytes(),
            )
            .to_bytes()[..8],
        );
        disc
    }
    /// Converts an instruction namespace and `&ItemFn` to an
    /// `InterfaceInstruction` for evaluation (Anchor)
    pub fn from_item_fn(
        interface_namespace: &String,
        instruction_namespace: &String,
        function: &ItemFn,
    ) -> Self {
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
            interface_namespace: interface_namespace.to_string(),
            instruction_namespace: instruction_namespace.to_string(),
            required_args,
        }
    }
    /// Converts an instruction namespace and `&Variant` to an
    /// `InterfaceInstruction` for evaluation (Native, Shank)
    pub fn from_variant(
        interface_namespace: &String,
        instruction_namespace: &String,
        variant: &Variant,
    ) -> Self {
        let mut required_args = vec![];
        for field in &variant.fields {
            if let Some(ident) = &field.ident {
                required_args.push((ident.to_string(), RequiredArgType::from(&field.ty)));
            }
        }
        Self {
            interface_namespace: interface_namespace.to_string(),
            instruction_namespace: instruction_namespace.to_string(),
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

/// Evaluates a program's interface instructions against any declared interfaces
pub fn evaluate_interface_instructions(
    declared_instructions: Vec<InterfaceInstruction>,
) -> Result<(), SplInterfaceError> {
    // Initialize a HashMap to keep track of all declared interfaces
    let mut declared_interfaces: HashMap<String, HashSet<InterfaceInstruction>> = HashMap::new();
    // Iterate through all declared instructions and
    // evaluate them against the declared interfaces
    for declared_ix in declared_instructions {
        if declared_ix.interface_namespace == SRFC20::NAMESPACE {
            process_declared_instruction::<SRFC20>(&mut declared_interfaces, declared_ix)?
        } else if declared_ix.interface_namespace == SRFC21::NAMESPACE {
            process_declared_instruction::<SRFC21>(&mut declared_interfaces, declared_ix)?
        } else if declared_ix.interface_namespace == SRFC22::NAMESPACE {
            process_declared_instruction::<SRFC22>(&mut declared_interfaces, declared_ix)?
        } else if declared_ix.interface_namespace == SRFC23::NAMESPACE {
            process_declared_instruction::<SRFC23>(&mut declared_interfaces, declared_ix)?
        } else {
            return Err(SplInterfaceError::InvalidInterfaceNamespace);
        }
    }
    // Make sure all declared interfaces have no remaining unmatched instructions
    for x in declared_interfaces.values() {
        if !x.is_empty() {
            dump_remaining_interface_instructions(declared_interfaces);
            return Err(SplInterfaceError::InstructionMissing);
        }
    }
    Ok(())
}

/// Processed a declared instruction by checking to see if it exists in the `HashMap`
/// and if it does, removing the instruction from the `HashSet`
fn process_declared_instruction<I: Interface>(
    declared_interfaces: &mut HashMap<String, HashSet<InterfaceInstruction>>,
    declared_ix: InterfaceInstruction,
) -> Result<(), SplInterfaceError> {
    match declared_interfaces.get_mut(&declared_ix.interface_namespace) {
        Some(set) => {
            if !set.remove(&declared_ix) {
                return Err(SplInterfaceError::InstructionNotFound);
            }
        }
        None => {
            let mut set = I::instruction_set();
            if !set.remove(&declared_ix) {
                println!("\n\nFound the following unknown interface instructions:\n");
                println!(
                    "  - {}::{}",
                    declared_ix.interface_namespace, declared_ix.instruction_namespace
                );
                println!("\n");
                return Err(SplInterfaceError::InstructionNotFound);
            }
            declared_interfaces.insert(declared_ix.interface_namespace, set);
        }
    }
    Ok(())
}

/// Dumps any remaining interface instructions in the evaluation
/// set for error reporting
fn dump_remaining_interface_instructions(
    declared_interfaces: HashMap<String, HashSet<InterfaceInstruction>>,
) {
    println!("\n\nThe following interface instructions were not implemented:\n");
    for (namespace, set) in declared_interfaces {
        for ix in set {
            println!("   - {}::{}", namespace, ix.instruction_namespace);
        }
    }
    println!("\n");
}
