use anchor_lang::prelude::*;
use instructions::{mint_tokens, trading_fee, dex_allocation, safe_badge, takeover, ownership, swap, admin};
use state::{bonding_curve, treasury, token_metadata};
use utils::*;


declare_id!("4EAQ6275rL3DRA7VAc8mmC6P8xySbokTbSrEo6irBpy2");

#[program]
pub mod wybe_launchpad {
    use super::*;

    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        mint_tokens::mint(ctx, amount)
    }

    pub fn trading_fee(ctx: Context<TradingFee>, amount: u64) -> Result<()> {
        trading_fee::apply_fee(ctx, amount)
    }

    pub fn dex_allocation(ctx: Context<DexAllocation>, amount: u64) -> Result<()> {
        dex_allocation::allocate(ctx, amount)
    }

    pub fn safe_badge(ctx: Context<SafeBadge>, amount: u64) -> Result<()> {
        safe_badge::apply(ctx, amount)
    }

    pub fn takeover(ctx: Context<Takeover>, amount: u64) -> Result<()> {
        takeover::trigger(ctx, amount)
    }

    pub fn ownership(ctx: Context<Ownership>) -> Result<()> {
        ownership::renounce(ctx)
    }

    pub fn swap(ctx: Context<Swap>, amount: u64) -> Result<()> {
        swap::execute(ctx, amount)
    }

    pub fn admin(ctx: Context<Admin>) -> Result<()> {
        admin::manage(ctx)
    }
}



// use anchor_lang::prelude::*;
// use anchor_spl::token::{self, Token, TokenAccount, Mint, InitializeMint, CreateAccount, Transfer};
// use anchor_spl::associated_token::{AssociatedToken};
// use solana_program::pubkey::Pubkey;
// pub mod admin;
// use admin::*;

// declare_id!("4EAQ6275rL3DRA7VAc8mmC6P8xySbokTbSrEo6irBpy2");

// #[program]
// pub mod meme_coin_launchpad {
//     use super::*;
    


//     pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
//         // Define the minting fee (0.01 SOL in lamports)
//         let fee: u64 = 10_000_000;
    
//         // Ensure the payer has sufficient balance
//         if **ctx.accounts.payer.to_account_info().lamports.borrow() < fee {
//             return Err(error!(CustomError::InsufficientFunds));
//         }
    
//         // Check if the total supply exceeds the max limit
//         let total_supply = ctx.accounts.token_supply.amount;
//         if total_supply + amount > 1_000_000_000 {
//             return Err(error!(CustomError::MaxSupplyExceeded));
//         }
    
//         // Transfer the minting fee to the treasury account
//         let treasury_account = ctx.accounts.treasury.to_account_info();
//         let payer_account = ctx.accounts.payer.to_account_info();
//         let system_program = ctx.accounts.system_program.to_account_info();
    
//         let transfer_instruction = solana_program::system_instruction::transfer(
//             payer_account.key,
//             treasury_account.key,
//             fee,
//         );
//         solana_program::program::invoke(
//             &transfer_instruction,
//             &[
//                 payer_account.clone(),
//                 treasury_account.clone(),
//                 system_program.clone(),
//             ],
//         )?;
    
//         // Mint the tokens
//         token::mint_to(
//             CpiContext::new(
//                 ctx.accounts.token_program.to_account_info(),
//                 token::MintTo {
//                     mint: ctx.accounts.mint.to_account_info(),
//                     to: ctx.accounts.creator_token_account.to_account_info(),
//                     authority: ctx.accounts.mint_authority.to_account_info(),
//                 },
//             ),
//             amount,
//         )?;
    
//         Ok(())
//     }
    
//     // Minting coins (0.01 SOL per 1 billion coins)
//     // pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
//     //     let creator_balance = ctx.accounts.creator_token_account.amount;
//     //     let total_supply = ctx.accounts.token_supply.amount;
        
//     //     // Ensure creator has enough SOL balance
//     //     let fee = 0.01 * 10u64.pow(9); // Fee calculation for minting 1 billion tokens
//     //     let creator_sol_balance = ctx.accounts.payer.to_account_info().lamports();

//     //     if creator_sol_balance < fee {
//     //         return Err(ProgramError::InsufficientFunds.into());
//     //     }

//     //     // Transfer the minting fee to the platform treasury
//     //     let treasury = &ctx.accounts.treasury;
//     //     token::transfer(
//     //         ctx.accounts.transfer_to_treasury_context(),
//     //         fee as u64,
//     //     )?;

//     //     // Mint the tokens
//     //     let mint = &ctx.accounts.mint;
//     //     let token_program = &ctx.accounts.token_program;
//     //     let token_account = &ctx.accounts.creator_token_account;

//     //     token::mint_to(
//     //         token_program,
//     //         mint,
//     //         token_account,
//     //         ctx.accounts.payer.to_account_info(),
//     //         &[&ctx.accounts.payer],
//     //         amount,
//     //     )?;

//     //     // Validate maximum supply
//     //     if total_supply + amount > 1_000_000_000 {
//     //         return Err(ProgramError::InvalidArgument.into()); // Maximum token supply exceeded
//     //     }

