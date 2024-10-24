use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResolverError {
    #[error("ResolverAdminInvalid")]
    ResolverAdminInvalid,

    #[error("SlasherAdminInvalid")]
    SlasherAdminInvalid,
    #[error("SlasherDelegateAdminInvalid")]
    SlasherDelegateAdminInvalid,

    #[error("SlashProposalVetoPeriodEnded")]
    SlashProposalVetoPeriodEnded,
    #[error("SlashProposalVetoPeriodNotEnded")]
    SlashProposalVetoPeriodNotEnded,
    #[error("SlashProposalCompleted")]
    SlashProposalCompleted,
    #[error("SlashProposalInvalid")]
    SlashProposalInvalid,
    #[error("SlashProposalResolverInvalid")]
    SlashProposalResolverInvalid,
    #[error("SlashProposalDeletePeriodNotEnded")]
    SlashProposalDeletePeriodNotEnded,

    #[error("ArithmeticOverflow")]
    ArithmeticOverflow = 3000,
    #[error("ArithmeticUnderflow")]
    ArithmeticUnderflow,
    #[error("DivisionByZero")]
    DivisionByZero,
}

impl<T> DecodeError<T> for ResolverError {
    fn type_of() -> &'static str {
        "resolver"
    }
}

impl From<ResolverError> for ProgramError {
    fn from(e: ResolverError) -> Self {
        Self::Custom(e as u32)
    }
}

impl From<ResolverError> for u64 {
    fn from(e: ResolverError) -> Self {
        e as Self
    }
}
