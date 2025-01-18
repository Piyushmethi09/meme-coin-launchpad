use anchor_lang::prelude::*;

pub fn apply_safe_badge(ctx: Context<SafeBadge>, amount: u64) -> Result<()> {
    let max_purchase = amount * 5 / 100; // 5% max purchase allowed
    if amount > max_purchase {
        return Err(error!(TokenError::ExceedsSafeBadgeLimit));
    }

    Ok(())
}
