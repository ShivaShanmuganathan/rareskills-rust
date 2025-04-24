fn function_1(var: &String) {
    println!("In function_1, variable is: {}", var);
}

fn function_2(var: String) {
    println!("In function_2, variable is: {}", var);
}

fn function_3(var: u16) {
    println!("In function_3, variable is: {}", var);
}

fn main() {
    let variable = String::from("Welcome to RustSkills");
    function_1(&variable);
    println!("In main, variable is: {}", variable);

    let variable = String::from("Welcome to RustSkills 2");
    function_2(variable.clone());
    println!("In main, variable is: {}", variable);

    let variable = 10;
    function_3(variable);
    println!("In main, variable is: {}", variable);
}
