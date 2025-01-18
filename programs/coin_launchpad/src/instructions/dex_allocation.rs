use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use solana_program::pubkey::Pubkey;

pub fn migrate_to_raydium(ctx: Context<MigrateToRaydium>, amount: u64) -> Result<()> {
    // Validate that the bonding curve limit has been reached
    let price = bonding_curve::calculate_price(ctx.accounts.market_cap);
    if price < 12_000 {
        return Err(error!(TokenError::RaydiumMigrationFailed));
    }

    // Transfer tokens to Raydium
    let cpi_accounts = Transfer {
        from: ctx.accounts.creator_token_account.to_account_info(),
        to: ctx.accounts.raydium_token_account.to_account_info(),
        authority: ctx.accounts.creator.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct MigrateToRaydium<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub creator_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub raydium_token_account: Account<'info, token::TokenAccount>,
    pub token_program: Program<'info, token::Token>,
    pub market_cap: Account<'info, MarketCap>,
}

