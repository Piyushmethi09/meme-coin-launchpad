use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Transfer};
use crate::bonding_curve;
use crate::metadata;

pub fn mint_tokens(
    ctx: Context<MintTokens>,
    amount: u64,
    uri: String,
) -> Result<()> {
    let price = bonding_curve::calculate_price(ctx.accounts.token_account.total_supply, 1_000_000_000);
    msg!("Calculated price: {}", price);

    // Mint logic
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.destination.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::mint_to(cpi_ctx, amount)?;

    metadata::create_metadata(
        ctx.accounts.name.clone(),
        ctx.accounts.symbol.clone(),
        uri,
    )?;
    Ok(())
}

pub fn swap_sol_to_token(ctx: Context<Swap>, sol_amount: u64) -> Result<()> {
    msg!("Swapping SOL to Token for {} SOL", sol_amount);
    // Implement swap logic here
    Ok(())
}

pub fn swap_token_to_sol(ctx: Context<Swap>, token_amount: u64) -> Result<()> {
    msg!("Swapping Token to SOL for {} Tokens", token_amount);
    // Implement swap logic here
    Ok(())
}

pub fn migrate_to_raydium(ctx: Context<MigrateToRaydium>, amount: u64) -> Result<()> {
    msg!("Migrating {} tokens to Raydium", amount);
    crate::migration::migrate_tokens(ctx, amount)?;
    Ok(())
}
