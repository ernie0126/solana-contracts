use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum RoundLoaderError {
    #[error("Relay not in the current round")]
    InvalidRelay,
    #[error("Invalid relay round")]
    InvalidRelayRound,
    #[error("Relay already voted for proposal")]
    RelayAlreadyVoted,
    #[error("Relay round expired")]
    RelayRoundExpired,
}

impl From<RoundLoaderError> for ProgramError {
    fn from(e: RoundLoaderError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
