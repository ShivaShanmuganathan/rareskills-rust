use crate::admin::{has_administrator, read_administrator, write_administrator};
use crate::approval::{read_approval, read_approval_all, write_approval, write_approval_all};
use crate::balance::{increment_supply, read_supply};
use crate::errors::Error;
use crate::event;
use crate::interface::NFTokenTrait;
use crate::owner::{check_owner, read_all_owned, read_owner, write_owner};
use crate::storage_types::{INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD};
use soroban_sdk::{contract, contractimpl, panic_with_error, Address, Env, Vec};

#[contract]
pub struct NFToken;

#[contractimpl]
impl NFTokenTrait for NFToken {
    fn initialize(e: Env, admin: Address) {
        if has_administrator(&e) {
            panic!("already initialized")
        }

        write_administrator(&e, &admin);
    }

    /*
        ADMIN FUNCTIONS
    */

    fn admin(env: Env) -> Address {
        // @audit-issue - Why extend instance TTL here, instead of the temporary storage TTL?
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_administrator(&env)
    }

    fn set_admin(env: Env, new_admin: Address) {
        // @audit-issue - Why extend instance TTL here, instead of the temporary storage TTL?
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let admin = read_administrator(&env);
        // admin == msg.sender
        admin.require_auth();

        write_administrator(&env, &new_admin);
        event::set_admin(&env, admin, new_admin);
    }

    fn mint_new(env: Env, to: Address) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let admin: Address = read_administrator(&env);
        // admin == msg.sender
        admin.require_auth();

        let id = read_supply(&env);
        write_owner(&env, id, Some(to.clone()));
        // @audit-issue - Minting without a supply cap. What if the supply goes beyond i128?
        increment_supply(&env);

        event::mint(&env, to, id)
    }

    // @audit-issue - There is no access control on burn to ensure only the owner can burn the token?
    // @audit-info - Should take the owner arg and check it against the stored owner
    // @audit-issue - Should check owner.require_auth()
    fn burn(env: Env, id: i128) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        let from = read_owner(&env, id);
        // @audit-issue - check_owner is missing here
        write_owner(&env, id, None);
        event::burn(&env, from, id);
    }

    /*
        VIEW FUNCTIONS
    */

    fn get_appr(env: Env, id: i128) -> Address {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_approval(&env, id)
    }

    fn is_appr(env: Env, owner: Address, operator: Address) -> bool {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_approval_all(&env, owner, operator)
    }

    fn owner(env: Env, id: i128) -> Address {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_owner(&env, id)
    }

    fn get_all_owned(env: Env, address: Address) -> Vec<i128> {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_all_owned(&env, address)
    }

    /*
        WRITE FUNCTIONS
    */

    fn appr(env: Env, owner: Address, operator: Address, id: i128) {
        // @audit-info - What exactly does require_auth do here?
        owner.require_auth();
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        check_owner(&env, &owner, id);

        write_approval(&env, id, Some(operator.clone()));
        event::approve(&env, operator, id);
    }

    fn appr_all(env: Env, owner: Address, operator: Address, approved: bool) {
        owner.require_auth();
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        // @audit-issue - Missing check_owner
        // @audit-issue - Is it possible to approve an operator for all tokens, while another operator is approved?
        write_approval_all(&env, owner.clone(), operator.clone(), approved);
        event::approve_all(&env, operator, owner)
    }

    fn transfer(env: Env, from: Address, to: Address, id: i128) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        check_owner(&env, &from, id);
        // @audit-info - What exactly does require_auth do here?
        // I think this might be equivalent to 'msg.sender == from'
        // @audit-issue - Approval not cleared when transferring
        from.require_auth();
        write_owner(&env, id, Some(to.clone()));

        event::transfer(&env, from, to, id);
    }

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, id: i128) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        check_owner(&env, &from, id);
        // 'msg.sender == spender'
        spender.require_auth();
        // @audit-issue - Approval not cleared when transferring
        if read_approval_all(&env, from.clone(), spender.clone())
            || spender == read_approval(&env, id)
        {
            write_owner(&env, id, Some(to.clone()));

            event::transfer(&env, from, to, id);
        } else {
            panic_with_error!(&env, Error::NotAuthorized)
        }
    }
}
