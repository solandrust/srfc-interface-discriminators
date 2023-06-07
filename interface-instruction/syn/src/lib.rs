mod bindings;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, Attribute, ItemEnum};

#[derive(Debug)]
pub struct InterfaceInstructionBuilder {
    pub item_enum: ItemEnum,
}

impl From<ItemEnum> for InterfaceInstructionBuilder {
    fn from(item_enum: ItemEnum) -> Self {
        let variants = &item_enum.variants;
        process_variants(variants);
        Self { item_enum }
    }
}

fn process_variants(variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>) {
    for variant in variants {
        if let Some(interface_attr) = variant
            .attrs
            .iter()
            .find(|attr| attr.path.is_ident("interface"))
        {
            let _x = extract_interface_from_attribute(interface_attr);
        }
    }
}

fn extract_interface_from_attribute(interface_attr: &Attribute) -> Option<(String, String)> {
    if let Ok(meta) = interface_attr.parse_meta() {
        if let syn::Meta::List(meta_list) = meta {
            for nested_meta in meta_list.nested {
                if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = nested_meta {
                    println!("Interface: {:#?}", path.get_ident());
                    println!("path: {:#?}", path);
                    println!(
                        "Interface (segments): {}::{}",
                        path.segments[0].ident, path.segments[1].ident
                    );
                    return Some((
                        path.segments[0].ident.to_string(),
                        path.segments[1].ident.to_string(),
                    ));
                }
            }
            return None;
        } else {
            return None;
        }
    } else {
        return None;
    }
}

impl Parse for InterfaceInstructionBuilder {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(ItemEnum::parse(input)?.into())
    }
}

impl ToTokens for InterfaceInstructionBuilder {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend::<TokenStream>(self.into());
    }
}

impl From<&InterfaceInstructionBuilder> for TokenStream {
    fn from(_builder: &InterfaceInstructionBuilder) -> Self {
        quote! {}
    }
}
