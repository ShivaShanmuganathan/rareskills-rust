# 🔍 Audit Report: `slashing_handler` Function

**Protocol Component**: StakePool Slashing Handler  
**Function Reviewed**: `slashing_handler`  
**Language / Framework**: Rust, Anchor (Solana)  
**Audit Scope**: Logic correctness, access control, CPI safety, bounds validation, and state integrity  
**Audit Date**: May 2025  
**Auditor**: [Redacted]

---

## ✅ Function Summary

The `slashing_handler` function is intended to:

- Update the `is_locked` flag of a specific reward pool (`reward_pools[router]`) inside a `StakePool` account.
- Perform a token transfer using the `stake_pool` PDA as authority.
- Use `vault` as both source and destination in the SPL Token `transfer` CPI.

---

## Code Snippet

```rust
use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Mint, Token, TokenAccount, Transfer};
use crate::errors::ErrorCode;
use crate::{stake_pool_signer_seeds, state::StakePool };  
  
#[derive(Accounts)] 
pub struct Slashing<'info> {
    // Payer to actually stake the mint tokens
    #[account(mut)]
    pub authority: Signer<'info>,  

    /// Vault of the StakePool token will be transfer to
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub stake_mint: Account<'info, Mint>,

    /// StakePool owning the vault that holds the deposit
    #[account(
        mut,
        has_one = vault @ ErrorCode::InvalidStakePoolVault,
        has_one = stake_mint @ ErrorCode::InvalidAuthority,
    )]
    pub stake_pool: AccountLoader<'info, StakePool>,
    pub token_program: Program<'info, Token>,
}
 
pub fn slashing_handler<'info>(
    ctx: Context<Slashing>,
    amount: u64,
    router: u8,
    is_locked: u8 
) -> Result<()> {
    {    
        let stake_pool = &mut ctx.accounts.stake_pool.load_mut()?;
        let pool = &mut stake_pool.reward_pools[usize::from(router)];
        pool.is_locked = is_locked;

        let cpi_ctx = CpiContext {
            program: ctx.accounts.token_program.to_account_info(),
            accounts: Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.stake_pool.to_account_info(),
            },
            remaining_accounts: Vec::new(),
            signer_seeds: &[stake_pool_signer_seeds!(stake_pool)],
        };
        let _ = token::transfer(cpi_ctx, amount);

        Ok(())
    } 
}
```

Stakepool Struct

```rust
#[assert_size(568)] // what does this mean?
#[account(zero_copy)] // what does this mean?
#[repr(C)] // what does this mean?
pub struct StakePool { 
    /// The original creator of the StakePool. Necessary for signer seeds
    pub creator: Pubkey,
    /** Pubkey that can make updates to StakePool */
    pub authority: Pubkey,
    /** Pubkey that can lock any reward pool */
    pub locker: Pubkey,
    /** Total amount staked that accounts for the lock up period weighting.
    Note, this is not equal to the amount of SPL Tokens staked. */
    pub total_weighted_stake: u128,
    /** Token Account to store the staked SPL Token */
    pub vault: Pubkey,
    /** Mint of the token being staked */
    pub mint: Pubkey,
    /** Mint of the token representing effective stake */
    pub stake_mint: Pubkey,
    /// Array of RewardPools that apply to the stake pool.
    /// Unused entries are Pubkey default. In arbitrary order, and may have gaps.
    pub reward_pools: [RewardPool; MAX_REWARD_POOLS],
    /// The minimum weight received for staking. In terms of 1 / SCALE_FACTOR_BASE.
    /// Examples:
    /// * `min_weight = 1 x SCALE_FACTOR_BASE` = minmum of 1x multiplier for > min_duration staking
    /// * `min_weight = 2 x SCALE_FACTOR_BASE` = minmum of 2x multiplier for > min_duration staking
    pub base_weight: u64,
    /// Maximum weight for staking lockup (i.e. weight multiplier when locked
    /// up for max duration). In terms of 1 / SCALE_FACTOR_BASE. Examples:
    /// * A `max_weight = 1 x SCALE_FACTOR_BASE` = 1x multiplier for max staking duration
    /// * A `max_weight = 2 x SCALE_FACTOR_BASE` = 2x multiplier for max staking duration
    pub max_weight: u64,
    /** Minimum duration for lockup. At this point, the staker would receive the base weight. In seconds. */
    pub min_duration: u64,
    /** Maximum duration for lockup. At this point, the staker would receive the max weight. In seconds. */
    pub max_duration: u64,
    /** Nonce to derive multiple stake pools from same mint */
    pub nonce: u8,
    /** Bump seed for stake_mint */
    pub bump_seed: u8,
    // padding to next 8-byte
    _padding0: [u8; 6],
    _reserved0: [u8; 256]
}
```
---

