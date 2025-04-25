mod calculator;

use calculator::Calculator;

fn main() {
    // Exercise 1 - Calculator
    let calculator = Calculator { x: 1, y: 2 };
    println!("Calculator:\n{}", calculator);
}
