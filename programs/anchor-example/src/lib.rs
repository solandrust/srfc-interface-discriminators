use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

/// Anchor programs can annotate their functions with the interface
/// annotations in order to enforce interface-based discriminators
/// for instructions
#[program]
pub mod anchor_example {
    use super::*;

    /// This instruction implements the `token` interface's `mint_to`
    /// instruction and will have discriminator `hash(token:mint_to)[..8]`
    #[interface(token::mint_to)]
    pub fn my_custom_mint_to(
        ctx: Context<MintTo>,
        custom_arg_1: String,
        custom_arg_2: u64,
    ) -> Result<()> {
        Ok(())
    }

    /// This instruction implements the `token` interface's `transfer`
    /// instruction and will have discriminator `hash(token:transfer)[..8]`
    #[interface(token::transfer)]
    pub fn my_custom_transfer(
        ctx: Context<Transfer>,
        custom_arg_1: Pubkey,
        custom_arg_2: u32,
    ) -> Result<()> {
        Ok(())
    }

    /// This instruction implements the `token` interface's `burn`
    /// instruction and will have discriminator `hash(token:burn)[..8]`
    #[interface(token::burn)]
    pub fn my_custom_burn(ctx: Context<Burn>, custom_arg_1: Vec<String>) -> Result<()> {
        Ok(())
    }

    /// This instruction implements the `associated_token` interface's `freeze`
    /// instruction and will have discriminator `hash(token:freeze)[..8]`
    #[interface(associated_token::freeze)]
    pub fn my_custom_freeze(
        ctx: Context<Freeze>,
        custom_arg_1: Pubkey,
        custom_arg_2: u32,
    ) -> Result<()> {
        Ok(())
    }

    /// This instruction implements the `associated_token` interface's `thaw`
    /// instruction and will have discriminator `hash(token:thaw)[..8]`
    #[interface(associated_token::thaw)]
    pub fn my_custom_thaw(ctx: Context<Thaw>, custom_arg_1: Pubkey) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintTo {
    pub mint: Account<'info, Mint>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Transfer {
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
    )]
    pub recipient: Account<'info, AssociatedToken>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::owner = authority,
    )]
    pub from: Account<'info, AssociatedToken>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Burn {
    pub mint: Account<'info, Mint>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Freeze {
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
    )]
    pub target: Account<'info, AssociatedToken>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Thaw {
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
    )]
    pub target: Account<'info, AssociatedToken>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
