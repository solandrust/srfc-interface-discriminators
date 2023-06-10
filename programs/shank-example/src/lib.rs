mod processor;

use borsh_derive::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;
use solana_program::{entrypoint, pubkey::Pubkey};
use spl_interface_instructions::*;

use crate::processor::process;

/// Native programs can annotate their enum variants with the interface
/// annotations in order to enforce interface-based discriminators
/// for instructions.
///
/// They will also need to derive `InterfaceInstruction` in order to have
/// access to the custom `unpack(..)` function which can unpack an instruction
/// that is built with the interface discriminator
#[derive(BorshDeserialize, BorshSerialize, ShankInstruction, SplInterfaceInstruction)]
pub enum SampleProgramInstruction {
    /// This instruction implements the `token` interface's `mint_to`
    /// instruction and will have discriminator `hash(token:mint_to)[..8]`
    #[interface(token::mint_to)]
    #[account(0, name = "mint")]
    #[account(1, name = "authority")]
    #[account(2, name = "token_program")]
    MintTo {
        amount: u64,
        custom_arg_1: String,
        custom_arg_2: u64,
    },
    /// This instruction implements the `token` interface's `transfer`
    /// instruction and will have discriminator `hash(token:transfer)[..8]`
    #[interface(token::transfer)]
    #[account(0, name = "mint")]
    #[account(1, writable, name = "recipient")]
    #[account(2, writable, name = "from")]
    #[account(3, name = "authority")]
    #[account(4, name = "token_program")]
    Transfer {
        this_should_error: u8,
        custom_arg_1: Pubkey,
        custom_arg_2: u32,
    },
    /// This instruction implements the `token` interface's `burn`
    /// instruction and will have discriminator `hash(token:burn)[..8]`
    #[interface(token::burn)]
    #[account(0, name = "mint")]
    #[account(1, name = "authority")]
    #[account(2, name = "token_program")]
    Burn {
        amount: u64,
        custom_arg_1: Vec<String>,
    },
    /// This instruction implements the `associated_token` interface's `freeze`
    /// instruction and will have discriminator `hash(token:freeze)[..8]`
    #[interface(associated_token::freeze)]
    #[account(0, name = "mint")]
    #[account(1, writable, name = "target")]
    #[account(2, name = "authority")]
    #[account(3, name = "token_program")]
    Freeze {
        custom_arg_1: Pubkey,
        custom_arg_2: u32,
    },
    /// This instruction implements the `associated_token` interface's `thaw`
    /// instruction and will have discriminator `hash(token:thaw)[..8]`
    #[interface(associated_token::thaw)]
    #[account(0, name = "mint")]
    #[account(1, writable, name = "target")]
    #[account(2, name = "authority")]
    #[account(3, name = "token_program")]
    Thaw { custom_arg_1: Pubkey },
    Custom {
        custom_arg_1: Pubkey,
        custom_arg_2: u32,
    },
}

entrypoint!(process);
