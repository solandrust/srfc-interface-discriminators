use spl_interface_instructions::*;

#[interface_instruction]
pub enum SampleTokenA {
    #[interface(srfc20_token::mint_to)]
    MintTo { amount: u64 },
    #[interface(srfc20_token::transfer)]
    Transfer { amount: u64 },
}

#[interface_instruction]
pub enum SampleTokenB {
    #[interface(srfc22_associated_token::freeze)]
    Freeze,
    #[interface(srfc22_associated_token::thaw)]
    Thaw,
}

#[interface_instruction]
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

#[interface_instruction]
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
    #[interface(srfc23_token_metadata::create_metadata)]
    Custom {
        custom_arg_1: Pubkey,
        custom_arg_2: u32,
    },
}

#[interface_instruction]
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
