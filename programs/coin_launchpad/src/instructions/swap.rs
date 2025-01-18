use anchor_lang::prelude::*;

pub fn swap_sol_to_token(ctx: Context<Swap>, amount: u64) -> Result<()> {
    let swap_fee = 0.0001 * 1_000_000_000; // Swap fee
    let treasury = &ctx.accounts.treasury;
    **treasury.to_account_info().lamports.borrow_mut() += swap_fee as u64;

    // Add swap logic here for SOL to Token swap
    msg!("Swapping {} SOL to tokens", amount);

    Ok(())
}
