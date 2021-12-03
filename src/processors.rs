use borsh::to_vec;
use solana_program::{pubkey::Pubkey, account_info::AccountInfo};
use solana_program::log::sol_log;
use spl_token::instruction::{mint_to, initialize_mint};
use spl_token::processor::Processor;
use std::convert::TryFrom;

pub fn mint_contract(sender: &[AccountInfo], amount: &u64) {
    let token_pub = "5dBCLJ2WkWCiivTACeYy2cd2yiCK2pw5nwPpsbAZtQRG";
    let token_account_pub = "3JpgSLAUCsKBv5NDzEbEM6Mq1JP3Tf38hqFibKcoJTBF";
    sol_log(&amount.to_string());

    let token_pub_o = Pubkey::try_from(token_pub).expect("HUI");
    let token_account_pub_o = Pubkey::try_from(token_account_pub).expect("HUI");

    // let a = Processor::process_mint_to(&token_pub_o, sender, *amount, Some(8));

    // let ins = mint_to(&token_pub_o, &Pubkey::new_unique(), &token_account_pub_o, sender.iter().next().unwrap().owner, &[], *amount).unwrap();
    Processor::process_mint_to(&token_pub_o, sender, &ins.data);
}