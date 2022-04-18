use borsh::{BorshDeserialize, BorshSerialize};
use bridge_derive::BridgePack;
use bridge_utils::types::{EverAddress, Vote};
use enum_as_inner::EnumAsInner;
use serde::{Deserialize, Serialize};

use solana_program::program_error::ProgramError;
use solana_program::program_pack::{IsInitialized, Pack, Sealed};
use solana_program::pubkey::{Pubkey, PUBKEY_BYTES};

pub const WITHDRAWAL_TOKEN_PERIOD: i64 = 86400;

const WITHDRAWAL_TOKEN_EVENT_LEN: usize = PUBKEY_BYTES + 1 + 1  // ever sender address
    + 8                                                         // amount
    + PUBKEY_BYTES                                              // solana recipient address
;

const WITHDRAWAL_TOKEN_META_LEN: usize = PUBKEY_BYTES   // author
    + 1                                                 // status
    + 8                                                 // bounty
;

const DEPOSIT_TOKEN_EVENT_LEN: usize = 8    // amount
    + PUBKEY_BYTES + 1 + 1                  // ever recipient address
    + PUBKEY_BYTES                          // solana sender address
;

#[derive(Debug, BorshSerialize, BorshDeserialize, BridgePack)]
#[bridge_pack(length = 500)]
pub struct Settings {
    pub is_initialized: bool,
    pub kind: TokenKind,
    pub admin: Pubkey,
    pub emergency: bool,
    pub deposit_limit: u64,
    pub withdrawal_limit: u64,
    pub withdrawal_daily_limit: u64,
    pub withdrawal_daily_amount: u64,
    pub withdrawal_ttl: i64,
}

impl Sealed for Settings {}

impl IsInitialized for Settings {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

#[derive(Debug, BorshSerialize, BorshDeserialize, BridgePack)]
#[bridge_pack(length = 5000)]
pub struct Deposit {
    pub is_initialized: bool,
    pub event: Vec<u8>,
    pub meta: Vec<u8>,
}

impl Sealed for Deposit {}

impl IsInitialized for Deposit {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

#[derive(Debug, BorshSerialize, BorshDeserialize, BridgePack)]
#[bridge_pack(length = 5000)]
pub struct DepositToken {
    pub is_initialized: bool,
    pub event: DepositTokenEventWithLen,
    pub meta: DepositTokenMetaWithLen,
}

impl Sealed for DepositToken {}

impl IsInitialized for DepositToken {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct DepositTokenEvent {
    pub sender_address: Pubkey,
    pub amount: u64,
    pub recipient_address: EverAddress,
    pub configuration: EverAddress,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct DepositTokenEventWithLen {
    pub len: u32,
    pub data: DepositTokenEvent,
}

impl DepositTokenEventWithLen {
    pub fn new(
        sender_address: Pubkey,
        amount: u64,
        recipient_address: EverAddress,
        configuration: EverAddress,
    ) -> Self {
        Self {
            len: DEPOSIT_TOKEN_EVENT_LEN as u32,
            data: DepositTokenEvent {
                sender_address,
                amount,
                recipient_address,
                configuration,
            },
        }
    }
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct DepositTokenMeta {
    pub token_symbol: String,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct DepositTokenMetaWithLen {
    pub len: u32,
    pub data: DepositTokenMeta,
}

impl DepositTokenMetaWithLen {
    pub fn new(token_symbol: String) -> Result<Self, ProgramError> {
        Ok(Self {
            len: token_symbol.try_to_vec()?.len() as u32,
            data: DepositTokenMeta { token_symbol },
        })
    }
}

#[derive(Debug, BorshSerialize, BorshDeserialize, BridgePack)]
#[bridge_pack(length = 5000)]
pub struct WithdrawalToken {
    pub is_initialized: bool,
    pub round_number: u32,
    pub required_votes: u32,
    pub event: WithdrawalTokenEventWithLen,
    pub meta: WithdrawalTokenMetaWithLen,
    pub signers: Vec<Vote>,
}

impl Sealed for WithdrawalToken {}

impl IsInitialized for WithdrawalToken {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct WithdrawalTokenEvent {
    pub sender_address: EverAddress,
    pub amount: u64,
    pub recipient_address: Pubkey,
    pub token_symbol: String,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct WithdrawalTokenEventWithLen {
    pub len: u32,
    pub data: WithdrawalTokenEvent,
}

impl WithdrawalTokenEventWithLen {
    pub fn new(
        sender_address: EverAddress,
        amount: u64,
        recipient_address: Pubkey,
        token_symbol: String,
    ) -> Result<Self, ProgramError> {
        Ok(Self {
            len: (WITHDRAWAL_TOKEN_EVENT_LEN + token_symbol.try_to_vec()?.len()) as u32,
            data: WithdrawalTokenEvent {
                sender_address,
                amount,
                recipient_address,
                token_symbol,
            },
        })
    }
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct WithdrawalTokenMeta {
    pub author: Pubkey,
    pub status: WithdrawalTokenStatus,
    pub bounty: u64,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct WithdrawalTokenMetaWithLen {
    pub len: u32,
    pub data: WithdrawalTokenMeta,
}

impl WithdrawalTokenMetaWithLen {
    pub fn new(author: Pubkey, status: WithdrawalTokenStatus, bounty: u64) -> Self {
        Self {
            len: WITHDRAWAL_TOKEN_META_LEN as u32,
            data: WithdrawalTokenMeta {
                author,
                status,
                bounty,
            },
        }
    }
}

#[derive(
    Debug,
    Copy,
    Clone,
    BorshSerialize,
    BorshDeserialize,
    Serialize,
    Deserialize,
    EnumAsInner,
    PartialEq,
    Eq,
)]
pub enum TokenKind {
    Ever { mint: Pubkey },
    Solana { mint: Pubkey, vault: Pubkey },
}

#[derive(
    Copy, BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, PartialEq,
)]
pub enum WithdrawalTokenStatus {
    New,
    Processed,
    Cancelled,
    Pending,
    WaitingForApprove,
    WaitingForRelease,
}
