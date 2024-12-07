#[cfg(test)]
mod tests;

mod example;
use example::{create::create, TestInstruction};

use pinocchio::account_info::AccountInfo;
use pinocchio::entrypoint;
use pinocchio::program_error::ProgramError;
use pinocchio::pubkey::Pubkey;
use pinocchio::ProgramResult;

entrypoint!(process_instruction);

pub const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match TestInstruction::try_from(discriminator)? {
        TestInstruction::Create => create(accounts, data),
    }
}
