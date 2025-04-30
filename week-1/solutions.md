### Exercise 1 - Ownership and Borrowing

- Explain why this code snippet does not work.
    - The ownership of the String gets transferred to function_1, so when it goes out of scope, it gets dropped.
- Give at least 2 ways to fix the issue (there are more than 2).
    - Pass a reference to the String instead of passing the String itself.
    - Use the `clone` method to create a new copy of the String.
- Replace the `String` variable with a scalar variable (`u32, i32, u64, i64, …`) and retest the same code snippet.
    - Why does it work?
        - Scalar variables are copied on the stack, so there is no ownership transfer.



### Exercise 2 - Code analysis
For each provided code snippet, analyze and document what it should do.

#### Snippet 1
- Without testing it, what is wrong with this code snippet?
    - The vector is not mutable, so the push method is not allowed.
- How can it be fixed?
    - Make the vector mutable.
```rust
fn main() {
    let a = vec![1,2,3,4];
    a.push(27);
}
```

#### Snippet 2
- Without testing it, what is wrong with this code snippet?
    - The function parameter `a` is not mutable, so the += operator is not allowed.
- How can it be fixed?
    - Make the function parameter `a` mutable.

```rust
fn my_operation(a: u64, b: u64) -> u64 {
    a += b;
    a
}


fn main() {
    let num1 = 1234;
    let num2 = 1122;
    println!("My result is {}!", my_operation(num1, num2));
}
```


#### Snippet 3

Without executing the code, what is the printed value of x?
Test it and explain why x has this value.
- The printed value of x is 3. The mutable x in the inner scope has its value changed to 3, and the non-mutable x is dropped.
```rust
fn main() {
    let x = 1;

    {
        let mut x = 2;

        x = x ^ 2;

        {
            x = 3;
            let x = 12;
        }
        println!("x is: {}", x);
    }
}
```


#### Snippet 4
    
- The following Solidity and Rust snippets shows the (Key ⇒ Value) functionality. Solidity provides this through a mapping while Rust provides it through an Hashmap.  
    - What is the main difference between the two languages about non-initialized data?
        - Rust's HashMap explicitly indicates the absence of a key through the Option::None variant.


### Exercise 3 - Security analysis

The user deposits collateral and receives a proportional number of share tokens based on the current exchange rate.

```rust
pub fn deposit(ctx: Context<Deposit>, collat: u64) -> Result<()> {
    let rate = exchange_rate.deposit_rate as u128;
    let amt = (collat as u128 * rate / DECIMALS_SCALAR) as u64; 

    token::transfer(collateral_token, ctx.caller, ctx.this, collat)?;
    token::mint_to(shares_token, ctx.caller, amt)?;

    Ok(())
}
```



If the final result exceeds u64::MAX, the cast to u64 will truncate it → wrong amount minted (likely too low).