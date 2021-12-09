use std::collections::HashMap;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;
// use crate::processors::mint_contract;
use std::convert::TryFrom;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Method {
    MINT = 0
}

pub trait OVNFrom<T> {
    fn ovn_from(key: T, signer: bool, writable: bool, executable: bool) -> Self;
}

// #[derive(BorshSerialize, BorshDeserialize, Debug)]
// pub struct ProgramArgument {
//     pub value: u64
// }

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ProgramData {
    pub method: Method,
    pub args: u128
}

pub struct OVNToken {
    pub token_pub: Pubkey,
    pub owner_pub: Pubkey,
    pub mint_pub: Pubkey,
    pub token_program_pub: Pubkey,

    pub decimals: u64
}

impl Default for OVNToken {
    fn default() -> Self {
        let mut keys: HashMap<&str, Pubkey> = HashMap::new();
        keys.insert("token_pub", Pubkey::try_from("2zbux95BEVcb6oCHerAEfosNeQUjZct95TuQNcySQMvL").unwrap());
        keys.insert("owner_pub", Pubkey::try_from("5aeAsopdEKRXXiKVn52iRRA1x3oXiaU1qyJEMzZ8g9YR").unwrap());
        keys.insert("mint_pub", Pubkey::try_from("9sjFya4L53b62uY35fgE8DzqYeZhXM6JMYhJbHmt9PqJ").unwrap());
        keys.insert("token_program_pub", Pubkey::try_from("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap());

        OVNToken {
            token_pub: *keys.get("token_pub").unwrap(),
            owner_pub: *keys.get("owner_pub").unwrap(),
            mint_pub: *keys.get("mint_pub").unwrap(),
            token_program_pub: *keys.get("token_program_pub").unwrap(),

            decimals: 8
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