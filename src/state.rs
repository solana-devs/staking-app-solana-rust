use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};

/// Define the type of state stored in
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct PoolStorageAccount {
pub pool_authority: Pubkey,

pub total_staked: u64,
pub user_count: u64,
pub rewards_per_token: u64,

pub is_initialized: bool,
}

pub const POOL_STORAGE_TOTAL_BYTES: usize = 32 + 8 + 8 + 8 + 1;

impl IsInitialized for PoolStorageAccount {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}