## ⚠️ Findings Summary

| ID | Severity | Title |
|----|----------|-------|
| F-01 | **High** | Missing Access Control for Slashing Authority |
| F-02 | **Medium** | No-Op Token Transfer (`from == to`) |
| F-03 | **High** | Ignored Result from Critical CPI Call |
| F-04 | **Medium** | Unbounded Index Access in Reward Pool Array |
| F-05 | **High** | Unverified Token Program ID |
| F-06 | **Low** | No Validation on Slashing Amount |
| F-07 | **Low** | No Explicit Owner Constraint on `stake_pool` |

---

## 📂 Findings Detail

### F-01: Missing Access Control for Slashing Authority

**Severity**: High  
**Description**:  
The function does not restrict who can invoke the slashing operation. Any user with access to the necessary accounts can toggle the `is_locked` flag on any reward pool.

**Impact**:  
- Unauthorized users can disable or enable reward distribution.
- May result in economic loss, broken logic, or denial-of-service of staking operations.

**Recommendation**:  
Introduce access control:
```rust
require!(
    stake_pool.authority == ctx.accounts.authority.key()
    || stake_pool.creator == ctx.accounts.authority.key()
    || stake_pool.locker == ctx.accounts.authority.key(),
    ErrorCode::Unauthorized
);
```

---

### F-02: No-Op Token Transfer (`from == to`)

**Severity**: Medium  
**Description**:  
The SPL Token `transfer` call sends tokens from the `vault` to the same `vault`. This is effectively a no-op unless the token is a programmable token that relies on transfer hooks.

**Impact**:  
- Wastes compute units.

**Recommendation**:  
- If slashing is to a burn address or treasury, explicitly include that destination.

---

### F-03: Ignored Result from Critical CPI Call

**Severity**: High  
**Description**:  
The result of the `token::transfer` CPI is ignored (`let _ = ...`). If the transfer fails (e.g., insufficient balance), the function still returns `Ok(())`.

**Impact**:  
- The `is_locked` state may change even if the slashing transfer fails.
- Leads to inconsistent or unexpected state.

**Recommendation**:  
Replace with:
```rust
token::transfer(cpi_ctx, amount)?;
```

---

### F-04: Unbounded Index Access in Reward Pool Array

**Severity**: Medium  
**Description**:  
No bounds check is performed when accessing `stake_pool.reward_pools[router]`.

**Impact**:  
- Could result in panics, invalid memory reads, or corrupted state.

**Recommendation**:  
Add bounds validation:
```rust
require!(
    usize::from(router) < MAX_REWARD_POOLS,
    ErrorCode::InvalidRouter
);
```

---

### F-05: Unverified Token Program ID

**Severity**: High  
**Description**:  
The `token_program` account is user-provided and not validated against the canonical SPL Token program ID.

**Impact**:  
- An attacker could pass a malicious token program that mimics SPL behavior.
- Could lead to fund theft or protocol bypass.

**Recommendation**:  
Add constraint:
```rust
#[account(address = token::ID)]
pub token_program: Program<'info, Token>,
```

---

### F-06: No Validation on Slashing Amount

**Severity**: Low  
**Description**:  
The function accepts an arbitrary `amount` without verifying if it’s > 0 or less than the vault balance.

**Impact**:  
- Allows meaningless or wasteful operations.
- May cause unexpected transfer behavior or unnecessary compute usage.

**Recommendation**:  
Add:
```rust
require!(amount > 0, ErrorCode::InvalidAmount);
```

---

### F-07: No Explicit Owner Constraint on StakePool

**Severity**: Low  
**Description**:  
The `stake_pool` account uses `AccountLoader` but has no constraint to ensure it's owned by the expected program.

**Impact**:  
- In edge cases (especially with account spoofing attempts), this could lead to misinterpretation or unsafe deserialization.

**Recommendation**:  
Add:
```rust
#[account(owner = crate::ID)]
pub stake_pool: AccountLoader<'info, StakePool>,
```

---
