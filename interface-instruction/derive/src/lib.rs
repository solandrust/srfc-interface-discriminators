extern crate proc_macro;

use proc_macro::TokenStream;
use quote::ToTokens;
use spl_interface_instruction_syn::InterfaceInstructionBuilder;
use syn::parse_macro_input;

/// Derive macro to add `InterfaceInstruction` trait
#[proc_macro_attribute]
pub fn interface_instruction(_: TokenStream, input: TokenStream) -> TokenStream {
    parse_macro_input!(input as InterfaceInstructionBuilder)
        .to_token_stream()
        .into()
}
