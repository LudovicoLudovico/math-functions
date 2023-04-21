use ruut_functions::{context::Context, F1D, F2D, F3D};
use std::str::FromStr;

fn main() {
    let func = F2D::from_str("(2pi)/[(xy)^(0.5)]").unwrap();
    println!("Function: {}", func);
    println!("Function: {:#?}", func);
    println!("Derivative: {:#?}", func.derivative() );
    println!("Derivative: {}", func.derivative().x );
}
