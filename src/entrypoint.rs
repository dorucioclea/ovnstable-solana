use self::super::structs::{ProgramData};
// use self::super::processors::{mint_contract};

use std::borrow::{Borrow, BorrowMut};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    // msg,
    log,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// use mercurial_stable_swap_n_pool_instructions;
use solana_program::log::sol_log;
use crate::structs::OVNProcessor;


// #[derive(BorshSerialize, BorshDeserialize, Debug)]
// pub struct OVNToken {
//     pub token_id: Pubkey,
//     pub total_mint: u128,
//     pub total_burn: u128
// }
//
//
// #[derive(BorshSerialize, BorshDeserialize, Debug)]
// pub struct GreetingAccount {
//     pub counter: u32
// }

// #[derive(BorshSerialize, BorshDeserialize, Debug)]
// pub struct ProgramData {
//     pub data: String
// }

entrypoint!(process_instruction);

pub fn process_instruction(program_id: &Pubkey,
                           accounts: &Vec<AccountInfo>,
                           _instruction_data: &[u8]
) -> ProgramResult {
    log::sol_log("HEY");
    let d: ProgramData = ProgramData::try_from_slice(&_instruction_data.borrow())?;
    log::sol_log("program data parsed");
    let processor = OVNProcessor::new();
    log::sol_log("Creating processor");
    processor.process(d, accounts.to_vec());

    // let account = next_account_info(account_iter)?;
    // match d.method {
    //      Method::MINT => {
    //          sol_log("EXECUTE MINT");
    //
    //          let amount: u64 = program_args as u64;
    //          mint_contract(accounts, &amount);
    //     }
    // }
    // let data = borsh::BorshDeserialize::try_from_slice(&_instruction_data.borrow());
    // let mut data = Data::try_from_slice(&_instruction_data.borrow()).unwrap();
    // mercurial_stable_swap_n_pool_instructions::instruction::exchange()
    // let account_iter = &mut accounts.iter();
    //
    // let account = next_account_info(account_iter)?;
    //
    // if account.owner != program_id {
    //     msg!("ERROR");
    //     return Err(ProgramError::IncorrectProgramId);
    // }
    //
    // let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    // greeting_account.counter += 1;
    // greeting_account.serialize(&mut &mut account.data.borrow_mut()[..]);
    //
    // msg!("ITER {}", greeting_account.counter);

    Ok(())
}


