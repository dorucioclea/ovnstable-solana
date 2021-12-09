use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::DerefMut;
use std::rc::Rc;
use std::slice::Iter;
use solana_program::{pubkey::Pubkey, account_info::AccountInfo};
use solana_program::account_info::next_account_info;
use solana_program::log::sol_log;
use spl_token::instruction::{mint_to, initialize_mint, mint_to_checked, initialize_account, transfer};
use spl_token::processor::Processor;
use solana_program::entrypoint::ProgramResult;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::message::Message;
use solana_program::program::invoke;
use solana_program::program_error::{PrintProgramError, ProgramError};
use spl_token::state::Account;
use self::super::structs::{Exchange, Method, OVNProcessor, OVNToken, ProgramData};


impl<'a> OVNProcessor {
    pub fn new() -> OVNProcessor {
        OVNProcessor {}
    }

    pub fn process(&self, program_data: ProgramData, account_infos: Vec<AccountInfo<'a>>) {
        let ovn = OVNToken::default();
        match program_data.method {
            Method::MINT => {
                let exchange = Exchange::new(ovn, program_data, account_infos);
                exchange.mint(3);
                // mint_contract(account_infos, 3);
            }
        }
    }
}


// impl<'a> OVNFrom<&'a Pubkey> for AccountInfo<'a> {
//     fn ovn_from(key: &'a Pubkey, signer: bool, writable: bool, executable: bool) -> Self {
//
//         // AccountInfo {
//         //     key,
//         //     is_signer: signer,
//         //     is_writable: writable,
//         //     lamports: Rc::new(RefCell::new(&mut 0)),
//         //     data: Rc::new(RefCell::new(&mut [])),
//         //     owner: key,
//         //     executable,
//         //     rent_epoch: 0
//         // }
//     }
// }

impl<'a> OVNToken {
    pub fn mint(&self, sender: &Pubkey, account_infos: &Vec<AccountInfo>, amount: u64) -> bool {

        match mint_to(
            &self.token_program_pub,
            &self.mint_pub,
            sender,
            &self.owner_pub,
            &[],
            (amount * self.decimals)) {


            Ok(ins) => {

                match invoke(&ins, account_infos) {
                    Ok(_) => {true}
                    Err(_) => {false}
                }
            }
            Err(err) => {false}
        }

    }
}

impl<'a> Exchange<'a> {
    pub fn new(ovn: OVNToken, program_data: ProgramData, account_infos: Vec<AccountInfo<'a>>) -> Self {
        Exchange {
            ovn,
            program_data,
            account_infos
        }
    }

    pub fn mint(&self, amount: u64) {
        let accounts_infos_iter = &mut self.account_infos.iter();

        let sender = next_account_info(accounts_infos_iter).expect("Cant get sender");
        self.ovn.mint(sender.key, self.account_infos.as_ref(), amount);
    }

}


// pub fn mint_contract(sender: &[AccountInfo], amount: &u64) {
//     let token_pub = "2zbux95BEVcb6oCHerAEfosNeQUjZct95TuQNcySQMvL";
//     let owner_pub = "5aeAsopdEKRXXiKVn52iRRA1x3oXiaU1qyJEMzZ8g9YR";
//     let ming_pub = "9sjFya4L53b62uY35fgE8DzqYeZhXM6JMYhJbHmt9PqJ";
//     let token_program_id = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
//     let cur_program_id = "Ggz2H6KyKsunBKcJd7Xfc9zLjQJYk4taDRHYjhRj8atZ";
//
//     sol_log(&amount.to_string());
//
//
//     for acc in sender {
//         sol_log(&acc.key.to_string());
//         sol_log(&acc.owner.to_string());
//         sol_log(&acc.executable.to_string());
//         sol_log(&acc.is_signer.to_string());
//         sol_log(&acc.is_writable.to_string());
//         sol_log("");
//     }
//
//
//     let owner_pub_o = Pubkey::try_from(owner_pub).expect("HUI");
//     let mint_pub_o = Pubkey::try_from(ming_pub).expect("HUI");
//     let token_program_id_o = Pubkey::try_from(token_program_id).expect("HUI");
//     let cur_program_id_o = Pubkey::try_from(cur_program_id).expect("HUI");
//     let (token_program_derived, b) = Pubkey::find_program_address(&[token_program_id_o.as_ref()], &token_program_id_o);
//
//     let token_pub_o = Pubkey::try_from(token_pub).expect("HUI");
//     sol_log("ACCOUNTS created");
//     // let a = Processor::process_mint_to(&token_account_pub_o, sender, *amount, Some(8)).unwrap();
//     // let ins = initialize_mint(&token_program_id_o, &mint_pub_o, &owner_pub_o, None, 8).expect("HUI2");
//     // Message::new(&[ins], Some(&owner_pub_o));
//     // let ins0 = initialize_account(&token_program_id_o, &owner_pub_o, &mint_pub_o, &owner_pub_o).unwrap();
//     // invoke(&ins0, &sender).unwrap();
//     let mut a = sender[3].clone();
//     a.is_signer = true;
//     a.data_is_empty()
//
//     AccountInfo::new()
//
//
//     let ins = mint_to(&token_program_id_o, &mint_pub_o, &token_pub_o, &owner_pub_o, &[], *amount).unwrap();
//     invoke(&ins, sender).unwrap();
//     sol_log(&sender.len().to_string());

// Processor::process(&ins.program_id, &sender, &ins.data).unwrap();


// let accs = ins.accounts.clone()
//     .iter_mut()
//     .map(|mut x| {
//         AccountInfo::from(x)
//     }).collect();
// const LENN: usize = ins.accounts.len();
// let mut accs: [AccountInfo; LENN] = [];
// for acc in ins.accounts {
// let mut c = ins.accounts.clone();
//
// let accs = c.iter_mut().map(|acc| {
//     AccountInfo::new(
//         &acc.pubkey.clone(),
//         acc.is_signer,
//         true,
//         0 as &mut u64,
//         [0] as &mut [u8],
//         &owner_pub_o.clone(),
//         true,
//         1
//     )
// }).collect();

// }


// sol_log("MINT INSTRUCTION CERATED");
// let out = Processor::process(&ins.program_id, sender, &ins.data).unwrap();
// }