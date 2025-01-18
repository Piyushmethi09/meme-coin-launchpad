use anchor_lang::prelude::*;

pub fn deduct_trading_fee(ctx: Context<TradingFee>, amount: u64) -> Result<()> {
    let trading_fee = amount * 1 / 100; // 1% trading fee
    let treasury = &ctx.accounts.treasury;
    **treasury.to_account_info().lamports.borrow_mut() += trading_fee;

    Ok(())
}
