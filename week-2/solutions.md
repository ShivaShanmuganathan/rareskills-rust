# Week 2 - Solutions
## Exercise 1 - Calculator program

Build your own "Calculator" Rust program with the following restrictions:

- Create a "Calculator" structure with 2 integer members
- The "Calculator" structure should define at least three traits:
    - `AdditiveOperations`
    - `MultiplicativeOperations`
    - `BinaryOperations`
- The "Calculator" allow severals operations on scalars:
    - Addition
    - Substraction
    - Multiplication
    - Division
    - AND
    - OR
    - XOR
- The “Calculator” can be printed through the following line of code `println!("calculator: {}", calculator);`
    - When printing the calculator, the result shows the result for each operation.

## Solution 1 - `calculator.rs`


## Exercise 2 - Code analysis - NEAR Smart contract
Goal: Analyze a smart contract written in Rust for the NEAR blockchain. ⚠️ This is not a security analysis.
Note: Some concepts have not been explained yet, give it your best! 
Expected outputs:
A summary explaining the purpose of this contract (should fit in 5-6 lines)
An in-depth analysis of the contract. Comments should be added to the code snippet to explain the concepts shown in Lecture of Week 2.


## Solution 2

#### Summary

- This smart contract allows users to post messages to the blockchain.
- If users attach at least 0.1 NEAR when posting, their message is marked as premium.
- The contract stores all messages and provides functions to retrieve messages (with pagination) and check the total number of messages.
- It uses NEAR SDK storage features and basic payable function handling for premium recognition.
- The contract includes unit tests to verify message posting and pagination behavior.

#### In-depth analysis - Unable to compile this code

```rust
// Import NEAR SDK helpers for (de)serialization
use near_sdk::borsh::{BorshDeserialize, BorshSerialize}; // Borsh is used for fast binary serialization
use near_sdk::json_types::U64; // A wrapped U64 type for JSON (since JSON does not support u64 directly)
use near_sdk::serde::Serialize; // For human-readable JSON serialization
use near_sdk::store::Vector; // Vector is similar to a dynamic array stored on chain
use near_sdk::{env, near_bindgen, AccountId, NearToken}; // Core NEAR primitives

// Constant for 0.1 NEAR, used for premium messages (similar to eth)
const POINT_ONE: NearToken = NearToken::from_millinear(100);

// Data structure for a single posted message
#[derive(BorshDeserialize, BorshSerialize, Serialize)] // Deriving the serialization traits for the struct
#[serde(crate = "near_sdk::serde")] // Tell serde to use NEAR's serde
#[borsh(crate = "near_sdk::borsh")] // Tell borsh to use NEAR's borsh
pub struct PostedMessage {
    pub premium: bool, // Is the message premium (based on attached deposit)
    pub sender: AccountId, // Who sent the message
    pub text: String, // Content of the message
}

// The main contract structure
#[near_bindgen] // Prepares Rust code to be compiled into a NEAR smart contract. (This macro is required to expose the data and functions to the blockchain. I can think of it as a public keyword in other languages as well.)
#[derive(BorshDeserialize, BorshSerialize)] // Deriving the serialization traits for the struct
#[borsh(crate = "near_sdk::borsh")] // Tell borsh to use NEAR's borsh
pub struct Contract {
    messages: Vector<PostedMessage>, // Store posted messages
}

// Implement default storage initialization for Contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            messages: Vector::new(b"m"), // Use a unique prefix (b"m") for storage key
        }
    }
}

// Implement contract methods
#[near_bindgen] // this is replaced with #[near] in the newer contracts
impl Contract {
    // Add a new message (payable allows users to attach NEAR tokens)
    #[payable]
    pub fn add_message(&mut self, text: String) // `self` here means the contract's storage
    {
        let premium = env::attached_deposit() >= POINT_ONE; // Check if user attached at least 0.1 NEAR
        let sender = env::predecessor_account_id(); // get the account id of the caller
        // create a new message struct with the premium, sender, and text
        let message = PostedMessage {
            premium,
            sender,
            text,
        };
        // push the message to the messages vector
        self.messages.push(message); // Store the message
    }

    // Get multiple messages with optional pagination
    pub fn get_messages(&self, from_index: Option<U64>, limit: Option<U64>) -> Vec<&PostedMessage> {
        let from = u64::from(from_index.unwrap_or(U64(0))); // Default start from index 0
        let limit = u64::from(limit.unwrap_or(U64(10))); // Default fetch up to 10 messages

        self.messages
            .iter() // Create an iterator over the vector
            .skip(from as usize) // Skip to the starting index
            .take(limit as usize) // Take only `limit` messages
            .collect() // Collect into a vector
    }

    // Get the total number of messages stored
    pub fn total_messages(&self) -> u32 {
        self.messages.len()
    }
}

// Unit tests for contract behavior
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_message() {
        let mut contract = Contract::default();
        contract.add_message("A message".to_string());

        let posted_message = &contract.get_messages(None, None)[0];
        assert_eq!(posted_message.premium, false); // Should not be premium without attached deposit
        assert_eq!(posted_message.text, "A message".to_string());
    }

    #[test]
    fn iters_messages() {
        let mut contract = Contract::default();
        contract.add_message("1st message".to_string());
        contract.add_message("2nd message".to_string());
        contract.add_message("3rd message".to_string());

        let total = &contract.total_messages(); // why are we using & here?
        assert!(*total == 3);

        let last_message = &contract.get_messages(Some(U64::from(1)), Some(U64::from(2)))[1];
        assert_eq!(last_message.premium, false); // Should not be premium
        assert_eq!(last_message.text, "3rd message".to_string());
    }
}
```

