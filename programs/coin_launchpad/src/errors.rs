use anchor_lang::prelude::*;

#[error_code]
pub enum TokenError {
    #[msg("Insufficient balance for minting")]
    InsufficientBalanceForMinting,
    
    #[msg("Creator has already purchased 5% of the token supply")]
    PurchaseLimitReached,

    #[msg("Ownership renouncement required before trading")]
    OwnershipRenouncementRequired,
    
    #[msg("Invalid creator token account")]
    InvalidCreatorTokenAccount,

    #[msg("Token minting fee not paid")]
    MintingFeeNotPaid,
    
    #[msg("Token allocation for DEX listing not done")]
    DEXListingAllocationMissing,
    
    #[msg("Failed to migrate token to Raydium")]
    RaydiumMigrationFailed,
}
