use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    declare_id
};


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct OVNToken {
    pub token_id: Pubkey,
    pub total_mint: u128,
    pub total_burn: u128
}


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    pub counter: u32
}

entrypoint!(process_instruction);

pub fn process_instruction(program_id: &Pubkey,
                           accounts: &[AccountInfo],
                           _instruction_data: &[u8]
) -> ProgramResult {
    msg!("HEY");
    declare_id!("");


    let account_iter = &mut accounts.iter();

    let account = next_account_info(account_iter)?;

    if account.owner != program_id {
        msg!("ERROR");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    greeting_account.counter += 1;
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..]);

    msg!("ITER {}", greeting_account.counter);

    Ok(())
}


