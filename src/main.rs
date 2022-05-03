#![allow(non_snake_case)]
use std::process::exit as quit;
use std::env::args;

fn main() {
    let args : Vec<String> = args().collect(); // place all the command line args in a vector

    let num1 : f32 = args[1].parse().unwrap(); // arg 2
    let num2 : f32 = args[3].parse().unwrap(); // arg 3
    let operator : char = args[2].chars().nth(0).unwrap(); // arg 4

    let out = calc_value(operator, num1, num2);
    println!("{} {} {} = {:?}", num1, operator, num2, out);
}
// returns a float if the operator is valid and exits the program if it is not
fn calc_value(operator : char, num1 : f32, num2 : f32) -> f32 {
    match operator {
        // the four basic operations
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        _ => {
            println!("Invalid operator");
            quit(1);
        }
    }
}
