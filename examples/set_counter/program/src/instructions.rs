use solana_program::{program_error::ProgramError};

#[derive(Debug)]
pub enum Instruction {
    Increment,
    Decrement,
    Set(u32)
}

impl Instruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&ix_code, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        match ix_code {
            0 => Ok(Instruction::Increment),
            1 => Ok(Instruction::Decrement),
            2 => {
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