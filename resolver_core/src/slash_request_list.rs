use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{AccountDeserialize, Discriminator};
use shank::ShankAccount;
use solana_program::{account_info::AccountInfo, msg, program_error::ProgramError, pubkey::Pubkey};

use crate::slash_request::SlashRequest;

/// The vault configuration account for the vault program.
/// Manages program-wide settings and state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable, AccountDeserialize, ShankAccount)]
#[repr(C)]
pub struct SlashRequestList {
    /// The NCN account
    pub ncn: Pubkey,

    pub list: [SlashRequest; 32],

    /// The bump seed for the PDA
    pub bump: u8,
    // Reserved space
    // reserved: [u8; 263],
}

impl Discriminator for SlashRequestList {
    const DISCRIMINATOR: u8 = 3;
}

impl SlashRequestList {
    pub const MAX_SLASH_REQUEST: usize = 32;

    pub fn new(ncn: Pubkey, bump: u8) -> Self {
        Self {
            ncn,
            list: [SlashRequest::default(); Self::MAX_SLASH_REQUEST],
            bump,
            // reserved: [0; 263],
        }
    }

    pub fn seeds(ncn: &Pubkey) -> Vec<Vec<u8>> {
        Vec::from_iter([b"slash_request_list".to_vec(), ncn.as_ref().to_vec()])
    }

    pub fn find_program_address(program_id: &Pubkey, ncn: &Pubkey) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds(ncn);
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let (pda, bump) = Pubkey::find_program_address(&seeds_iter, program_id);
        (pda, bump, seeds)
    }

    /// Loads the account as an [`Resolver`] account, returning an error if it is not.
    ///
    /// # Arguments
    /// * `program_id` - The program ID
    /// * `slash_request_list` - The account to load the SlashRequestList from
    /// * `ncn` - The account to load the SlashRequestList from
    /// * `expect_writable` - Whether the account should be writable
    ///
    /// # Returns
    /// * `Result<(), ProgramError>` - The result of the operation
    pub fn load(
        program_id: &Pubkey,
        slash_request_list: &AccountInfo,
        ncn: &AccountInfo,
        expect_writable: bool,
    ) -> Result<(), ProgramError> {
        if slash_request_list.owner.ne(program_id) {
            msg!("Slash request list account has an invalid owner");
            return Err(ProgramError::InvalidAccountOwner);
        }
        if slash_request_list.data_is_empty() {
            msg!("Slash request list account data is empty");
            return Err(ProgramError::InvalidAccountData);
        }
        if expect_writable && !slash_request_list.is_writable {
            msg!("Slash request list account is not writable");
            return Err(ProgramError::InvalidAccountData);
        }
        if slash_request_list.data.borrow()[0].ne(&Self::DISCRIMINATOR) {
            msg!("Slash request list account discriminator is invalid");
            return Err(ProgramError::InvalidAccountData);
        }

        let expected_pubkey = Self::find_program_address(program_id, &ncn.key).0;
        if slash_request_list.key.ne(&expected_pubkey) {
            msg!("Slash Request list account is not at the correct PDA");
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}
