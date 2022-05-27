Create a project called `program`
```bash
cargo new program --lib
```

Paste below code to `Cargo.toml`
```
[features]
# https://docs.solana.com/developing/on-chain-programs/developing-rust#project-layout
#
# When toggled on this feature will cause the crate to not compile a
# bpf entrypoint.
#
# See the corresponding `#[cfg(not(feature = "exclude_entrypoint"))]`
# in lib.rs. This is needed so that other Solana programs can import
# helper functions from this library without causing symbol conflicts
# with our entrypoint.
exclude_entrypoint = []

[dependencies]
solana-program = "1"

[lib]
# This is the name of the compiled file
# For this example, the compiled file path is `target/deploy/hellosolana.so`
name = "hellosolana"
crate-type = ["cdylib", "lib"]
```

In `lib.rs`
```rust
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
```

Compile the program
```bash
cargo build-bpf
```

Run test validator before deploy!
```bash
solana-test-validator
```

Deploy the program
```bash
solana program deploy target/deploy/hellosolana.so
```

If the deploy fails, check if you have enough SOL in your account

Check account balance
```bash
solana balance
```

Airdrop some SOL
```bash
solana airdrop 1
```

If the deploy success, you should see your program ID on the terminal.
You should see different ID as mine.
```text
$ solana program deploy target/deploy/hellosolana.so
Program Id: 4kwL8iV4WuCJv41rxeLqhAVxnuKRrZ9PFUSeBkY78BiW
```