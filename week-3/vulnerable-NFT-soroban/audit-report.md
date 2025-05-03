# Soroban NFT Audit Report

## Files

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


## Issues
1. Missing access control for `burn` function
- `burn` function is missing `check_owner` and `require_auth`
- this allows anyone to burn any other user's token

2. Admin is wrongly stored in temporary storage
- Admin should be stored in instance storage

3. Approve is not revoked after transfer
- This allows users to transfer the token to another user and still have access to it

4. Missing TTL extension on temporary storage
- There is no TTL extension on temporary storage on `read_administrator`, it is extending instance TTL.
