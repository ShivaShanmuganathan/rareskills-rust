### Exercise 1 - Ownership and Borrowing

- Explain why this code snippet does not work.
    - The ownership of the String gets transferred to function_1, so when it goes out of scope, it gets dropped.
- Give at least 2 ways to fix the issue (there are more than 2).
    - Pass a reference to the String instead of passing the String itself.
    - Use the `clone` method to create a new copy of the String.
- Replace the `String` variable with a scalar variable (`u32, i32, u64, i64, …`) and retest the same code snippet.
    - Why does it work?
        - Scalar variables are copied on the stack, so there is no ownership transfer.



