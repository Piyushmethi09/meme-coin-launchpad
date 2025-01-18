use anchor_lang::prelude::*;

pub fn renounce_ownership(ctx: Context<RenounceOwnership>) -> Result<()> {
    ctx.accounts.token_mint.authority = Some(Pubkey::default()); // Set the owner to null
    Ok(())
}
