# Meeting with Daniel [30.04.2025]

1. What is the main difference between debug and release mode in Rust?
1.1 When you’re compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two’s complement wrapping. 

2. Was working on building the HTTP Server from the codecrafters course. Then, jumped into the vulnerable HTTP Server section from week-3 and completed it