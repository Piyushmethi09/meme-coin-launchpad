use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use solana_program::pubkey::Pubkey;
pub mod admin;

use admin::*;

declare_id!("4EAQ6275rL3DRA7VAc8mmC6P8xySbokTbSrEo6irBpy2");

#[program]
pub mod meme_coin_launchpad {
    use super::*;

    // Minting coins (0.01 SOL per 1 billion coins)
    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        let creator_balance = ctx.accounts.creator_token_account.amount;
        let total_supply = ctx.accounts.token_supply.amount;
        
        // Ensure creator has enough SOL balance
        let fee = 0.01 * 10u64.pow(9); // Fee calculation for minting 1 billion tokens
        let creator_sol_balance = ctx.accounts.payer.to_account_info().lamports();

        if creator_sol_balance < fee {
            return Err(ProgramError::InsufficientFunds.into());
        }

        // Transfer the minting fee to the platform treasury
        let treasury = &ctx.accounts.treasury;
        token::transfer(
            ctx.accounts.transfer_to_treasury_context(),
            fee as u64,
        )?;

        // Mint the tokens
        let mint = &ctx.accounts.mint;
        let token_program = &ctx.accounts.token_program;
        let token_account = &ctx.accounts.creator_token_account;

        token::mint_to(
            token_program,
            mint,
            token_account,
            ctx.accounts.payer.to_account_info(),
            &[&ctx.accounts.payer],
            amount,
        )?;

        // Validate maximum supply
        if total_supply + amount > 1_000_000_000 {
            return Err(ProgramError::InvalidArgument.into()); // Maximum token supply exceeded
        }

        Ok(())
    }

    pub fn initialize_admin(ctx: Context<InitializeAdmin>, treasury_wallet: Pubkey) -> Result<()> {
        let admin_config = &mut ctx.accounts.admin_config;
        admin_config.admin = *ctx.accounts.admin.key;
        admin_config.treasury_wallet = treasury_wallet;
        admin_config.minting_fee = 10_000_000; // Default minting fee: 0.01 SOL
        admin_config.trading_fee = 100;        // Default trading fee: 1%
        admin_config.paused = false;
        Ok(())
    }

    pub fn update_treasury(ctx: Context<UpdateTreasury>, new_treasury: Pubkey) -> Result<()> {
        update_treasury(ctx, new_treasury)
    }

    pub fn update_minting_fee(ctx: Context<UpdateMintingFee>, new_fee: u64) -> Result<()> {
        update_minting_fee(ctx, new_fee)
    }

    pub fn pause_contract(ctx: Context<PauseContract>, pause: bool) -> Result<()> {
        pause_contract(ctx, pause)
    }
}

    // Migration function (tokens to Raydium DEX)
    pub fn migrate_to_raydium(ctx: Context<MigrateToRaydium>, nonce: u8) -> Result<()> {
        let required_tokens_for_dex = ctx.accounts.token_supply.amount / 100; // 1% token collection
        
        // Check if the 1% token allocation to treasury is met
        let treasury_balance = ctx.accounts.treasury.amount;
        if treasury_balance < required_tokens_for_dex {
            return Err(ProgramError::InsufficientFunds.into());
        }

        // Verify the market cap threshold (e.g., $69,000)
        let market_cap_threshold = 69000;
        let current_market_cap = calculate_market_cap(ctx.accounts.token_supply.amount);
        if current_market_cap < market_cap_threshold {
            return Err(ProgramError::InvalidArgument.into());
        }

        // Migrate tokens to Raydium
        token::transfer(
            ctx.accounts.transfer_to_raydium_context(),
            ctx.accounts.token_supply.amount,
        )?;

        Ok(())
    }

    // Function to calculate market cap (stub for now)
    fn calculate_market_cap(total_supply: u64) -> u64 {
        total_supply * 1 // Assume price is 1 USD for simplicity
    }

    // Swap functionality (SOL to token & token to SOL)
    pub fn swap(ctx: Context<SwapTokens>, amount_in: u64, direction: String) -> Result<()> {
        let swap_fee = 0.0001; // 0.0001 SOL swap fee
        if direction == "SOL_to_token" {
            // Swap SOL to token
            token::transfer(
                ctx.accounts.transfer_to_token_context(),
                amount_in - swap_fee as u64,
            )?;
        } else if direction == "token_to_SOL" {
            // Swap token to SOL
            token::transfer(
                ctx.accounts.transfer_to_sol_context(),
                amount_in - swap_fee as u64,
            )?;
        }
        Ok(())
    }

    // Safe badge logic (5% purchase limit)
    pub fn assign_safe_badge(ctx: Context<AssignSafeBadge>) -> Result<()> {
        let creator_balance = ctx.accounts.creator_token_account.amount;
        let total_supply = ctx.accounts.token_supply.amount;

        if creator_balance > total_supply / 20 {  // 5% of total supply
            return Err(ProgramError::InvalidArgument.into());
        }

        // Assign "Safe" badge
        ctx.accounts.metadata.safe_badge = true;
        Ok(())
    }

    // Community takeover badge logic
    pub fn assign_community_takeover_badge(ctx: Context<AssignCommunityTakeoverBadge>) -> Result<()> {
        if ctx.accounts.creator_token_account.amount == 0 {
            ctx.accounts.metadata.community_takeover_badge = true;
        }
        Ok(())
    }

    // Token ownership renouncement
    pub fn renounce_ownership(ctx: Context<RenounceOwnership>) -> Result<()> {
        let null_address = Pubkey::default();
        ctx.accounts.token_program = null_address; // Transfer ownership to null address
        Ok(())
    }
}

// Accounts for different functions

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, // User minting the token
    #[account(mut)]
    pub creator_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>, // Treasury wallet for minting fee
    #[account(mut)]
    pub token_supply: Account<'info, TokenAccount>, // Total supply account
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct MigrateToRaydium<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, 
    #[account(mut)]
    pub creator_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_supply: Account<'info, TokenAccount>,
    #[account(mut)]
    pub raydium_wallet: Account<'info, TokenAccount>, // Raydium wallet
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SwapTokens<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub swap_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct AssignSafeBadge<'info> {
    #[account(mut)]
    pub creator_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_supply: Account<'info, TokenAccount>,
    #[account(mut)]
    pub metadata: Account<'info, TokenMetadata>,
}

#[derive(Accounts)]
pub struct AssignCommunityTakeoverBadge<'info> {
    #[account(mut)]
    pub creator_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub metadata: Account<'info, TokenMetadata>,
}

#[derive(Accounts)]
pub struct RenounceOwnership<'info> {
    pub payer: Signer<'info>,
    pub token_program: Account<'info, Token>,
    pub mint: Account<'info, Mint>,
    pub token_account: Account<'info, TokenAccount>,
}

#[account]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub uri: String, // Off-chain metadata (e.g., IPFS)
    pub safe_badge: bool,
    pub community_takeover_badge: bool,
}
