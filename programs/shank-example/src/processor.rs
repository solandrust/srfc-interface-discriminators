use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::SampleProgramInstruction;

fn process_mint_to(amount: u64) -> ProgramResult {
    Ok(())
}

fn process_transfer(this_should_error: u8) -> ProgramResult {
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

pub fn process(_program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    match SampleProgramInstruction::unpack(data) {
        Ok(ix) => match ix {
            SampleProgramInstruction::MintTo {
                amount,
                custom_arg_1,
                custom_arg_2,
            } => process_mint_to(amount),
            SampleProgramInstruction::Transfer {
                this_should_error,
                custom_arg_1,
                custom_arg_2,
            } => process_transfer(this_should_error),
            SampleProgramInstruction::Burn {
                amount,
                custom_arg_1,
            } => process_burn(amount),
            SampleProgramInstruction::Freeze {
                custom_arg_1,
                custom_arg_2,
            } => process_freeze(),
            SampleProgramInstruction::Thaw { custom_arg_1 } => process_thaw(),
        },
        Err(_) => Err(ProgramError::InvalidInstructionData),
    }
}
