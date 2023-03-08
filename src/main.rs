use math_functions::functions::{Context, Function};
use std::str::FromStr;

fn main() {
    let mut ctx = Context::new();
    let func = Function::from_str("e^(x^2)").unwrap();
    ctx.add_func("myFunc", &func);
    ctx.add_symbol("yt", 69.);
    println!("CTX: {ctx:#?}");
    let func2 = Function::build("2+myFunc(x)+yt", &ctx).unwrap();
    println!("FUNC: {func2:#?}");
    let func3 = func + func2;
    println!("SUM: {func3:#?}");
}
