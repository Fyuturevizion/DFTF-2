use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("HvLTpRqvi7F9YY7oMh6xrFwSEopgJPk91RyJDqqH3wyB");

#[program]
mod vault_program {
    use super::*;

    pub fn create_vault(
        ctx: Context<CreateVault>,
        vault_id: u64,
        strategy_hash: [u8; 32],
    ) -> Result<()> {
        // TODO: initialize vault account and mint authority
        // store strategy hash and set authority
        Ok(())
    }

    pub fn deposit_lp(ctx: Context<DepositLp>, amount: u64) -> Result<()> {
        // TODO: transfer LP tokens and mint vLP
        Ok(())
    }

    pub fn withdraw_lp(ctx: Context<WithdrawLp>, shares: u64) -> Result<()> {
        // TODO: burn vLP and transfer underlying LP
        Ok(())
    }

    pub fn rebalance(ctx: Context<Rebalance>, new_weights: Vec<u16>) -> Result<()> {
        // TODO: adjust strategy and charge performance fee
        Ok(())
    }
}

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub strategy_hash: [u8; 32],
    pub total_lp_value: u128,
    pub total_shares: u128,
}

#[derive(Accounts)]
pub struct CreateVault<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 16 + 16,
        seeds = [b"vault", authority.key().as_ref(), &vault_id.to_le_bytes()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositLp<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub lp_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vlp_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_vlp: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct WithdrawLp<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub lp_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vlp_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_vlp: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Rebalance<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
}

#[event]
pub struct VaultFee {
    pub vault: Pubkey,
    pub amount: u64,
}
