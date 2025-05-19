# Staking Program (Solana)
This Solana Program implements a minimalistic staking system for SPL tokens. It allows a pool authority to initialize a staking pool with a fixed reward rate, and lets users create personal staking accounts (via PDAs) and stake or unstake tokens while accruing rewards.

✨ Key Features
**Pool Initialization** – A pool authority sets up a new staking pool with a fixed rewards_per_token rate.

**User Registration** – Users create unique staking accounts (PDA-based) linked to their wallet.

**Token Staking/Unstaking** – Users stake tokens to earn rewards and can later unstake them.

**Reward Claiming** – Users may claim rewards calculated based on their stake and time elapsed.

🔐 Accounts Used
Pool Owner – Initializes and owns the staking pool.

User Wallet – Each user signs and interacts with their own staking account.

Pool Storage Account – Holds global staking data: total staked, reward rate, user count, etc.

User Storage PDA Account – Stores individual staking balances and metadata for each user.

Associated Token Accounts (ATA) – Used for SPL token transfers during staking/unstaking.

Token Program & System Program – Standard Solana programs used for token ops and account creation.

🛠 Instructions Implemented
Initialize { rewards_per_token }
Sets up the pool with the given reward rate.

CreateUser {}
Creates a PDA-based user staking account.

Stake { amount }
Stakes the specified amount of SPL tokens.

Unstake { amount }
Withdraws the specified amount of staked tokens.

Claim {}
Transfers earned rewards to the user.