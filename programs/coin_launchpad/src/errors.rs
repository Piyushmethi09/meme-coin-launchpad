use anchor_lang::prelude::*;

/// Error codes for the Meme Token Launchpad program
#[error_code]
pub enum TokenError {
    #[msg("Insufficient balance for minting.")]
    InsufficientBalanceForMinting,
    
    #[msg("Creator has already purchased 5% of the token supply.")]
    PurchaseLimitReached,
    
    #[msg("Ownership renouncement required before trading.")]
    OwnershipRenouncementRequired,
    
    #[msg("Invalid creator token account.")]
    InvalidCreatorTokenAccount,
    
    #[msg("Token minting fee has not been paid.")]
    MintingFeeNotPaid,
    
    #[msg("Token allocation for DEX listing has not been completed.")]
    DEXListingAllocationMissing,
    
    #[msg("Failed to migrate the token to Raydium.")]
    RaydiumMigrationFailed,
    
    #[msg("Attempting to exceed token supply limit.")]
    TokenSupplyLimitExceeded,
    
    #[msg("Trading fee not deducted correctly.")]
    TradingFeeDeductionFailed,
    
    #[msg("Swap fee not transferred to the treasury.")]
    SwapFeeNotTransferred,
    
    #[msg("Token swap failed due to insufficient balance.")]
    InsufficientBalanceForSwap,
    
    #[msg("Invalid parameters for bonding curve calculation.")]
    InvalidBondingCurveParameters,
    
    #[msg("The bonding curve limit has been reached.")]
    BondingCurveLimitReached,
    
    #[msg("Token metadata storage failed.")]
    MetadataStorageFailed,
    
    #[msg("Token ownership renouncement has already been completed.")]
    OwnershipAlreadyRenounced,
    
    #[msg("Only the creator can perform this operation.")]
    UnauthorizedCreatorAction,
    
    #[msg("An unknown error occurred.")]
    UnknownError,

    #[msg("Insufficient funds for minting fee.")]
    InsufficientFunds,
    
    #[msg("Minting exceeds the maximum token supply.")]
    MaxSupplyExceeded,
}
