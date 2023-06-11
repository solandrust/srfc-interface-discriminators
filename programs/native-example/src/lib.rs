mod processor;

use borsh_derive::{BorshDeserialize, BorshSerialize};
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
#[derive(BorshDeserialize, BorshSerialize, SplInterfaceInstruction)]
pub enum SampleProgramInstruction {
    /// This instruction implements the `token` interface's `mint_to`
    /// instruction and will have discriminator `hash(token:mint_to)[..8]`
    #[interface(srfc20_token::mint_to)]
    MintTo { amount: u64 },
    /// This instruction implements the `token` interface's `transfer`
    /// instruction and will have discriminator `hash(token:transfer)[..8]`
    #[interface(srfc20_token::transfer)]
    Transfer { amount: u64 },
    /// This instruction implements the `token` interface's `burn`
    /// instruction and will have discriminator `hash(token:burn)[..8]`
    #[interface(srfc21_token::burn)]
    Burn { amount: u64 },
    /// This instruction implements the `associated_token` interface's `freeze`
    /// instruction and will have discriminator `hash(token:freeze)[..8]`
    #[interface(srfc22_associated_token::freeze)]
    Freeze,
    /// This instruction implements the `associated_token` interface's `thaw`
    /// instruction and will have discriminator `hash(token:thaw)[..8]`
    #[interface(srfc22_associated_token::thaw)]
    Thaw,
    Custom {
        custom_arg_1: Pubkey,
        custom_arg_2: u32,
    },
}

entrypoint!(process);
