use math_functions::{context::Context, F1D, F2D, F3D};
use std::str::FromStr;

fn main() {
    let func = F1D::from_str("x^x").unwrap();
    println!("FUNCTION: {}", func);
    println!("DERIVATIVE: {}", func.derivative());

    let func = F2D::from_str("x+y^2").unwrap();
    println!("FUNCTION: {}", func);
    println!("DERIVATIVE: {}", func.derivative());

    let func = F3D::from_str("x^y+z").unwrap();
    println!("FUNCTION: {}", func);
    println!("DERIVATIVE: {}", func.derivative());

    let func = F3D::from_str("3x^3+xy+xz^5").unwrap();
    println!("FUNCTION: {}", func);
    println!("HESSIAN: \n{}", func.hessian());

    let func = F1D::from_str("(x+2)^(x+2)").unwrap();
    let func_2 = F1D::from_str("x^2").unwrap();
    let mut ctx = Context::new();
    ctx.add_f1d("POWER", &func_2);
    println!("FUNCTION: {}", func);
    println!("FUNCTION: {}", func_2);
    let func_3 = F1D::build("POWER+POWER", &ctx).unwrap();
    println!("FUNCTION: {}", func_3);
}