## Exercise 3 - Code analysis - Rust variants

Several languages such as [Cairo](https://www.cairo-lang.org/) (for Starknet chain) or [Move](https://move-language.github.io/move/) (for [Aptos](https://aptos.dev/en/build/smart-contracts) and [Sui](https://docs.sui.io/concepts/sui-move-concepts) chains) are derivated from Rust. Knowing Rust offers a solid basis to understand smart contracts written with these languages.

Goal: Analyze a smart contract written in Cairo. ⚠️ This is not a security analysis.

Expected outputs:

- A summary explaining the purpose of this contract (should fit in 5-6 lines)
- An in-depth analysis of the contract. Comments should be added to the code snippet to explain the concepts shown in Week 2 Lectures.


## Solution 3

### Summary

- Simple vault for deposit/withdraw with ERC-20 integration.
- This smart contract defines a SimpleVault on Starknet where users can deposit ERC-20 tokens and receive shares representing their stake in the vault.
- Users can later withdraw by redeeming shares for the underlying token amount.
- The vault tracks total deposits and each user’s balance using internal mappings.
- It interacts with an external ERC-20 token via the IERC20 interface, using transfer_from to pull tokens in and transfer to pay tokens back on withdrawal.

### In-depth analysis

```rust
// Import the ContractAddress type
use starknet::ContractAddress;

// Define the ERC-20 interface to interact with external tokens
#[starknet::interface]
pub trait IERC20<TContractState> {
    fn get_name(self: @TContractState) -> felt252;
    fn get_symbol(self: @TContractState) -> felt252;
    fn get_decimals(self: @TContractState) -> u8;
    fn get_total_supply(self: @TContractState) -> felt252;
    fn balance_of(self: @TContractState, account: ContractAddress) -> felt252;
    fn allowance(self: @TContractState, owner: ContractAddress, spender: ContractAddress) -> felt252;
    fn transfer(ref self: TContractState, recipient: ContractAddress, amount: felt252);
    fn transfer_from(ref self: TContractState, sender: ContractAddress, recipient: ContractAddress, amount: felt252);
    fn approve(ref self: TContractState, spender: ContractAddress, amount: felt252);
    fn increase_allowance(ref self: TContractState, spender: ContractAddress, added_value: felt252);
    fn decrease_allowance(ref self: TContractState, spender: ContractAddress, subtracted_value: felt252);
}

// Define a custom vault interface
#[starknet::interface]
pub trait ISimpleVault<TContractState> {
    fn deposit(ref self: TContractState, amount: u256);
    fn withdraw(ref self: TContractState, shares: u256);
    fn user_balance_of(ref self: TContractState, account: ContractAddress) -> u256;
    fn contract_total_supply(ref self: TContractState) -> u256;
}

// The actual contract implementation
#[starknet::contract]
pub mod SimpleVault {
    // Import ERC20 interface and core functions
    use super::{IERC20Dispatcher, IERC20DispatcherTrait};
    use starknet::{ContractAddress, get_caller_address, get_contract_address};

    // Storage layout: defines how data is stored on-chain
    #[storage]
    struct Storage {
        token: IERC20Dispatcher,          // The ERC20 token we are managing
        total_supply: u256,                // Total vault shares in existence
        balance_of: LegacyMap<ContractAddress, u256> // Map user -> vault shares
    }

    // Constructor function: runs once when contract is deployed
    #[constructor]
    fn constructor(ref self: ContractState, token: ContractAddress) {
        // Save the token address inside the vault
        self.token.write(IERC20Dispatcher { contract_address: token });
    }

    // Private internal functions
    #[generate_trait]
    impl PrivateFunctions of PrivateFunctionsTrait { 
        // Mint new shares to a user
        fn _mint(ref self: ContractState, to: ContractAddress, shares: u256) {
            self.total_supply.write(self.total_supply.read() + shares); // Increase total supply
            self.balance_of.write(to, self.balance_of.read(to) + shares); // Increase user balance
        }

        // Burn shares from a user
        fn _burn(ref self: ContractState, from: ContractAddress, shares: u256) {
            self.total_supply.write(self.total_supply.read() - shares); // Decrease total supply
            self.balance_of.write(from, self.balance_of.read(from) - shares); // Decrease user balance
        }
    }

    // The public functions of the vault
    #[abi(embed_v0)]
    impl SimpleVault of super::ISimpleVault<ContractState> { // Similar to `impl Trait for Struct`, but we are doing `impl trait ISimpleVault of module SimpleVault `
        
        // Get a user's vault share balance
        fn user_balance_of(ref self: ContractState, account: ContractAddress) -> u256 {
            self.balance_of.read(account)
        }

        // Get the total number of vault shares
        fn contract_total_supply(ref self: ContractState) -> u256 {
            self.total_supply.read()
        }

        // Deposit tokens into the vault
        fn deposit(ref self: ContractState, amount: u256) {
            let caller = get_caller_address(); // The user depositing
            let this = get_contract_address(); // The vault contract address

            let mut shares = 0;
            if self.total_supply.read() == 0 {
                // If no shares exist yet, mint 1:1 with the deposit
                shares = amount;
            } else {
                // Otherwise, calculate proportional shares
                let balance: u256 = self.token.read().balance_of(this).try_into()
                    .unwrap(); // Current vault balance
                shares = (amount * self.total_supply.read()) / balance;
            }

            // Mint shares to the depositor
            PrivateFunctions::_mint(ref self, caller, shares);

            // Transfer tokens from the user to the vault
            let amount_felt252: felt252 = amount.low.into();
            self.token.read().transfer_from(caller, this, amount_felt252);
        }

        // Withdraw tokens by burning shares
        fn withdraw(ref self: ContractState, shares: u256) {
            let caller = get_caller_address(); // The user withdrawing
            let this = get_contract_address(); // The vault address

            let balance = self.user_balance_of(this); // Vault token balance
            let amount = (shares * balance) / self.total_supply.read(); // Calculate how much to return
            PrivateFunctions::_burn(ref self, caller, shares); // Burn shares

            let amount_felt252: felt252 = amount.low.into(); // Adjust types
            self.token.read().transfer(caller, amount_felt252); // Transfer tokens back to user
        }
    }
}
```


## Exercise 4 - Security analysis - NEAR Smart contract

The content of this exercise is available at https://github.com/zigtur/vulnerable-NEAR-contract/

- Audit the smart contract written for the NEAR blockchain
- Describe at least 2 issues with high severity. Write a recommendation to fix the code!
- Write one Proof-of-Concept for each issue as a unit test.
    - Always initialize the contract with “admin.near” as AccountId, so that the first token is minted to this user.
    - Write your PoC with Bob as the attacker.
        
        ```rust
        #[cfg(test)]
        mod tests {
            use near_sdk::{test_utils::VMContextBuilder, testing_env};
            use super::*;
            
            #[test]
            fn a_unit_test() {
                let bob: AccountId = "bob.near".parse().unwrap();
                set_context(bob.clone()); // bob.near will be the account used for the following operations.
                
                // POC here
            }
        
            // Auxiliar fn: create a mock context
            fn set_context(predecessor: AccountId) {
                let mut builder = VMContextBuilder::new();
                builder.predecessor_account_id(predecessor);
        
                testing_env!(builder.build());
            }
        }
        ```


# NEAR Smart Contract Audit Report

## Overview
This audit analyzes a NEAR blockchain smart contract designed for a minimal NFT ownership system. The contract allows users to mint, approve, and transfer tokens identified by IDs. Two high severity vulnerabilities were identified.

---

## High Severity Issues Identified

### 1. Token ID Overflow and Ownership Overwrite

**Description:**
- In the `mint()` function, token IDs are stored by only using the lowest byte of the supply counter: `supply.to_le_bytes()[0]`.
- This limits token IDs to the range 0-255 (`u8`).
- After 256 mints, token IDs wrap around and start reusing IDs from 0, overwriting existing token ownership.

**Impact:**
- Token owners (such as `admin`) can lose ownership of previously minted NFTs.
- An attacker (bob`) can overwrite and steal the NFTs.

**Proof-of-Concept:**
```rust
#[test]
fn exploit_mint_overflow() {
    let bob: AccountId = "bob.near".parse().unwrap();
    set_context(bob.clone());
    let admin: AccountId = "admin.near".parse().unwrap();
    let mut contract = Contract::init(admin.clone());
    assert_eq!(contract.owner_of(0).unwrap(), admin);

    for _ in 0..256 {
        contract.mint();
    }
    assert_eq!(contract.supply, 257);
    assert_eq!(contract.owner_of(0).unwrap(), bob); // Ownership overwritten
}
```

**Recommendation:**
- Enforce a maximum supply of 255 tokens.
- Alternatively, properly store the full `u16` supply without truncating to a single byte.
- Check that the token ID is not already minted before inserting.

**Sample Fix:**
```rust
require!(self.supply <= u8::MAX as u16, "Maximum NFT supply reached");
let id = self.supply.to_le_bytes()[0];
require!(self.tokens.get(&id).is_none(), "Token already exists!");
self.tokens.insert(id, env::predecessor_account_id());
```

---

### 2. Approval Not Cleared After Transfer

**Description:**
- When a token is transferred, the contract does not remove the existing approval associated with that token ID.
- This allows previously approved addresses to continue controlling the token after it has been transferred to a new owner.

**Impact:**
- Attackers (`bob`) can exploit old approvals to steal tokens back from new owners.
- This breaks the ownership trust model and results in potential permanent asset loss.

**Proof-of-Concept:**
```rust
#[test]
fn exploit_approval_transfer() {
    let bob: AccountId = "bob.near".parse().unwrap();
    set_context(bob.clone());
    let admin: AccountId = "admin.near".parse().unwrap();
    let mut contract = Contract::init(admin.clone());
    assert_eq!(contract.owner_of(0).unwrap(), admin);

    let id = contract.mint();
    assert_eq!(contract.owner_of(id).unwrap(), bob);

    contract.approve(id, bob.clone());
    contract.transfer(id, admin.clone());
    assert_eq!(contract.owner_of(id).unwrap(), admin);
    assert_eq!(contract.approvals.get(&id).unwrap().clone(), bob); // Still approved

    contract.transfer(id, bob.clone());
    assert_eq!(contract.owner_of(id).unwrap(), bob); // Bob steals it back
}
```

**Recommendation:**
- Clear any approvals immediately after transferring a token.

**Sample Fix:**
```rust
self.tokens.insert(id, receiver);
self.approvals.remove(&id); // Clear approval after transfer
```

---

## Conclusion

Both identified issues are categorized as high severity and must be addressed immediately to ensure the security and correctness of the contract. Fixing these vulnerabilities will prevent asset loss, ownership overwrites, and unintended token transfers, aligning the contract behavior with expectations for secure NFT ownership on the NEAR blockchain.

## Summary Table

| Issue | Description | Severity | Recommendation |
|------|--------------|----------|----------------|
| Token ID Overflow and Ownership Overwrite | Minting >255 NFTs causes ID wrapping and overwrites existing ownership. | High | Enforce max supply or handle full `u16` IDs safely. |
| Approval Not Cleared After Transfer | Old approved delegates can steal tokens post-transfer. | High | Clear approvals immediately after every transfer. |

---


