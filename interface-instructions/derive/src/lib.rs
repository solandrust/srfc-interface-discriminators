//! Proc macro attribute for defining a Solana program interface
//! in native or Shank programs
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::ToTokens;
use spl_interface_instructions_syn::InterfaceInstructionBuilder;
use syn::parse_macro_input;

/// Proc macro attribute for defining a Solana program interface
/// in native or Shank programs
#[proc_macro_derive(SplInterfaceInstruction, attributes(interface))]
pub fn spl_interface_instruction(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as InterfaceInstructionBuilder)
        .to_token_stream()
        .into()
}
