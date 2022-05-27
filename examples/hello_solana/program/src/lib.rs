use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    pubkey::Pubkey,
    msg
};

// Declare the programs entrypoint. The entrypoint is the function
// that will get run when the program is executed.
#[cfg(not(feature = "exclude_entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> entrypoint::ProgramResult {
    msg!("Hello Solana");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
