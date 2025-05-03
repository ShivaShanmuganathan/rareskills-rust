use crate::errors::Error;
use crate::storage_types::DataKey;
use soroban_sdk::{panic_with_error, Address, Env};

// Shouldn't there be a TTL extension for temporary storage?

// @audit-issue - Why use temporary storage for storing admin address?
pub fn has_administrator(env: &Env) -> bool {
    let key = DataKey::Admin;
    env.storage().temporary().has(&key)
}

pub fn read_administrator(env: &Env) -> Address {
    let key = DataKey::Admin;
    match env.storage().temporary().get::<DataKey, Address>(&key) {
        Some(data) => data,
        None => panic_with_error!(env, Error::NotFound),
    }
}

// @audit-issue - Why No access control on write_administrator?
pub fn write_administrator(env: &Env, id: &Address) {
    let key = DataKey::Admin;
    env.storage().temporary().set(&key, id);
}
