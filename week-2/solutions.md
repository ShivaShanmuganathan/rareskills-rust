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

### Solution 1 - `calculator.rs`


## Exercise 2 - Code analysis - NEAR Smart contract
Goal: Analyze a smart contract written in Rust for the NEAR blockchain. ⚠️ This is not a security analysis.
Note: Some concepts have not been explained yet, give it your best! 
Expected outputs:
A summary explaining the purpose of this contract (should fit in 5-6 lines)
An in-depth analysis of the contract. Comments should be added to the code snippet to explain the concepts shown in Lecture of Week 2.

```rust
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U64;
use near_sdk::serde::Serialize;
use near_sdk::store::Vector;
use near_sdk::{env, near_bindgen, AccountId, NearToken};

const POINT_ONE: NearToken = NearToken::from_millinear(100);

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[borsh(crate = "near_sdk::borsh")]
pub struct PostedMessage {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[borsh(crate = "near_sdk::borsh")]
pub struct Contract {
    messages: Vector<PostedMessage>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            messages: Vector::new(b"m"),
        }
    }
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn add_message(&mut self, text: String) {
        let premium = env::attached_deposit() >= POINT_ONE;
        let sender = env::predecessor_account_id();

        let message = PostedMessage {
            premium,
            sender,
            text,
        };

        self.messages.push(message);
    }

    pub fn get_messages(&self, from_index: Option<U64>, limit: Option<U64>) -> Vec<&PostedMessage> {
        let from = u64::from(from_index.unwrap_or(U64(0)));
        let limit = u64::from(limit.unwrap_or(U64(10)));

        self.messages
            .iter()
            .skip(from as usize)
            .take(limit as usize)
            .collect()
    }

    pub fn total_messages(&self) -> u32 {
        self.messages.len()
    }

    pub fn get_my_messages(&self) -> Vec<&PostedMessage> {
        let caller = env::predecessor_account_id();
        self.messages
            .iter()
            .filter(|msg| msg.sender == caller)
            .collect()
    }

    pub fn my_total_messages(&self) -> u32 {
        let caller = env::predecessor_account_id();
        self.messages
            .iter() 
            .filter(|msg| msg.sender == caller)
            .count() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_message() {
        let mut contract = Contract::default();
        contract.add_message("A message".to_string());

        let posted_message = &contract.get_messages(None, None)[0];
        assert_eq!(posted_message.premium, false);
        assert_eq!(posted_message.text, "A message".to_string());
    }

    #[test]
    fn iters_messages() {
        let mut contract = Contract::default();
        contract.add_message("1st message".to_string());
        contract.add_message("2nd message".to_string());
        contract.add_message("3rd message".to_string());

        let total = &contract.total_messages();
        assert!(*total == 3);

        let last_message = &contract.get_messages(Some(U64::from(1)), Some(U64::from(2)))[1];
        assert_eq!(last_message.premium, false);
        assert_eq!(last_message.text, "3rd message".to_string());
    }
}
```


### Solution 2

A summary explaining the purpose of this contract (should fit in 5-6 lines)