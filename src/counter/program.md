# Program

Again, this example is originally from Solana-labs. I just renamed some parts of it so it makes more sence to me. Feel free to check out [the origin repo](https://github.com/solana-labs/example-helloworld).

First, let's declare a struct, which represents the account to hold the counter:
```rust, ignore
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct State {
    counter: u32
}
```

Then we update our `process_instruction` in `main.rs`.
The main difference from the [Hello Solana Program](/hello.md) is we will use `program_id`, and `accounts` this time.
```rust, ignore
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8], // still won't be used in this example
) -> entrypoint::ProgramResult {

    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the counter account
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
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
```

The logic is simple:
1. Get the account
2. Check the account is owned by the program
3. Modify it and write it back