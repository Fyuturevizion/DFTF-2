use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("FeePDDzUked1RW5dUvNbKQWPYFQki71RkVQUN1dGHtgj");

#[program]
mod fee_collector_program {
    use super::*;

    pub fn accumulate(ctx: Context<Accumulate>, amount: u64) -> Result<()> {
        // TODO: record fees in FeeBucket
        Ok(())
    }

    pub fn buyback_and_burn(ctx: Context<BuybackAndBurn>) -> Result<()> {
        // TODO: swap fee token for GDX via router_program
        // TODO: burn with spl_token::burn
        Ok(())
    }
}

#[account]
pub struct FeeBucket {
    pub token_mint: Pubkey,
    pub amount: u128,
}

#[derive(Accounts)]
pub struct Accumulate<'info> {
    #[account(mut)]
    pub bucket: Account<'info, FeeBucket>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BuybackAndBurn<'info> {
    #[account(mut)]
    pub bucket: Account<'info, FeeBucket>,
    #[account(mut)]
    pub gdx_mint: Account<'info, Mint>,
    #[account(mut)]
    pub burn_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

