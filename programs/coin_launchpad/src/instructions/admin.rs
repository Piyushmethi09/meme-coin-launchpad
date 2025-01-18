use anchor_lang::prelude::*;

#[account]
pub struct AdminConfig {
    pub admin: Pubkey,          // Address of the current admin
    pub treasury_wallet: Pubkey, // Address of the treasury wallet
    pub minting_fee: u64,        // Fee for minting tokens (in lamports)
    pub trading_fee: u64,        // Trading fee (percentage in basis points, e.g., 100 = 1%)
    pub paused: bool,            // Contract pause state
}

impl AdminConfig {
    pub fn only_admin(&self, signer: &Pubkey) -> Result<()> {
        if &self.admin != signer {
            return Err(error!(AdminError::Unauthorized));
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateTreasury<'info> {
    #[account(mut, has_one = admin)]
    pub admin_config: Account<'info, AdminConfig>,
    #[signer]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateMintingFee<'info> {
    #[account(mut, has_one = admin)]
    pub admin_config: Account<'info, AdminConfig>,
    #[signer]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PauseContract<'info> {
    #[account(mut, has_one = admin)]
    pub admin_config: Account<'info, AdminConfig>,
    #[signer]
    pub admin: AccountInfo<'info>,
}

#[error_code]
pub enum AdminError {
    #[msg("Unauthorized admin access.")]
    Unauthorized,
}

/// Admin Instructions
pub fn update_treasury(ctx: Context<UpdateTreasury>, new_treasury: Pubkey) -> Result<()> {
    let admin_config = &mut ctx.accounts.admin_config;
    admin_config.treasury_wallet = new_treasury;
    Ok(())
}

pub fn update_minting_fee(ctx: Context<UpdateMintingFee>, new_fee: u64) -> Result<()> {
    let admin_config = &mut ctx.accounts.admin_config;
    admin_config.minting_fee = new_fee;
    Ok(())
}

pub fn pause_contract(ctx: Context<PauseContract>, pause: bool) -> Result<()> {
    let admin_config = &mut ctx.accounts.admin_config;
    admin_config.paused = pause;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeAdmin<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 32 + 8 + 8 + 1)]
    pub admin_config: Account<'info, AdminConfig>,
    #[signer]
    pub admin: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

