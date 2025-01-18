use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Token, TokenAccount, Mint, InitializeMint, CreateAccount, Transfer };
use anchor_spl::associated_token::{ AssociatedToken };
use solana_program::pubkey::Pubkey;

declare_id!("4EAQ6275rL3DRA7VAc8mmC6P8xySbokTbSrEo6irBpy2");

#[program]
pub mod token_launchpad {
    use super::*;

    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        // Define the minting fee (0.01 SOL in lamports)
        let fee: u64 = 10_000_000;

        // Ensure the payer has sufficient balance
        if **ctx.accounts.payer.to_account_info().lamports.borrow() < fee {
            return Err(error!(CustomError::InsufficientFunds));
        }

        // Check if the total supply exceeds the max limit
        let total_supply = ctx.accounts.token_supply.amount;
        if total_supply + amount > 1_000_000_000 {
            return Err(error!(CustomError::MaxSupplyExceeded));
        }

        // Transfer the minting fee to the treasury account
        let treasury_account = ctx.accounts.treasury.to_account_info();
        let payer_account = ctx.accounts.payer.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();

        let transfer_instruction = solana_program::system_instruction::transfer(
            payer_account.key,
            treasury_account.key,
            fee
        );
        solana_program::program::invoke(
            &transfer_instruction,
            &[payer_account.clone(), treasury_account.clone(), system_program.clone()]
        )?;

        // Mint the tokens
        token::mint_to(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), token::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.creator_token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            }),
            amount
        )?;

        Ok(())
    }

    // pub fn mint_tokens(
    //     ctx: Context<MintTokens>,
    //     amount: u64,
    //     fee_paid: bool,
    // ) -> Result<()> {
    //     // Ensure the creator has enough SOL balance to mint
    //     let creator_balance = **ctx.accounts.creator.to_account_info().lamports.borrow();
    //     if creator_balance < 0.01 * 1_000_000_000 {
    //         return Err(error!(TokenError::InsufficientBalanceForMinting));
    //     }

    //     // Deduct the minting fee and send it to the treasury
    //     if fee_paid {
    //         let fee = 0.01 * 1_000_000_000;
    //         let treasury = &ctx.accounts.treasury;
    //         **treasury.to_account_info().lamports.borrow_mut() += fee as u64;
    //     } else {
    //         return Err(error!(TokenError::MintingFeeNotPaid));
    //     }

    //     // Mint the tokens
    //     let cpi_accounts = token::MintTo {
    //         mint: ctx.accounts.token_mint.to_account_info(),
    //         to: ctx.accounts.creator_token_account.to_account_info(),
    //         authority: ctx.accounts.token_authority.to_account_info(),
    //     };
    //     let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    //     token::mint_to(cpi_ctx, amount)?;

    //     Ok(())
    // }

    // pub fn renounce_ownership(ctx: Context<RenounceOwnership>) -> Result<()> {
    //     // Renounce ownership logic: Set the owner to null
    //     ctx.accounts.token_mint.authority = Some(Pubkey::default());
    //     Ok(())
    // }

    // pub fn swap_sol_to_token(ctx: Context<Swap>, amount: u64) -> Result<()> {
    //     // Swap SOL to token logic with a fee of 0.0001 SOL
    //     let swap_fee = 0.0001 * 1_000_000_000;
    //     let treasury = &ctx.accounts.treasury;
    //     **treasury.to_account_info().lamports.borrow_mut() += swap_fee as u64;

    //     // Logic for swapping SOL to token
    //     // (Use CPI to transfer SOL to the creator's token account)
    //     Ok(())
    // }
}

// #[derive(Accounts)]
// pub struct MintTokens<'info> {
//     #[account(mut)]
//     pub creator: Signer<'info>,
//     #[account(mut)]
//     pub token_mint: Account<'info, token::Mint>,
//     #[account(mut)]
//     pub creator_token_account: Account<'info, token::TokenAccount>,
//     pub token_authority: Account<'info, token::Mint>,
//     pub token_program: Program<'info, token::Token>,
//     #[account(mut)]
//     pub treasury: AccountInfo<'info>,
// }
#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub creator_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury: AccountInfo<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_supply: Account<'info, TokenAccount>,
    #[account(signer)]
    pub mint_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
// #[derive(Accounts)]
// pub struct RenounceOwnership<'info> {
//     #[account(mut)]
//     pub token_mint: Account<'info, token::Mint>,
// }

// #[derive(Accounts)]
// pub struct Swap<'info> {
//     #[account(mut)]
//     pub creator: Signer<'info>,
//     #[account(mut)]
//     pub treasury: AccountInfo<'info>,
//     pub token_program: Program<'info, token::Token>,
// }
