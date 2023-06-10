use solana_program::pubkey::Pubkey;
use spl_interface_instructions::*;

#[derive(SplInterfaceInstruction)]
pub enum SampleTokenA {
    #[interface(srfc20_token::mint_to)]
    MintTo { amount: u64 },
    #[interface(srfc20_token::transfer)]
    Transfer { amount: u64 },
}

#[derive(SplInterfaceInstruction)]
pub enum SampleTokenB {
    #[interface(srfc22_associated_token::freeze)]
    Freeze,
    #[interface(srfc22_associated_token::thaw)]
    Thaw,
}

#[derive(SplInterfaceInstruction)]
pub enum SampleTokenC {
    #[interface(srfc23_token_metadata::create_metadata)]
    CreateMetadata {
        name: String,
        symbol: String,
        uri: String,
    },
    #[interface(srfc23_token_metadata::update_metadata)]
    UpdateMetadata {
        name: String,
        symbol: String,
        uri: String,
    },
}

#[derive(SplInterfaceInstruction)]
pub enum SampleTokenD {
    #[interface(srfc20_token::mint_to)]
    MintTo { amount: u64 },
    #[interface(srfc20_token::transfer)]
    Transfer { amount: u64 },
    #[interface(srfc21_token::burn)]
    Burn { amount: u64 },
    #[interface(srfc22_associated_token::freeze)]
    Freeze,
    #[interface(srfc22_associated_token::thaw)]
    Thaw,
    Custom {
        custom_arg_1: Pubkey,
        custom_arg_2: u32,
    },
}

#[derive(SplInterfaceInstruction)]
pub enum SampleTokenE {
    #[interface(srfc20_token::mint_to)]
    MintTo { amount: u64 },
    #[interface(srfc20_token::transfer)]
    Transfer { amount: u64 },
    #[interface(srfc21_token::burn)]
    Burn { amount: u64 },
    #[interface(srfc22_associated_token::freeze)]
    Freeze,
    #[interface(srfc22_associated_token::thaw)]
    Thaw,
    #[interface(srfc23_token_metadata::create_metadata)]
    CreateMetadata {
        name: String,
        symbol: String,
        uri: String,
    },
    #[interface(srfc23_token_metadata::update_metadata)]
    UpdateMetadata {
        name: String,
        symbol: String,
        uri: String,
    },
    Custom {
        custom_arg_1: Pubkey,
        custom_arg_2: u32,
    },
}

#[test]
fn test_compiles() {
    let _a = SampleTokenA::instructions();
    let _b = SampleTokenB::instructions();
    let _c = SampleTokenC::instructions();
    let _d = SampleTokenD::instructions();
    let _e = SampleTokenE::instructions();
}
