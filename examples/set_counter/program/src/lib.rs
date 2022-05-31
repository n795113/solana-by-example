use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub mod instructions;
use crate::instructions::Instruction;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct State {
    pub counter: u32
}

#[cfg(not(feature = "exclude_entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> entrypoint::ProgramResult {

    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut state = State::try_from_slice(&account.data.borrow())?;
    let ix = Instruction::unpack(instruction_data)?;
    match ix {
        Instruction::Increament => state.counter += 1,
        Instruction::Decreament => state.counter -= 1,
        Instruction::Set(val) => state.counter = val
    }
    state.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}
