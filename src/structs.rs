use solana_program::{pubkey::Pubkey};
use borsh::{BorshDeserialize, BorshSerialize};
use std::any::{Any};
use std::borrow::Borrow;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Method {
    MINT = 0
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


