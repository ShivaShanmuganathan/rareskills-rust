# Soroban NFT Smart Contract Audit Report

## Overview
The contract is an NFT implementation on Soroban with standard ERC721-like functionality including minting, burning, transferring, and approval mechanisms. The audit revealed several critical security vulnerabilities that need to be addressed.

## Files Audited
- [x] admin.rs
- [x] approval.rs
- [x] balance.rs
- [x] contract.rs
- [x] errors.rs
- [x] event.rs
- [x] interfaces.rs
- [x] lib.rs
- [x] owner.rs
- [x] storage_types.rs
- [x] test_util.rs
- [x] test.rs

## Critical Vulnerabilities

### 1. Unauthorized Token Burning
**Severity: Critical**

**Description:**
The `burn` function in `contract.rs` lacks proper access control, allowing any user to burn tokens they don't own.

**Location:**
`contract.rs` - `burn` function

**Proof of Concept:**
Test case `test_unauthorized_burn_exploit` demonstrates this vulnerability by allowing a non-owner to burn a token.

**Fix:**
```rust
fn burn(env: Env, id: i128) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

    let from = read_owner(&env, id);
    check_owner(&env, &from, id);
    from.require_auth();
    
    write_owner(&env, id, None);
    event::burn(&env, from, id);
}
```

### 2. Temporary Storage for Admin
**Severity: Critical**

**Description:**
The admin address is stored in temporary storage instead of instance storage, making it vulnerable to TTL expiration attacks.

**Location:**
`admin.rs` - Storage implementation

**Proof of Concept:**
Test case `test_temporary_admin_exploit` demonstrates how the admin can be lost after TTL expiration.

**Fix:**
- Move admin storage from temporary to instance storage
- Update all admin-related functions to use instance storage

### 3. Approval Persistence After Transfer
**Severity: Critical**

**Description:**
Approvals are not revoked after token transfers, allowing previous approved addresses to still transfer tokens they no longer have permission for.

**Location:**
`contract.rs` - `transfer` and `transfer_from` functions

**Proof of Concept:**
Test case `test_approve_exploit` demonstrates:
1. User1 approves User2 for a token
2. User1 transfers token to User3
3. User2 can still transfer the token using `transfer_from`

**Fix:**
```rust
fn transfer(env: Env, from: Address, to: Address, id: i128) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    check_owner(&env, &from, id);
    from.require_auth();
    
    // Clear approval before transfer
    write_approval(&env, id, None);
    write_owner(&env, id, Some(to.clone()));

    event::transfer(&env, from, to, id);
}

fn transfer_from(env: Env, spender: Address, from: Address, to: Address, id: i128) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    check_owner(&env, &from, id);
    spender.require_auth();
    
    if read_approval_all(&env, from.clone(), spender.clone())
        || spender == read_approval(&env, id)
    {
        // Clear approval before transfer
        write_approval(&env, id, None);
        write_owner(&env, id, Some(to.clone()));

        event::transfer(&env, from, to, id);
    } else {
        panic_with_error!(&env, Error::NotAuthorized)
    }
}
```

### 4. TTL Extension Issues
**Severity: Critical**

**Description:**
The contract extends instance TTL instead of temporary storage TTL in several functions, which could lead to storage expiration issues.

**Location:**
Multiple functions in `contract.rs`

**Fix:**
```rust
fn admin(env: Env) -> Address {
    env.storage()
        .temporary()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    read_administrator(&env)
}
```

## Additional Recommendations

### 1. Supply Cap Implementation
- Add a maximum supply cap to prevent potential integer overflow issues
- Implement checks in the `mint_new` function

### 2. Approval All Function
- Add proper owner checks in `appr_all` function
- Consider implementing a more granular approval system

### 3. Event Emission
- Ensure all state changes are properly logged through events
- Add more detailed event data for better tracking

### 4. Error Handling
- Implement more specific error types
- Add better error messages for debugging

## Conclusion
The contract has several critical security vulnerabilities that need to be addressed before deployment. The most pressing issues are the unauthorized burning capability and the approval persistence after transfer. The fixes provided should be implemented to ensure the security and proper functioning of the NFT contract.
