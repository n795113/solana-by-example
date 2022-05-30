use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey
};

/// A PDA to store state
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct StateAccount {
    pub counter: u32
}

// Declare the programs entrypoint. The entrypoint is the function
// that will get run when the program is executed.
#[cfg(not(feature = "exclude_entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> entrypoint::ProgramResult {
    // Get counter account
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order for the
    // program to write to it. If that is not the case then the
    // program has been invoked incorrectly and we report as much.
    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize the state infomation from the account
    let mut state = State::try_from_slice(&account.data.borrow())?;

    // Modify it
    state.counter += 1;

    // Then write it back
    state.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}