//     //     Ok(())
//     // }

//     // pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
//     //     // Convert minting fee to lamports
//     //     let fee: u64 = (0.01 * 1_000_000_000.0) as u64;
    
//     //     // Ensure sufficient balance
//     //     if **ctx.accounts.payer.to_account_info().lamports.borrow() < fee {
//     //         return Err(ProgramError::InsufficientFunds.into());
//     //     }
    
//     //     // Transfer SOL fee to treasury
//     //     **ctx.accounts.payer.to_account_info().try_borrow_mut_lamports()? -= fee;
//     //     **ctx.accounts.treasury.to_account_info().try_borrow_mut_lamports()? += fee;
    
//     //     // Check max supply
//     //     let total_supply = ctx.accounts.token_supply.amount;
//     //     if total_supply + amount > 1_000_000_000 {
//     //         return Err(ProgramError::InvalidArgument.into()); // Exceeds max supply
//     //     }
    
//     //     // Mint tokens
//     //     token::mint_to(
//     //         ctx.accounts
//     //             .token_program
//     //             .to_account_info(),
//     //         ctx.accounts.mint.to_account_info(),
//     //         ctx.accounts.creator_token_account.to_account_info(),
//     //         ctx.accounts.payer.to_account_info().key,
//     //         &[&ctx.accounts.payer.to_account_info().key],
//     //         amount,
//     //     )?;
    
//     //     Ok(())
//     // }
    

//     pub fn initialize_admin(
//         ctx: Context<InitializeAdmin>,
//         treasury_wallet: Pubkey,
//     ) -> Result<()> {
//         let admin_config = &mut ctx.accounts.admin_config;
//         admin_config.admin = *ctx.accounts.admin.key;
//         admin_config.treasury_wallet = treasury_wallet;
//         admin_config.minting_fee = 10_000_000; // Default minting fee: 0.01 SOL
//         admin_config.trading_fee = 100;        // Default trading fee: 1%
//         admin_config.paused = false;

//         msg!("Admin and configuration initialized successfully.");
//         Ok(())
//     }
// }

//     // pub fn initialize_admin(ctx: Context<InitializeAdmin>) -> Result<()> {
//     //     msg!("Token Mint Address: {}", ctx.accounts.mint.key());
//     //     msg!("Admin ATA Address: {}", ctx.accounts.admin_ata.key());
//     //     Ok(())
//     // }


//     pub fn update_treasury(ctx: Context<UpdateTreasury>, new_treasury: Pubkey) -> Result<()> {
//         update_treasury(ctx, new_treasury)
//     }

//     pub fn update_minting_fee(ctx: Context<UpdateMintingFee>, new_fee: u64) -> Result<()> {
//         update_minting_fee(ctx, new_fee)
//     }

//     pub fn pause_contract(ctx: Context<PauseContract>, pause: bool) -> Result<()> {
//         pause_contract(ctx, pause)
//     }


// }

//     // Migration function (tokens to Raydium DEX)
//     pub fn migrate_to_raydium(ctx: Context<MigrateToRaydium>, nonce: u8) -> Result<()> {
//         let required_tokens_for_dex = ctx.accounts.token_supply.amount / 100; // 1% token collection
        
//         // Check if the 1% token allocation to treasury is met
//         let treasury_balance = ctx.accounts.treasury.amount;
//         if treasury_balance < required_tokens_for_dex {
//             return Err(ProgramError::InsufficientFunds.into());
//         }

//         // Verify the market cap threshold (e.g., $69,000)
//         let market_cap_threshold = 69000;
//         let current_market_cap = calculate_market_cap(ctx.accounts.token_supply.amount);
//         if current_market_cap < market_cap_threshold {
//             return Err(ProgramError::InvalidArgument.into());
//         }

//         // Migrate tokens to Raydium
//         token::transfer(
//             ctx.accounts.transfer_to_raydium_context(),
//             ctx.accounts.token_supply.amount,
//         )?;

//         Ok(())
//     }

//     // Function to calculate market cap (stub for now)
//     fn calculate_market_cap(total_supply: u64) -> u64 {
//         total_supply * 1 // Assume price is 1 USD for simplicity
//     }

//     // Swap functionality (SOL to token & token to SOL)
//     pub fn swap(ctx: Context<SwapTokens>, amount_in: u64, direction: String) -> Result<()> {
//         let swap_fee = 0.0001; // 0.0001 SOL swap fee
//         if direction == "SOL_to_token" {
//             // Swap SOL to token
//             token::transfer(
//                 ctx.accounts.transfer_to_token_context(),
//                 amount_in - swap_fee as u64,
//             )?;
//         } else if direction == "token_to_SOL" {
//             // Swap token to SOL
//             token::transfer(
//                 ctx.accounts.transfer_to_sol_context(),
//                 amount_in - swap_fee as u64,
//             )?;
//         }
//         Ok(())
//     }

//     // Safe badge logic (5% purchase limit)
//     pub fn assign_safe_badge(ctx: Context<AssignSafeBadge>) -> Result<()> {
//         let creator_balance = ctx.accounts.creator_token_account.amount;
//         let total_supply = ctx.accounts.token_supply.amount;

