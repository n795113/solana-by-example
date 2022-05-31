# Program

Include `instractions` crate, and add the instruction branching after the account-owner-check:

```rust, ignore
pub mod instructions;
use crate::instructions::Instruction;

// ...

let mut state = State::try_from_slice(&account.data.borrow())?;
let ix = Instruction::unpack(instruction_data)?;
match ix {
    Instruction::Increament => state.counter += 1,
    Instruction::Decreament => state.counter -= 1,
    Instruction::SetValue(val) => state.counter = val
}
state.serialize(&mut &mut account.data.borrow_mut()[..])?;
```

Don't forget to rename the variable name of instruction data
```rust, ignore
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8], // _instruction_data -> instruction_data
) -> entrypoint::ProgramResult {
    // ...
}
```

[Check out the full code](https://github.com/n795113/solana-by-example/tree/main/examples/set_counter/program)