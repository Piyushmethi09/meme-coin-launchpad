use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

pub fn get_account_balance(account: &AccountInfo) -> u64 {
    **account.lamports.borrow()
}

pub fn transfer_sol(from: &AccountInfo, to: &AccountInfo, amount: u64) -> Result<()> {
    let ix = system_instruction::transfer(from.key, to.key, amount);
    let account_info = vec![from.clone(), to.clone()];
    invoke_signed(&ix, &account_info, &[])?;
    Ok(())
}
