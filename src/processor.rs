use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_pack::IsInitialized,
    pubkey::Pubkey,
};

use crate::instruction::Instruction;
use crate::state::PoolStorageAccount;
use crate::error::StakingError;

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = Instruction::try_from_slice(instruction_data)?;

    match instruction {
        Instruction::Initialize { rewards_per_token } => {
            msg!("Initialize pool");
            process_initialize_pool(program_id, accounts, rewards_per_token)
        }
        _ => Err(StakingError::InvalidInstruction.into()),
    }
}

fn process_initialize_pool(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    rewards_per_token: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let signer = next_account_info(accounts_iter)?;
    if !signer.is_signer {
        return Err(StakingError::InvalidSigner.into());
    }

    let storage = next_account_info(accounts_iter)?;
    if storage.owner != program_id {
        return Err(StakingError::InvalidOwner.into());
    }

    let mut user_storage_data = PoolStorageAccount::try_from_slice(&storage.data.borrow())?;
    if user_storage_data.is_initialized() {
        return Err(StakingError::AlreadyInitialized.into());
    }

    user_storage_data.pool_authority = *signer.key;
    user_storage_data.total_staked = 0u64;
    user_storage_data.user_count = 0u64;
    user_storage_data.rewards_per_token = rewards_per_token;
    user_storage_data.is_initialized = true;

    user_storage_data.serialize(&mut &mut storage.data.borrow_mut()[..])?;

    msg!("Staking pool is initialized {:#?}", user_storage_data);

    Ok(())
}