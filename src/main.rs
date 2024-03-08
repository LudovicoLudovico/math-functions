use ruut_functions::F2D;
use std::str::FromStr;

fn main() {
    let func = F2D::from_str("(2pi)/[(xy)^(1/2)]").unwrap();
    println!("Function: {}", func);
    println!("Function: {:#?}", func);
    println!("Derivative: {:#?}", func.derivative());
    println!("Derivative: {}", func.derivative().x);
}
