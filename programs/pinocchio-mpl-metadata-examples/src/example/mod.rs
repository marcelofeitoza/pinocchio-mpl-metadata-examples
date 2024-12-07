use pinocchio::program_error::ProgramError;

pub mod create;

#[derive(Clone, Copy, Debug)]
pub enum TestInstruction {
    Create,
}

impl TryFrom<&u8> for TestInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TestInstruction::Create),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
