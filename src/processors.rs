use std::borrow::{Borrow, BorrowMut};
use std::cell::{BorrowError, Ref, RefCell};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::slice::Iter;
use solana_program::{pubkey::Pubkey, account_info::AccountInfo};
use solana_program::account_info::next_account_info;
use solana_program::log::sol_log;
use spl_token::instruction::{mint_to, initialize_mint, mint_to_checked, initialize_account, transfer, initialize_account2, approve, sync_native};
use spl_token::processor::Processor;
use solana_program::entrypoint::ProgramResult;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::message::Message;
use solana_program::program::{get_return_data, invoke, invoke_unchecked};
use solana_program::program_error::{PrintProgramError, ProgramError};
use spl_associated_token_account::{create_associated_token_account, get_associated_token_address};
use crate::structs::{AccountTokenData, MintProgramData};
use self::super::structs::{Exchange, Method, OVNProcessor, OVNToken, ProgramData};
use std::convert::TryFrom;
use std::mem;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::borsh::try_from_slice_unchecked;
use solana_program::system_instruction::{allocate, assign};


impl<'a> OVNProcessor {
    pub fn new() -> OVNProcessor {
        OVNProcessor {}
    }

    pub fn process(&self, program_data: ProgramData, account_infos: Vec<AccountInfo<'a>>) {
        let ovn = OVNToken::default();
        match program_data.method {
            Method::MINT => {
                sol_log("MINT");
                let mint_program_data: u64 = program_data.args.borrow().amount;
                let exchange = Exchange::new(ovn, program_data, account_infos);
                exchange.mint(mint_program_data);
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
    pub fn mint(&self, receiver: &Pubkey, account_infos: &mut Vec<AccountInfo>, amount: u64) -> bool {
        sol_log(amount.to_string().as_ref());
        sol_log(self.convert_decimals(amount).to_string().as_ref());

        // let acc_created = match self.create_token_account(receiver, account_infos) {
        //     Ok(_) => {true}
        //     Err(_) => {false}
        // };
        // let mut lm = account_infos[0].lamports.try_borrow().ok().unwrap();
        // sol_log("lamports");
        // sol_log(lm.to_string().as_str());

        match mint_to(
            &self.token_program_pub,
            &self.mint_pub,
            &self.token_pub,
            &self.owner_pub,
            &[],
            self.convert_decimals(amount)) {


            Ok(ins) => {
                let acc_iter = &mut account_infos.iter();

                let receiver_acc = next_account_info(acc_iter).unwrap();
                let token_acc = next_account_info(acc_iter).unwrap();
                let mint_acc = next_account_info(acc_iter).unwrap();
                let owner_acc = next_account_info(acc_iter).unwrap();
                let spl_acc = next_account_info(acc_iter).unwrap();

                let spl_mint_ai = vec![mint_acc.clone(), token_acc.clone(), owner_acc.clone(), spl_acc.clone(), receiver_acc.clone()];

                match invoke(&ins, spl_mint_ai.as_slice()) {
                    Ok(_) => {
                        self.transfer_to(receiver, account_infos, amount)
                    }
                    Err(_) => {false}
                }
            }
            Err(err) => {false}
        }
    }

    fn create_token_account(&self, receiver_pub: &Pubkey, account_infos: &Vec<AccountInfo>) -> ProgramResult {
        let acc_iter = &mut account_infos.iter();

        let receiver_acc = next_account_info(acc_iter).unwrap();
        let token_acc = next_account_info(acc_iter).unwrap();
        let mint_acc = next_account_info(acc_iter).unwrap();
        let owner_acc = next_account_info(acc_iter).unwrap();
        let spl_acc = next_account_info(acc_iter).unwrap();
        let sysvar_acc = next_account_info(acc_iter).unwrap();
        let sysprog_acc = next_account_info(acc_iter).unwrap();

        sol_log(receiver_acc.key.to_string().as_ref());

        // let mut receiver_acc_signer = receiver_acc.clone();
        // receiver_acc_signer.is_signer = true;
        for x in account_infos {
            sol_log(x.is_signer.to_string().as_ref());
        }


        let ins = create_associated_token_account(owner_acc.key, receiver_acc.key, &self.mint_pub);
        //
        // //
        // //
        // //
        let ac_to_send = vec![owner_acc.clone(), token_acc.clone(), receiver_acc.clone(), mint_acc.clone(), sysprog_acc.clone(), spl_acc.clone(), sysvar_acc.clone()];
        sol_log("start invoke create associated acc");
        match invoke(&ins, ac_to_send.as_slice()) {
            Ok(_) => {sol_log("EEEEEE"); Ok(())}
            Err(_) => {Err(ProgramError::Custom(101))}
        }

    }

    fn transfer_to(&self, to: &Pubkey, account_infos: &Vec<AccountInfo>, amount: u64) -> bool {
        let acc_iter = &mut account_infos.iter();

        let receiver_acc = next_account_info(acc_iter).unwrap();
        let token_acc = next_account_info(acc_iter).unwrap();
        let mint_acc = next_account_info(acc_iter).unwrap();
        let owner_acc = next_account_info(acc_iter).unwrap();
        let spl_acc = next_account_info(acc_iter).unwrap();
        let sysvar_acc = next_account_info(acc_iter).unwrap();
        let sysprog_acc = next_account_info(acc_iter).unwrap();
        let mut associated_acc = next_account_info(acc_iter).unwrap();
        let associated_program_acc = next_account_info(acc_iter).unwrap();

        match transfer(
            &self.token_program_pub,
            &self.token_pub,
            associated_acc.key,
            &self.owner_pub,
            &[],
            self.convert_decimals(amount)
        ) {
            Ok(ins) => {


                let acc_infos_to_send = vec![token_acc.clone(), associated_acc.clone(), owner_acc.clone()];

                match invoke_unchecked(&ins, acc_infos_to_send.as_slice()) {
                    Ok(_) => {

                        sol_log("transfered");
                        true
                    }
                    Err(_) => {sol_log("NOT transfered"); false}
                }
            }
            Err(_) => {false}
        }
    }

    fn allocate_space(&self, acc: &AccountInfo, size: u64) -> ProgramResult {
        let ins = allocate(acc.key, size);
        let ai = vec![acc.clone()];
        invoke_unchecked(&ins, ai.as_slice())
    }

    fn convert_decimals(&self, amount: u64) -> u64 {
        amount * (u64::pow(10, self.decimals))
    }


    pub fn balance(&self, acc: &AccountInfo, account_infos: &Vec<AccountInfo>) {

        sol_log(acc.owner.to_string().as_str());
        match sync_native(&self.token_program_pub, acc.key) {
            Ok(ins_sync) => {

                let v = vec![acc.clone()];
                match invoke(&ins_sync, v.as_slice()) {
                    Ok(_) => {
                        sol_log("sync native");
                        let a = acc.lamports.try_borrow().ok().unwrap();
                    }
                    Err(_) => {
                        sol_log("err sync native")
                    }
                }
            }
            Err(_) => {}
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

        // sol_log("balance");
        // self.balance();

        let receiver = next_account_info(accounts_infos_iter).expect("Cant get sender");
        self.ovn.mint(receiver.key, &mut self.account_infos.clone(), amount);
    }

    pub fn balance(&self) {
        let acc_iter = &mut self.account_infos.iter();

        let receiver_acc = next_account_info(acc_iter).unwrap();
        let token_acc = next_account_info(acc_iter).unwrap();
        let mint_acc = next_account_info(acc_iter).unwrap();
        let owner_acc = next_account_info(acc_iter).unwrap();
        let spl_acc = next_account_info(acc_iter).unwrap();
        let sysvar_acc = next_account_info(acc_iter).unwrap();
        let sysprog_acc = next_account_info(acc_iter).unwrap();
        let mut associated_acc = next_account_info(acc_iter).unwrap();
        let associated_program_acc = next_account_info(acc_iter).unwrap();

        self.ovn.balance(associated_acc, &self.account_infos.clone());

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