//         if creator_balance > total_supply / 20 {  // 5% of total supply
//             return Err(ProgramError::InvalidArgument.into());
//         }

//         // Assign "Safe" badge
//         ctx.accounts.metadata.safe_badge = true;
//         Ok(())
//     }

//     // Community takeover badge logic
//     pub fn assign_community_takeover_badge(ctx: Context<AssignCommunityTakeoverBadge>) -> Result<()> {
//         if ctx.accounts.creator_token_account.amount == 0 {
//             ctx.accounts.metadata.community_takeover_badge = true;
//         }
//         Ok(())
//     }

//     // Token ownership renouncement
//     pub fn renounce_ownership(ctx: Context<RenounceOwnership>) -> Result<()> {
//         let null_address = Pubkey::default();
//         ctx.accounts.token_program = null_address; // Transfer ownership to null address
//         Ok(())
//     }
// }

// // Accounts for different functions


// #[derive(Accounts)]
// pub struct MintTokens<'info> {
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     #[account(mut)]
//     pub creator_token_account: Account<'info, TokenAccount>,
//     #[account(mut)]
//     pub treasury: AccountInfo<'info>,
//     #[account(mut)]
//     pub mint: Account<'info, Mint>,
//     #[account(mut)]
//     pub token_supply: Account<'info, TokenAccount>,
//     #[account(signer)]
//     pub mint_authority: AccountInfo<'info>,
//     pub token_program: Program<'info, Token>,
//     pub system_program: Program<'info, System>,
// }


// #[derive(Accounts)]
// pub struct MigrateToRaydium<'info> {
//     #[account(mut)]
//     pub payer: Signer<'info>, 
//     #[account(mut)]
//     pub creator_token_account: Account<'info, TokenAccount>,
//     #[account(mut)]
//     pub treasury: Account<'info, TokenAccount>,
//     #[account(mut)]
//     pub token_supply: Account<'info, TokenAccount>,
//     #[account(mut)]
//     pub raydium_wallet: Account<'info, TokenAccount>, // Raydium wallet
//     pub token_program: Program<'info, Token>,
// }

// #[derive(Accounts)]
// pub struct SwapTokens<'info> {
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     pub swap_account: Account<'info, TokenAccount>,
//     pub token_program: Program<'info, Token>,
// }

// #[derive(Accounts)]
// pub struct AssignSafeBadge<'info> {
//     #[account(mut)]
//     pub creator_token_account: Account<'info, TokenAccount>,
//     #[account(mut)]
//     pub token_supply: Account<'info, TokenAccount>,
//     #[account(mut)]
//     pub metadata: Account<'info, TokenMetadata>,
// }

// #[derive(Accounts)]
// pub struct AssignCommunityTakeoverBadge<'info> {
//     #[account(mut)]
//     pub creator_token_account: Account<'info, TokenAccount>,
//     #[account(mut)]
//     pub metadata: Account<'info, TokenMetadata>,
// }

// #[derive(Accounts)]
// pub struct RenounceOwnership<'info> {
//     pub payer: Signer<'info>,
//     pub token_program: Account<'info, Token>,
//     pub mint: Account<'info, Mint>,
//     pub token_account: Account<'info, TokenAccount>,
// }

// #[account]
// pub struct TokenMetadata {
//     pub name: String,
//     pub symbol: String,
//     pub uri: String, // Off-chain metadata (e.g., IPFS)
//     pub safe_badge: bool,
//     pub community_takeover_badge: bool,
// }

// #[derive(Accounts)]
// pub struct InitializeAdmin<'info> {
//     #[account(init, payer = admin, space = 8 + 32 + 32 + 8 + 8 + 1)]
//     pub admin_config: Account<'info, AdminConfig>,
    
//     #[account(mut)]
//     pub admin: Signer<'info>, // Admin wallet signs the transaction
    
//     #[account(
//         init,
//         payer = admin,
//         mint::decimals = 6,               // Set token decimals to 6
//         mint::authority = admin,          // Admin has mint authority
//         mint::freeze_authority = admin    // (Optional) Admin can freeze accounts
//     )]
//     pub mint: Account<'info, Mint>,       // The token mint account
    
//     #[account(
//         init,
//         payer = admin,
//         associated_token::mint = mint,    // Create an ATA for the admin
//         associated_token::authority = admin
//     )]
//     pub admin_ata: Account<'info, TokenAccount>, // Admin's associated token account
    
//     #[account(address = system_program::ID)]
//     pub system_program: Program<'info, System>,
    
//     #[account(address = token::ID)]
//     pub token_program: Program<'info, Token>,
    
//     #[account(address = anchor_spl::associated_token::ID)]
//     pub associated_token_program: Program<'info, AssociatedToken>,
    
//     pub rent: Sysvar<'info, Rent>, // For determining account rent
// }
// #[account]
// pub struct AdminConfig {
//     pub admin: Pubkey,
//     pub treasury_wallet: Pubkey,
//     pub minting_fee: u64,
//     pub trading_fee: u64,
//     pub paused: bool,
// }