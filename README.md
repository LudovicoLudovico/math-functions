A math library that allows for:
- parsing from string to function
- support for 1/2/3 variable(s) functions
- supports the following functions:
    - Ln, Sin, Cos, Tan, Sec, Csc, ASin, ACos, ATan, Sinh, Cosh, Tanh, Coth, Sech, Csch, ASinh, ACosh, ATanh, Abs
- add/sub/mul/div/pow between functions and f64
- Ability to define function and use them in other functions
- Operations for F1D (One dimensional functions):
    - Derivative
    - Definite integral between a and b
    - Evaluate functions at x
- Operations for F2D (Two dimensional functions):
    - Derivative
    - Hessian
    - Evaluate functions at (x,y)
- Operations for F3D (Three dimensional functions):
    - Derivative
    - Hessian
    - Evaluate functions at (x,y,z)

# Examples
```
use math_functions::{F1D, context::Context};

let func = F1D::from_str("(x+2)^(x+2)").unwrap();
let func_2 = F1D::from_str("x^2").unwrap();

let mut ctx = Context::new();
ctx.add_f1d("POWER", &func_2);

println!("FUNCTION: {}", func);
println!("FUNCTION: {}", func_2);

let func_3 = F1D::build("POWER+POWER", &ctx).unwrap();
println!("FUNCTION: {}", func_3); // "x^2+x^2"

```
