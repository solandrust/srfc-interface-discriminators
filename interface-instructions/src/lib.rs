//! Crate for defining and implementing Solana program interfaces
//! for instructions
extern crate self as spl_interface_instructions;

// Simply exporting both the proc_macro crate and the syn crate
// so that everything is available downstream
pub use spl_interface_instructions_derive::SplInterfaceInstruction;
pub use spl_interface_instructions_registry::*;
pub use spl_interface_instructions_syn::*;
