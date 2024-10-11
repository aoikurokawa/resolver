mod execute_instant_slash;
mod execute_slash;
mod initialize_config;
mod initialize_resolver;
mod request_slash;
mod veto_slash;

use borsh::BorshDeserialize;
use resolver_sdk::instruction::VaultInstruction;
use solana_program::{
    account_info::AccountInfo, declare_id, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

declare_id!("AE7fSUJSGxMzjNxSPpNTemrz9cr26RFue4GwoJ1cuR6f");

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if program_id.ne(&id()) {
        return Err(ProgramError::IncorrectProgramId);
    }

    let instruction = VaultInstruction::try_from_slice(instruction_data)?;

    match instruction {
        VaultInstruction::CreateTokenMetadata { name, symbol, uri } => {
            msg!("Instruction: CreateTokenMetadata");
            process_create_token_metadata(program_id, accounts, name, symbol, uri)?;
        }
    }

    Ok(())
}
