use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

/// Anchor programs can annotate their functions with the interface
/// annotations in order to enforce interface-based discriminators
/// for instructions
#[program]
pub mod anchor_example {
    use super::*;

    /// This instruction implements the `token` interface's `mint_to`
    /// instruction and will have discriminator `hash(token:mint_to)[..8]`
    #[interface(srfc20_token::mint_to)]
    pub fn my_custom_mint_to(ctx: Context<MintTo>, amount: u64) -> Result<()> {
        Ok(())
    }

    /// This instruction implements the `token` interface's `transfer`
    /// instruction and will have discriminator `hash(token:transfer)[..8]`
    #[interface(srfc20_token::transfer)]
    pub fn my_custom_transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        Ok(())
    }

    /// This instruction implements the `token` interface's `burn`
    /// instruction and will have discriminator `hash(token:burn)[..8]`
    #[interface(srfc21_token::burn)]
    pub fn my_custom_burn(ctx: Context<Burn>, amount: u64) -> Result<()> {
        Ok(())
    }

    /// This instruction implements the `associated_token` interface's `freeze`
    /// instruction and will have discriminator `hash(token:freeze)[..8]`
    #[interface(srfc22_associated_token::freeze)]
    pub fn my_custom_freeze(ctx: Context<Freeze>) -> Result<()> {
        Ok(())
    }

    /// This instruction implements the `associated_token` interface's `thaw`
    /// instruction and will have discriminator `hash(token:thaw)[..8]`
    #[interface(srfc22_associated_token::thaw)]
    pub fn my_custom_thaw(ctx: Context<Thaw>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintTo<'info> {
    pub mint: Account<'info, Mint>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = authority,
    )]
    pub from: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Burn<'info> {
    pub mint: Account<'info, Mint>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Freeze<'info> {
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub target: Account<'info, TokenAccount>,
    pub owner: SystemAccount<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Thaw<'info> {
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub target: Account<'info, TokenAccount>,
    pub owner: SystemAccount<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
