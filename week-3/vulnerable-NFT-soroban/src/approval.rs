use crate::errors::Error;
use crate::storage_types::DataKey;
use crate::storage_types::{
    ApprovalAll, ApprovalKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD,
};
use soroban_sdk::{panic_with_error, Address, Env};

pub fn read_approval(env: &Env, id: i128) -> Address {
    let key: DataKey = DataKey::Approval(ApprovalKey::ID(id));
    if let Some(approval) = env.storage().persistent().get::<DataKey, Address>(&key) {
        env.storage().persistent().extend_ttl(
            &key,
            BALANCE_LIFETIME_THRESHOLD,
            BALANCE_BUMP_AMOUNT,
        );
        approval
    } else {
        panic_with_error!(env, Error::NotFound)
    }
}

pub fn read_approval_all(env: &Env, owner: Address, operator: Address) -> bool {
    let key = DataKey::Approval(ApprovalKey::All(ApprovalAll { operator, owner }));
    if let Some(approval) = env.storage().persistent().get::<DataKey, bool>(&key) {
        env.storage().persistent().extend_ttl(
            &key,
            BALANCE_LIFETIME_THRESHOLD,
            BALANCE_BUMP_AMOUNT,
        );
        approval
    } else {
        false
    }
}

// @audit-issue - There is no access control on write_approval
// @audit-info - Option<Address> is used to to approve some address or use None to revoke
pub fn write_approval(env: &Env, id: i128, operator: Option<Address>) {
    let key = DataKey::Approval(ApprovalKey::ID(id));
    env.storage().persistent().set(&key, &operator);
    env.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

// @audit-issue - There is no access control on write_approval_all
// @audit-issue - Is it possible to approve an operator for all tokens, while another operator is approved?
// @audit-issue - This sounds like a design flaw? We are not revoking the previous approval?
pub fn write_approval_all(env: &Env, owner: Address, operator: Address, approved: bool) {
    let key = DataKey::Approval(ApprovalKey::All(ApprovalAll { operator, owner }));
    env.storage().persistent().set(&key, &approved);
    env.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}
