mod calculator;

use calculator::{print_output, Calculator};

fn main() {
    // Exercise 1 - Calculator
    let calculator = Calculator { x: 2, y: 2 };
    print_output(&calculator);

    let calc_float = Calculator { x: 10.5, y: 2.5 };
    print_output(&calc_float);
}
