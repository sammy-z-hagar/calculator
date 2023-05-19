use std::env;

mod reverse_polish_calculator;

use reverse_polish_calculator::calculator::Calculator;

fn main() {
    let args: Vec<String> = env::args().collect();

    let calculator = Calculator::new(args[1..].to_vec());
    println!("{}", calculator.calculate());
}
