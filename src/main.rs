#![allow(non_snake_case)]
fn main() {
    let arguments : Vec<_> = std::env::args().collect();
    let num1 = arguments[1].parse().unwrap(); // arg 2
    let num2  = arguments[3].parse().unwrap(); // arg 3
    let operator = arguments[2].chars().nth(0).unwrap(); // arg 4
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
            panic!();
        }
    }
}