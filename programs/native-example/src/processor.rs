use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::SampleProgramInstruction;

fn process_mint_to(amount: u64) -> ProgramResult {
    Ok(())
}

fn process_transfer(amount: u64) -> ProgramResult {
    Ok(())
}

fn process_burn(amount: u64) -> ProgramResult {
    Ok(())
}

fn process_freeze() -> ProgramResult {
    Ok(())
}

fn process_thaw() -> ProgramResult {
    Ok(())
}

fn process(_program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    match SampleProgramInstruction::unpack(data) {
        Ok(ix) => match ix {
            SampleProgramInstruction::MintTo { amount } => process_mint_to(amount),
            SampleProgramInstruction::Transfer { amount } => process_transfer(amount),
            SampleProgramInstruction::Burn { amount } => process_burn(amount),
            SampleProgramInstruction::Freeze => process_freeze(),
            SampleProgramInstruction::Thaw => process_thaw(),
        },
        Err(_) => Err(ProgramError::InvalidInstructionData),
    }
}
