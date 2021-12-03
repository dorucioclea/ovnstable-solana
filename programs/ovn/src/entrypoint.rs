use std::borrow::Borrow;
use std::io::Read;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    log,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use mercurial_stable_swap_n_pool_instructions;


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

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Data {
    pub data: String
}

entrypoint!(process_instruction);

pub fn process_instruction(program_id: &Pubkey,
                           accounts: &[AccountInfo],
                           _instruction_data: &[u8]
) -> ProgramResult {
    log::sol_log("HEY");
    // let data = borsh::BorshDeserialize::try_from_slice(&_instruction_data.borrow());
    let mut data = Data::try_from_slice(&_instruction_data.borrow()).unwrap();
    log::sol_log(&data.data);
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


