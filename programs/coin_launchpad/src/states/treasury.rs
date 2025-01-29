use anchor_lang::prelude::*;

#[account]
pub struct Treasury {
    pub balance: u64,
    pub creator: Pubkey,
}
