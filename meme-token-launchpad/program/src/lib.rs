use anchor_lang::prelude::*;

declare_id!("8MzVk3kK5sNRppJMLGyeTdjBcQ6y6Rz8ewGPqahggBky");

#[program]
mod wybe_meme_token_launchpad {
    use super::*;

    pub fn create(ctx: Context<Create>, authority: Pubkey) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.authority = authority;
        counter.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = payer, space = 8 + 40)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, has_one = authority @ MemeTokenLaunchpadError::InvalidAuthority)]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
}

#[error_code]
pub enum MemeTokenLaunchpadError {
    #[msg("The provided authority doesn't match the counter account's authority")]
    InvalidAuthority,
}
