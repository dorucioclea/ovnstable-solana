use std::borrow::Borrow;
use std::collections::HashMap;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;
// use crate::processors::mint_contract;
use std::convert::TryFrom;
use std::marker::PhantomData;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Method {
    MINT = 0
}

pub trait ConvertProgramData<T> {
    fn convert(&self) -> T;
}


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ProgramData {
    pub method: Method,
    pub args: MintProgramData
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MintProgramData {
    pub amount: u64
}

pub struct OVNToken {
    pub token_pub: Pubkey,
    pub owner_pub: Pubkey,
    pub mint_pub: Pubkey,
    pub token_program_pub: Pubkey,
    sysvar_pub: Pubkey,

    pub decimals: u32,
    pub ovn_program_pub: Pubkey
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AccountTokenData {
    pub token_amount: u64
}

impl Default for OVNToken {
    fn default() -> Self {
        OVNToken {
            token_pub: Pubkey::try_from("CFtGW5J3wV42kDLAAeA25i1PeyX7XTZK6uGga9DmkjH6").unwrap(),
            owner_pub: Pubkey::try_from("2j6W2fSrFaNM7UtaBFsjM6qFg3p1cXstPYDBe2RfyoTm").unwrap(),
            mint_pub: Pubkey::try_from("BKhR2CPsv11T59jKJS9bKBp4er4N9F8hdRGhBHw6Aqey").unwrap(),
            token_program_pub: Pubkey::try_from("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
            sysvar_pub: Pubkey::try_from("SysvarRent111111111111111111111111111111111").unwrap(),

            decimals: 9,
            ovn_program_pub: Pubkey::try_from("Ggz2H6KyKsunBKcJd7Xfc9zLjQJYk4taDRHYjhRj8atZ").unwrap()
        }
    }
}


pub struct OVNProcessor {

}

pub struct Exchange<'a> {
    pub ovn: OVNToken,
    pub program_data: ProgramData,
    pub account_infos: Vec<AccountInfo<'a>>,
}