#![cfg(test)]
use crate::contract::{NFToken, NFTokenClient};
use crate::test_util::setup_test_token;
use soroban_sdk::testutils::Ledger;
use soroban_sdk::testutils::{Address as _, MockAuth};
use soroban_sdk::{Address, Env};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, NFToken);
    let client = NFTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);
    assert_eq!(admin, client.admin());
    // TODO: getters for other fields?
}

#[test]
fn test_mint_new() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let client = setup_test_token(&env, &admin);

    let to = Address::generate(&env);
    client.mint_new(&to);
    assert_eq!(to, client.owner(&0));
}

#[test]
fn test_unauthorized_burn_exploit() {
    let env = Env::default();

    let admin = Address::generate(&env);
    let client = setup_test_token(&env, &admin);
    let user_1 = Address::generate(&env);

    // env.mock_auths(&[MockAuth::Contract(admin)]); // Only this admin is authorized
    env.mock_all_auths();
    client.mint_new(&user_1);
    // assert that user_1 is the owner of the token 0
    assert_eq!(user_1, client.owner(&0));
    // burn the token
    client.burn(&0);
}

#[test]
#[should_panic]
fn test_temporary_admin_exploit() {
    let env = Env::default();
    env.ledger().with_mut(|li| {
        // Current ledger sequence - the TTL is the number of
        // ledgers from the `sequence_number` (exclusive) until
        // the last ledger sequence where entry is still considered
        // alive.
        li.sequence_number = 100_000;
        // Minimum TTL for persistent entries - new persistent (and instance)
        // entries will have this TTL when created.
        li.min_persistent_entry_ttl = 500;
        // Minimum TTL for temporary entries - new temporary
        // entries will have this TTL when created.
        li.min_temp_entry_ttl = 100;
        // Maximum TTL of any entry. Note, that entries can have their TTL
        // extended indefinitely, but each extension can be at most
        // `max_entry_ttl` ledger from the current `sequence_number`.
        li.max_entry_ttl = 15000;
    });
    let admin = Address::generate(&env);
    let user_1 = Address::generate(&env);
    let client = setup_test_token(&env, &admin);

    // Check the admin address in the contract
    assert_eq!(admin, client.admin());

    // Bump the ledger sequence by 7001 ledgers (one ledger past TTL).
    env.ledger().with_mut(|li| {
        li.sequence_number = 100_000 + 101;
    });
    // env.logs().print(); //

    // Check the admin address in the contract again
    let admin_address = client.admin();
}

// Approve is not revoked after transfer
// - This allows users to transfer the token to another user and still have access to it
#[test]
fn test_approve_exploit() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let client = setup_test_token(&env, &admin);

    let user_1 = Address::generate(&env);
    let user_2 = Address::generate(&env);
    let user_3 = Address::generate(&env);

    client.mint_new(&user_1);
    // assert that user_1 is the owner of the token 0
    assert_eq!(user_1, client.owner(&0));

    // approve for user_2
    client.appr(&user_1, &user_2, &0);

    // check approval all status for user_2
    assert_eq!(false, client.is_appr(&user_1, &user_2));
    // Check approval status for token 0
    assert_eq!(user_2, client.get_appr(&0));

    // transfer the token to user_3
    client.transfer(&user_1, &user_3, &0);

    // Check approval status for token 0
    assert_eq!(user_2, client.get_appr(&0));

    // Transfer From
    client.transfer_from(&user_2, &user_3, &user_1, &0);

    // Check owner of token 0
    assert_eq!(user_1, client.owner(&0));
}
