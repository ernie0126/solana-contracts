use borsh::{BorshDeserialize, BorshSerialize};
use bridge_utils::types::{EverAddress, Vote};

use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum TokenProxyInstruction {
    /// Vote for withdraw EVER/SOL request
    ///
    /// # Account references
    /// ...
    VoteForWithdrawRequest {
        // Vote type
        vote: Vote,
    },

    /// Withdraw Multi Token EVER
    ///
    /// # Account references
    /// ...
    WithdrawMultiTokenEver,

    /// Withdraw Multi Token SOL
    ///
    /// # Account references
    /// ...
    WithdrawMultiTokenSol,

    /// Initialize Token Proxy
    ///
    /// # Account references
    /// ...
    Initialize {
        // Guardian pubkey
        guardian: Pubkey,
        // Manager pubkey
        manager: Pubkey,
        // Withdrawal manager pubkey
        withdrawal_manager: Pubkey,
    },

    /// Deposit Multi token EVER
    ///
    /// # Account references
    /// ...
    DepositMultiTokenEver {
        // Deposit seed
        deposit_seed: u128,
        // Ever recipient address
        recipient: EverAddress,
        // Deposit amount
        amount: u64,
        // Sol amount to transfer to ever
        sol_amount: u64,
        // Random payload to transfer to ever
        payload: Vec<u8>,
    },

    /// Deposit Multi token SOL
    ///
    /// # Account references
    /// ...
    DepositMultiTokenSol {
        // Deposit seed
        deposit_seed: u128,
        // Mint name
        name: String,
        // Mint symbol
        symbol: String,
        // Ever recipient address
        recipient: EverAddress,
        // Deposit amount
        amount: u64,
        // Sol amount to transfer to ever
        sol_amount: u64,
        // Random payload to transfer to ever
        payload: Vec<u8>,
    },

    /// Withdraw Multi token EVER request
    ///
    /// # Account references
    /// ...
    WithdrawMultiTokenEverRequest {
        // Ever event timestamp
        event_timestamp: u32,
        // Ever event transaction lt
        event_transaction_lt: u64,
        // Ever event configuration
        event_configuration: Pubkey,
        // Ever token root address
        token: EverAddress,
        // token name
        name: String,
        // token symbol
        symbol: String,
        // decimals
        decimals: u8,
        // Solana recipient address
        recipient: Pubkey,
        // Withdrawal amount
        amount: u128,
    },

    /// Withdraw multi token SOL request
    ///
    /// # Account references
    /// ...
    WithdrawMultiTokenSolRequest {
        // Ever event timestamp
        event_timestamp: u32,
        // Ever event transaction lt
        event_transaction_lt: u64,
        // Ever event configuration
        event_configuration: Pubkey,
        // Solana recipient address
        recipient: Pubkey,
        // Withdrawal amount
        amount: u128,
    },

    /// Change Guardian Role
    ///
    /// # Account references
    /// ...
    ChangeGuardian {
        // New guardian pubkey
        new_guardian: Pubkey,
    },

    /// Change Manager Role
    ///
    /// # Account references
    /// ...
    ChangeManager {
        // New guardian pubkey
        new_manager: Pubkey,
    },

    /// Change Withdrawal Manager Role
    ///
    /// # Account references
    /// ...
    ChangeWithdrawalManager {
        // New withdrawal manager pubkey
        new_withdrawal_manager: Pubkey,
    },

    /// Change deposit limit
    ///
    /// # Account references
    /// ...
    ChangeDepositLimit {
        // Deposit limit
        new_deposit_limit: u64,
    },

    /// Change withdrawal limits
    ///
    /// # Account references
    /// ...
    ChangeWithdrawalLimits {
        // Withdrawal limit
        new_withdrawal_limit: Option<u64>,
        // Withdrawal daily limit
        new_withdrawal_daily_limit: Option<u64>,
    },

    /// Enable emergency mode
    ///
    /// # Account references
    /// ...
    EnableEmergencyMode,

    /// Disable emergency mode
    ///
    /// # Account references
    /// ...
    DisableEmergencyMode,

    /// Enable token emergency mode
    ///
    /// # Account references
    /// ...
    EnableTokenEmergencyMode,

    /// Disable token emergency mode
    ///
    /// # Account references
    /// ...
    DisableTokenEmergencyMode,

    /// Approve Withdraw Ever
    ///
    /// # Account references
    /// ...
    ApproveWithdrawEver,

    /// Approve Withdraw SOL
    ///
    /// # Account references
    /// ...
    ApproveWithdrawSol,
}
