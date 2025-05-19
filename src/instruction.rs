use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub enum Instruction {
    /// Accounts expected:
    ///
    /// 0. `[signer]` Pool owner wallet account
    /// 1. `[writable]` Pool storage sccount
    Initialize { rewards_per_token: u64 },
    /// Accounts expected:
    ///
    /// 0. `[signer]` User wallet account
    /// 1. `[writable]` User storage PDA account
    /// 2. `[writable]` Pool storage account
    /// 3. `[]` System Program
    CreateUser {},
    /// Accounts expected:
    ///
    /// 0. `[signer]` User wallet account
    /// 1. `[writable]` User storage PDA account
    /// 2. `[writable]` Pool storage account
    /// 3. `[writable]` Token mint
    /// 4. `[writable]` ATA to Debit (User)
    /// 5. `[writable]` ATA to Credit (Pool)
    /// 6. `[]` Token Program
    Stake { amount: u64 },
    /// Accounts expected:
    ///
    /// 0. `[signer]` User wallet account
    /// 1. `[writable]` User storage PDA account
    /// 2. `[writable]` Pool storage account
    /// 3. `[writable]` Token mint
    /// 4. `[writable]` ATA to Debit (Pool)
    /// 5. `[writable]` ATA to Credit (User)
    /// 6. `[]` Token Program
    Unstake { amount: u64 },
    /// Accounts expected:
    ///
    /// 0. `[signer]` User wallet account
    /// 1. `[writable]` User storage PDA account
    /// 2. `[writable]` Pool storage account
    /// 3. `[writable]` Token mint
    /// 4. `[writable]` ATA to Debit (Pool)
    /// 5. `[writable]` ATA to Credit (User)
    /// 6. `[]` Token Program
    Claim {},
}