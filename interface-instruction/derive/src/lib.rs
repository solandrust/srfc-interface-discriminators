extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemEnum};

/// Derive macro to add `InterfaceInstruction` trait
#[proc_macro_derive(InterfaceInstruction)]
pub fn derive_interface_instruction(input: TokenStream) -> TokenStream {
    generate_tokens(parse_macro_input!(input as ItemEnum)).into()
}

fn generate_tokens(item_enum: ItemEnum) {
    todo!()
}
