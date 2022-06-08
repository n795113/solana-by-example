# Instructions

First, let's declare our instructions by adding a new file, `instructions.rs`.

Scaffolding:
```text
client_ts/
├── src/
│  ├── instructions.rs
│  └── main.rs
└── Cargo.toml
```

In `instructions.rs`:
```rust, ignore
#[derive(Debug)]
pub Enum Instruction {
    Increment,
    Decrement,
    SetValue(u32)
}
```

If you are not familar with `Enum` in Rust, please check [this](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) out.

Purpose: when an instruction data (`&[u8]`) be passed into our program, we are gonna to resolve (unpack) it 
into ethier above 3 instructions:
- `Instruction::Increment`,
- `Instruction::Decrement`,
- `Instruction::SetValue(val)`

The program will then match the instruction to do different operations.

### Unpack
Let's implement a method for unpacking (or say deserializing) the given isntruction data.
In this example, we gonna to use the first byte to represent the instruction code:
- 0: Increment
- 1: Decrement
- 2: set value

If the given code is 2 (set value), a user should also pass an u32 number in the other
4 bytes after the first bytes which means the total length of the array should be 5.

Moreover, there is no uniform rule about how to arrange the instruction data. 
You can design your own patterns for specific purpoeses.

```rust, ignore
use solana_program::{program_error::ProgramError};

impl Instruction {
    pub unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&ix_code, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        match ix {
            0 => Ok(Instruction::Increment),
            1 => Ok(Instruction::Decrement),
            2 => {
                // an u32 value should occupy 4 bytes
                if rest.len() != 4 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let val: Result<[u8; 4], _> = rest[..4].try_into();
                match val {
                    Ok(bytes) => Ok(Instruction::Set(u32::from_le_bytes(bytes))),
                    _ => Err(ProgramError::InvalidInstructionData)
                }
            },
            _ => Err(ProgramError::InvalidInstructionData)
        }
    }
}
```

`ix` often represents "instruction", and `tx` represents "transaction".