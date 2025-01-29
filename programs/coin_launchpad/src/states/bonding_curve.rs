use anchor_lang::prelude::*;

// pub fn calculate_price(market_cap: u64) -> u64 {
//     // Simple example of a bonding curve: price increases with market cap
//     if market_cap >= 69_000_000 {
//         return 12_000;  // Just an example; actual logic would be more complex
//     }
//     market_cap / 10_000
// }

pub fn calculate_price(current_supply: u64, max_supply: u64) -> u64 {
    let supply_ratio = current_supply as f64 / max_supply as f64;
    let base_price = 1_000_000; // 0.001 SOL
    let dynamic_price = base_price as f64 * (1.0 + supply_ratio);
    dynamic_price as u64
}


#[account]
pub struct BondingCurve {
    pub minting_fee: u64,
    pub trading_fee: u64,
    pub creator: Pubkey,
    pub token_supply: u64,
    // Add more fields for bonding curve specifics
}