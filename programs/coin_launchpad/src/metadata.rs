use anchor_lang::prelude::*;

pub fn create_token_metadata(
    ctx: Context<CreateMetadata>,
    name: String,
    symbol: String,
    uri: String,
) -> Result<()> {
    // Here we would use an off-chain storage solution like IPFS or Arweave
    msg!("Token Metadata created: {} - {} - {}", name, symbol, uri);
    Ok(())
}

#[derive(Accounts)]
pub struct CreateMetadata<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
}
