use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program_memory::sol_memcmp,
    program_pack::{IsInitialized, Pack},
    pubkey::PUBKEY_BYTES,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::state::Account as SplAccount;

use crate::error::*;

pub fn is_zero_account(account_info: &AccountInfo) -> bool {
    account_info.data.borrow().iter().all(|byte| byte.eq(&0))
}
pub fn assert_initialized<T: Pack + IsInitialized>(account_info: &AccountInfo) -> Result<T> {
    let account: T = T::unpack_unchecked(&account_info.data.borrow())?;
    if !account.is_initialized() {
        return err!(SolmateError::Uninitialized);
    } else {
        Ok(account)
    }
}
pub fn assert_owner(cur_owner: Pubkey, given_owner: Pubkey) -> Result<()> {
    if cur_owner != given_owner {
        return Err(error!(SolmateError::InvalidOwner));
    }
    Ok(())
}

pub fn require(flag: bool) -> Result<()> {
    if !flag {
        return Err(error!(SolmateError::NotAllowed));
    }
    Ok(())
}

pub fn bump(seeds: &[&[u8]], program_id: &Pubkey) -> u8 {
    let (_found_key, bump) = Pubkey::find_program_address(seeds, program_id);
    bump
}

pub fn assert_pda(seeds: &[&[u8]], program_id: &Pubkey, pda: &Pubkey) -> Result<()> {
    let (found_key, _bump) = Pubkey::find_program_address(seeds, program_id);
    if found_key != *pda {
        return Err(error!(SolmateError::InvalidProgramAddress));
    }
    Ok(())
}

pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> Result<()> {
    if account.owner != owner {
        return err!(SolmateError::InvalidOwner);
    } else {
        Ok(())
    }
}

pub fn assert_keys_equal(key1: Pubkey, key2: Pubkey) -> Result<()> {
    if sol_memcmp(key1.as_ref(), key2.as_ref(), PUBKEY_BYTES) != 0 {
        return err!(SolmateError::InvalidPubkey);
    } else {
        Ok(())
    }
}

pub fn assert_is_ata(ata: &AccountInfo, wallet: &Pubkey, mint: &Pubkey) -> Result<SplAccount> {
    assert_owned_by(ata, &spl_token::id())?;
    let ata_account: SplAccount = assert_initialized(ata)?;
    assert_keys_equal(ata_account.owner, *wallet)?;
    assert_keys_equal(ata_account.mint, *mint)?;
    assert_keys_equal(get_associated_token_address(wallet, mint), *ata.key)?;
    Ok(ata_account)
}
