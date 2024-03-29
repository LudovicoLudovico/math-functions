use super::Matrix;
use super::{Function, FunctionType, Operation, F1D, F2D, F3D};
use super::{Vec2, Vec3};
use crate::algebra::rational::Rational;
use crate::context::Context;
use crate::parser::splitter::split;
use crate::parser::{parse, ParsingError};
use std::fmt::Display;

impl Function {
    fn evaluate(&self, inputs: &[f64]) -> f64 {
        match &self {
            Self::X => inputs[0],
            Self::Y => inputs[1],
            Self::Z => inputs[2],
            Self::Rational(val) => val.eval(),
            Self::E => std::f64::consts::E,
            Self::PI => std::f64::consts::PI,
            Self::Binary { operation, terms } => {
                let left = terms.0.evaluate(inputs);
                let right = terms.1.evaluate(inputs);
                eval_ops(operation, left, right)
            }
            Self::Special { kind, argument } => {
                let argument = argument.evaluate(inputs);
                eval_trascendental(kind, argument)
            }
        }
    }

    fn derivative(&self, on_x: i32, on_y: i32, on_z: i32) -> Self {
        match self {
            Self::X => Self::Rational(Rational::new_from_int(on_x)),
            Self::Y => Self::Rational(Rational::new_from_int(on_y)),
            Self::Z => Self::Rational(Rational::new_from_int(on_z)),
            Self::Rational(_) => Self::Rational(Rational::zero()),
            Self::E | Self::PI => Self::Rational(Rational::zero()),
            Self::Binary { operation, terms } => match operation {
                Operation::Add => {
                    terms.0.derivative(on_x, on_y, on_z) + terms.1.derivative(on_x, on_y, on_z)
                }
                Operation::Sub => {
                    terms.0.derivative(on_x, on_y, on_z) - terms.1.derivative(on_x, on_y, on_z)
                }
                Operation::Mul => {
                    terms.0.derivative(on_x, on_y, on_z) * *terms.1.clone()
                        + *terms.0.clone() * terms.1.derivative(on_x, on_y, on_z)
                }
                Operation::Div => {
                    (terms.0.derivative(on_x, on_y, on_z) * *terms.1.clone()
                        - *terms.0.clone() * terms.1.derivative(on_x, on_y, on_z))
                        / (terms.1.clone().powr(Rational::new_from_int(2)))
                }
                Operation::Pow => {
                    if let Function::Rational(val) = &*terms.1 {
                        if on_x != 0 {
                            if let Function::X = *terms.0 {
                                return val.clone() * Function::X.powr(val.clone() - 1);
                            }
                        } else if on_y != 0 {
                            if let Function::Y = *terms.0 {
                                return val.clone() * Function::Y.powr(val.clone() - 1);
                            }
                        } else if let Function::Z = *terms.0 {
                            return val.clone() * Function::Z.powr(val.clone() - 1);
                        };

                        terms.0.derivative(on_x, on_y, on_z)
                            * (val.clone() * terms.0.clone().powr(val.clone() - 1))
                    } else if let Function::E = *terms.0 {
                        terms.1.derivative(on_x, on_y, on_z)
                            * (terms.0.clone().pow(*terms.1.clone()))
                    } else {
                        Function::E
                            .pow(
                                *terms.1.clone()
                                    * Function::Special {
                                        kind: FunctionType::Ln,
                                        argument: terms.0.clone(),
                                    },
                            )
                            .derivative(on_x, on_y, on_z)
                    }
                }
                Operation::Comp => panic!("Something went wrong"),
            },

            Self::Special { kind, argument } => {
                let arg = argument.derivative(on_x, on_y, on_z);
                let argument = Box::new(*argument.clone());

                match kind {
                    FunctionType::Ln => arg / *argument,
                    FunctionType::Sin => {
                        arg * Function::Special {
                            kind: FunctionType::Cos,
                            argument,
                        }
                    }
                    FunctionType::Cos => {
                        -1 * arg
                            * Function::Special {
                                kind: FunctionType::Sin,
                                argument,
                            }
                    }
                    FunctionType::Tan => {
                        arg * Function::Special {
                            kind: FunctionType::Sec,
                            argument,
                        }
                        .powr(Rational::new_from_int(2))
                    }
                    FunctionType::Cot => {
                        -1 * arg
                            / (Function::Special {
                                kind: FunctionType::Sin,
                                argument,
                            })
                            .powr(Rational::new_from_int(2))
                    }
                    FunctionType::Sec => {
                        arg * Function::Special {
                            kind: FunctionType::Sec,
                            argument: argument.clone(),
                        } * Function::Special {
                            kind: FunctionType::Tan,
                            argument,
                        }
                    }
                    FunctionType::Csc => {
                        -1 * arg
                            * Function::Special {
                                kind: FunctionType::Cot,
                                argument: argument.clone(),
                            }
                            * Function::Special {
                                kind: FunctionType::Csc,
                                argument: argument.clone(),
                            }
                    }
                    FunctionType::ASin => {
                        arg / (1 - argument.powr(Rational::new_from_int(2)))
                            .powr(Rational::new(1, 2))
                    }
                    FunctionType::ACos => {
                        -1 * arg
                            / (1 - argument.powr(Rational::new_from_int(2)))
                                .powr(Rational::new(1, 2))
                    }
                    FunctionType::ATan => arg / (1 + argument.powr(Rational::new_from_int(2))),
                    FunctionType::Sinh => {
                        arg * Function::Special {
                            kind: FunctionType::Cosh,
                            argument,
                        }
                    }
                    FunctionType::Cosh => {
                        arg * Function::Special {
                            kind: FunctionType::Sinh,
                            argument,
                        }
                    }
                    FunctionType::Tanh => {
                        arg * Function::Special {
                            kind: FunctionType::Sech,
                            argument,
                        }
                        .powr(Rational::new_from_int(2))
                    }
                    FunctionType::Coth => {
                        -1 * arg
                            * Function::Special {
                                kind: FunctionType::Csch,
                                argument,
                            }
                            .powr(Rational::new_from_int(2))
                    }
                    FunctionType::Sech => {
                        -1 * arg
                            * Function::Special {
                                kind: FunctionType::Sech,
                                argument: argument.clone(),
                            }
                            * Function::Special {
                                kind: FunctionType::Tanh,
                                argument: argument.clone(),
                            }
                    }
                    FunctionType::Csch => {
                        -1 * arg
                            * Function::Special {
                                kind: FunctionType::Csch,
                                argument: argument.clone(),
                            }
                            * Function::Special {
                                kind: FunctionType::Coth,
                                argument: argument.clone(),
                            }
                    }
                    FunctionType::ASinh => {
                        arg / (1 + argument.powr(Rational::new_from_int(2)))
                            .powr(Rational::new(1, 2))
                    }
                    FunctionType::ACosh => {
                        arg * (argument.powr(Rational::new_from_int(2))
                            - Function::Rational(Rational::new_from_int(1)))
                        .powr(Rational::new(1, 2))
                    }
                    FunctionType::ATanh => arg / (1 - argument.powr(Rational::new_from_int(2))),
                    FunctionType::Abs => {
                        arg * *argument.clone()
                            / Function::Special {
                                kind: FunctionType::Abs,
                                argument,
                            }
                    }
                }
            }
        }
    }
}
impl F1D {
    /// Builds a F1D from a string and a context (meaning that you can use already created
    /// functions)
    /// ```
    /// use ruut_functions::{F1D, context::Context};
    /// use std::str::FromStr;
    ///
    /// let func = F1D::from_str("x^2").unwrap();
    /// let mut ctx = Context::new();
    ///
    /// ctx.add_f1d("POWER", &func);
    ///
    /// let func2 = F1D::build("POWER(x)+POWER(x)", &ctx);
    ///
    /// assert_eq!(func2, F1D::from_str("x^2+x^2"));
    /// ```
    pub fn build(input: &str, ctx: &Context) -> Result<Self, ParsingError> {
        let res = parse(split(input)?, ctx, 1);
        match res {
            Ok(func) => Ok(F1D(func)),
            Err(err) => Err(err),
        }
    }

    /// Evaluate F1D at a given x
    /// ```
    /// use ruut_functions::{F1D,approx};
    /// use std::str::FromStr;
    ///
    /// let func = F1D::from_str("xsin(x)").unwrap();
    ///
    /// assert_eq!(approx(func.eval(2.), 5), 1.81859);
    /// ```
    pub fn eval(&self, x: f64) -> f64 {
        self.0.evaluate(&[x, 0., 0.])
    }

    /// Computes the derivative of a F1D
    /// ```
    /// use ruut_functions::F1D;
    /// use std::str::FromStr;
    ///
    /// let func = F1D::from_str("xln(x)").unwrap();
    ///
    /// assert_eq!(func.derivative(), F1D::from_str("ln(x)+1").unwrap());
    /// ```
    pub fn derivative(&self) -> Self {
        F1D(self.0.derivative(1, 0, 0))
    }

    /// Computes the definite integral of F1D
    /// ```
    /// use ruut_functions::{F1D,approx};
    /// use std::str::FromStr;
    ///
    /// let func = F1D::from_str("x^2+6").unwrap();
    ///
    /// assert_eq!(approx(func.integrate(0.,1., 10_000), 5), 6.33333)
    /// ```
    pub fn integrate(&self, a: f64, b: f64, steps: u32) -> f64 {
        let mut result = 0.;

        for i in 1..=steps {
            // Evaluating function at midpoint of dx
            result += self.eval(a + ((b - a) / steps as f64) * (i as f64 - 0.5));
        }

        ((b - a) / steps as f64) * result
    }
}

impl F2D {
    /// Builds a F2D from a string and a context (meaning that you can use already created
    /// functions)
    /// ```
    /// use ruut_functions::{F1D,F2D,F3D, context::Context};
    /// use std::str::FromStr;
    ///
    /// let func = F1D::from_str("x^2").unwrap();
    /// let mut ctx = Context::new();
    ///
    /// ctx.add_f1d("POWER", &func);
    ///
    /// let func2 = F2D::build("y(POWER+POWER)", &ctx).unwrap();
    ///
    /// assert_eq!(func2, F2D::from_str("y(x^2+x^2)").unwrap());
    /// ```
    pub fn build(input: &str, ctx: &Context) -> Result<Self, ParsingError> {
        let res = parse(split(input)?, ctx, 2);
        match res {
            Ok(func) => Ok(F2D(func)),
            Err(err) => Err(err),
        }
    }
    /// Evaluate F2D at a given (x,y)
    /// ```
    /// use ruut_functions::{F2D,approx};
    /// use std::str::FromStr;
    ///
    /// let func = F2D::from_str("ysin(x)").unwrap();
    ///
    /// assert_eq!(approx(func.eval(2., 0.5), 5), 0.45465);
    /// ```
    pub fn eval(&self, x: f64, y: f64) -> f64 {
        self.0.evaluate(&[x, y, 0.])
    }

    /// Computes the derivative of a F2D
    /// ```
    /// use ruut_functions::{F2D, Vec2};
    /// use std::str::FromStr;
    ///
    /// let func = F2D::from_str("yln(x)").unwrap();
    ///
    /// assert_eq!(func.derivative(), Vec2{ x: F2D::from_str("y/x").unwrap(), y:
    /// F2D::from_str("ln(x)").unwrap()});
    /// ```
    pub fn derivative(&self) -> Vec2<Self> {
        Vec2 {
            x: F2D(self.0.derivative(1, 0, 0)),
            y: F2D(self.0.derivative(0, 1, 0)),
        }
    }
}
impl F3D {
    /// Builds a F3D from a string and a context (meaning that you can use already created
    /// functions)
    /// ```
    /// use ruut_functions::{F2D,F3D, context::Context};
    /// use std::str::FromStr;
    ///
    /// let func = F2D::from_str("yx^2").unwrap();
    /// let mut ctx = Context::new();
    ///
    /// ctx.add_f2d("CUSTOM", &func);
    ///
    /// let func2 = F3D::build("z(CUSTOM+CUSTOM)", &ctx).unwrap();
    ///
    /// assert_eq!(func2, F3D::from_str("z(yx^2+yx^2)").unwrap());
    /// ```
    pub fn build(input: &str, ctx: &Context) -> Result<Self, ParsingError> {
        let res = parse(split(input)?, ctx, 3);
        match res {
            Ok(func) => Ok(F3D(func)),
            Err(err) => Err(err),
        }
    }
    /// Evaluate F3D at a given (x,y,z)
    /// ```
    /// use ruut_functions::{F3D,approx};
    /// use std::str::FromStr;
    ///
    /// let func = F3D::from_str("ysin(x)ln(z)").unwrap();
    ///
    /// assert_eq!(approx(func.eval(2., 0.5, 4.), 5), 0.63028);
    /// ```
    pub fn eval(&self, x: f64, y: f64, z: f64) -> f64 {
        self.0.evaluate(&[x, y, z])
    }

    /// Computes the gradient of a F3D
    /// ```
    /// use ruut_functions::{F3D, Vec3};
    /// use std::str::FromStr;
    ///
    /// let func = F3D::from_str("xyz^2").unwrap();
    ///
    /// assert_eq!(func.derivative(), Vec3{ x: F3D::from_str("yz^2").unwrap(), y:
    /// F3D::from_str("xz^2").unwrap(), z: F3D::from_str("2xyz").unwrap()});
    /// ```
    pub fn derivative(&self) -> Vec3<Self> {
        Vec3 {
            x: F3D(self.0.derivative(1, 0, 0)),
            y: F3D(self.0.derivative(0, 1, 0)),
            z: F3D(self.0.derivative(0, 0, 1)),
        }
    }
    /// Computes hessian matrix of the given function
    ///  ```
    ///  use ruut_functions::{F3D, Matrix};
    ///  use std::str::FromStr;
    ///  let func = F3D::from_str("3x^2+y^4+xyz^2").unwrap();
    ///  let hessian = func.hessian();
    ///  println!("{}", hessian);
    /// //|         6          |        z^2         |        2yz         |
    /// //|        z^2         |       12y^2        |        2xz         |
    /// //|        2yz         |        2xz         |        2xy         |
    /// let result: Matrix<F3D> = Matrix::new(
    ///     vec![
    ///         F3D::from_str("6").unwrap(),
    ///         F3D::from_str("z^2").unwrap(),
    ///         F3D::from_str("2yz").unwrap(),
    ///         F3D::from_str("z^2").unwrap(),
    ///         F3D::from_str("12y^2").unwrap(),
    ///         F3D::from_str("2xz").unwrap(),
    ///         F3D::from_str("2yz").unwrap(),
    ///         F3D::from_str("2xz").unwrap(),
    ///         F3D::from_str("2xy").unwrap(),
    ///     ],
    ///     3,
    ///     3,
    /// );
    /// assert_eq!(result, hessian);
    /// ```
    pub fn hessian(&self) -> Matrix<F3D> {
        let deriv_x = self.0.derivative(1, 0, 0);
        let deriv_y = self.0.derivative(0, 1, 0);
        let deriv_z = self.0.derivative(0, 0, 1);

        Matrix::new(
            vec![
                F3D(deriv_x.derivative(1, 0, 0)),
                F3D(deriv_x.derivative(0, 1, 0)),
                F3D(deriv_x.derivative(0, 0, 1)),
                F3D(deriv_y.derivative(1, 0, 0)),
                F3D(deriv_y.derivative(0, 1, 0)),
                F3D(deriv_y.derivative(0, 0, 1)),
                F3D(deriv_z.derivative(1, 0, 0)),
                F3D(deriv_z.derivative(0, 1, 0)),
                F3D(deriv_z.derivative(0, 0, 1)),
            ],
            3,
            3,
        )
    }
}

fn eval_trascendental(kind: &FunctionType, arg: f64) -> f64 {
    match kind {
        FunctionType::Ln => arg.ln(),
        FunctionType::Sin => arg.sin(),
        FunctionType::Cos => arg.cos(),
        FunctionType::Tan => arg.tan(),
        FunctionType::Cot => 1. / arg.tan(),
        FunctionType::Sec => 1. / arg.cos(),
        FunctionType::Csc => 1. / arg.sin(),
        FunctionType::ASin => arg.asin(),
        FunctionType::ACos => arg.acos(),
        FunctionType::ATan => arg.atan(),
        FunctionType::Sinh => arg.sinh(),
        FunctionType::Cosh => arg.cosh(),
        FunctionType::Tanh => arg.tanh(),
        FunctionType::Coth => 1. / arg.tanh(),
        FunctionType::Sech => 1. / arg.cosh(),
        FunctionType::Csch => 1. / arg.sinh(),
        FunctionType::ASinh => arg.asinh(),
        FunctionType::ACosh => arg.acosh(),
        FunctionType::ATanh => arg.atanh(),
        FunctionType::Abs => arg.abs(),
    }
}

fn eval_ops(operation: &Operation, left: f64, right: f64) -> f64 {
    match operation {
        Operation::Add => left + right,
        Operation::Sub => left - right,
        Operation::Mul => left * right,
        Operation::Div => left / right,
        Operation::Pow => left.powf(right),
        Operation::Comp => panic!("Somethig went wrong"),
    }
}
impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::E => write!(f, "e"),
            Self::PI => write!(f, "𝜋"),
            Self::X => write!(f, "x"),
            Self::Y => write!(f, "y"),
            Self::Z => write!(f, "z"),
            Self::Rational(val) => {
                if val.is_integer() {
                    if *val == 1 {
                        write!(f, "")
                    } else if *val == -1 {
                        write!(f, "-")
                    } else {
                        write!(f, "{}", val.num())
                    }
                } else {
                    write!(f, "{}/{}", val.num(), val.den())
                }
            }
            Self::Special { kind, argument } => match kind {
                FunctionType::Ln => write!(f, "ln({argument})"),
                FunctionType::Sin => write!(f, "sin({argument})"),
                FunctionType::Cos => write!(f, "cos({argument})"),
                FunctionType::Tan => write!(f, "tan({argument})"),
                FunctionType::Cot => write!(f, "cot({argument})"),
                FunctionType::Sec => write!(f, "sec({argument})"),
                FunctionType::Csc => write!(f, "csc({argument})"),
                FunctionType::ACos => write!(f, "acos({argument})"),
                FunctionType::ASin => write!(f, "asin({argument})"),
                FunctionType::ATan => write!(f, "atan({argument})"),
                FunctionType::Sinh => write!(f, "sinh({argument})"),
                FunctionType::Cosh => write!(f, "cosh({argument})"),
                FunctionType::Tanh => write!(f, "tanh({argument})"),
                FunctionType::Coth => write!(f, "coth({argument})"),
                FunctionType::Sech => write!(f, "sech({argument})"),
                FunctionType::Csch => write!(f, "csch({argument})"),
                FunctionType::ASinh => write!(f, "asinh({argument})"),
                FunctionType::ACosh => write!(f, "acosh({argument})"),
                FunctionType::ATanh => write!(f, "atanh({argument})"),
                FunctionType::Abs => write!(f, "|{argument}|"),
            },
            Self::Binary { terms, operation } => match operation {
                Operation::Add => write!(f, "{}+{}", terms.0, terms.1),
                Operation::Sub => write!(f, "{}-{}", terms.0, terms.1),
                Operation::Mul => {
                    let first = &terms.0;
                    let second = &terms.1;

                    if let Function::Binary {
                        operation: Operation::Add | Operation::Sub | Operation::Pow,
                        terms: _,
                    } = &*terms.0
                    {
                        return write!(f, "({}){}", first, second);
                    }
                    if let Function::Binary {
                        operation: Operation::Add | Operation::Sub | Operation::Pow,
                        terms: _,
                    } = &*terms.1
                    {
                        return write!(f, "{}({})", first, second);
                    }

                    write!(f, "{}{}", terms.0, terms.1)
                }
                Operation::Div => {
                    let first = &terms.0;
                    let second = &terms.1;

                    if let Function::Binary {
                        operation: Operation::Add | Operation::Sub,
                        terms: _,
                    } = &*terms.0
                    {
                        if let Function::Binary {
                            operation: _,
                            terms: _,
                        } = &*terms.1
                        {
                            return write!(f, "({})/({})", first, second);
                        }
                        return write!(f, "({})/{}", first, second);
                    }
                    if let Function::Binary {
                        operation: _,
                        terms: _,
                    } = &*terms.1
                    {
                        return write!(f, "{}/({})", first, second);
                    }
                    write!(f, "{}/{}", terms.0, terms.1)
                }
                Operation::Pow => {
                    let first = &terms.0;
                    let second = &terms.1;

                    if let Function::Binary {
                        operation: Operation::Add | Operation::Sub,
                        terms: _,
                    } = &*terms.0
                    {
                        if let Function::Binary {
                            operation: _,
                            terms: _,
                        } = &*terms.1
                        {
                            return write!(f, "({})^({})", first, second);
                        }
                        return write!(f, "({})^{}", first, second);
                    }
                    if let Function::Binary {
                        operation: _,
                        terms: _,
                    } = &*terms.1
                    {
                        return write!(f, "{}^({})", first, second);
                    }
                    write!(f, "{}^{}", terms.0, terms.1)
                }
                Operation::Comp => panic!("Something went wrong"),
            },
        }
    }
}

#[test]
fn test_derivative() {
    use std::str::FromStr;
    let func = F1D::from_str("3x+7+e").unwrap();
    assert_eq!(
        func.derivative(),
        F1D(Function::Rational(Rational::new_from_int(3)))
    );

    let func = F1D::from_str("x*sin(x)").unwrap();
    assert_eq!(
        func.derivative(),
        F1D(Function::Special {
            kind: FunctionType::Sin,
            argument: Box::new(Function::X)
        } + (Function::X
            * Function::Special {
                kind: FunctionType::Cos,
                argument: Box::new(Function::X)
            }))
    );

    let func = F1D::from_str("tan(x^2)").unwrap();
    assert_eq!(
        func.derivative(),
        F1D(
            (Function::Rational(Rational::new_from_int(2)) * Function::X)
                * Function::Special {
                    kind: FunctionType::Sec,
                    argument: Box::new(Function::X.powr(Rational::new_from_int(2)))
                }
                .powr(Rational::new_from_int(2))
        )
    );

    let func = F1D::from_str("x^x").unwrap();
    assert_eq!(
        func.derivative(),
        F1D::from_str("(ln(x)+1)e^(xln(x))").unwrap()
    );

    let func = F3D::from_str("xyz^2").unwrap();
    assert_eq!(
        func.derivative(),
        Vec3 {
            x: F3D::from_str("yz^2").unwrap(),
            y: F3D::from_str("xz^2").unwrap(),
            z: F3D::from_str("2xyz").unwrap()
        }
    );

    let func = F2D::from_str("(xy)^(-1/2)").unwrap();
    println!(
        "{}, {}, \n{:#?}",
        func,
        func.derivative(),
        func.derivative()
    );

    /* let func = F2D::from_str("1/(xy)^(1/2)").unwrap();
    println!("{} \n {} \n {:#?}", func, func.derivative(), func.derivative());
    assert_eq!(
        func.derivative(),
        Vec2 {
            x: F2D::from_str("-pi*y/(xy)^(3/2)").unwrap(),
            y: F2D::from_str("-pi*x/(xy)^(3/2)").unwrap(),
        }
    )*/
}

#[test]
fn test_hessian() {
    use std::str::FromStr;
    let func = F3D::from_str("3x^2+y^4+xyz^2").unwrap();
    let hessian = func.hessian();
    println!("\n{}", hessian);
    let result: Matrix<F3D> = Matrix::new(
        vec![
            F3D::from_str("6").unwrap(),
            F3D::from_str("z^2").unwrap(),
            F3D::from_str("2yz").unwrap(),
            F3D::from_str("z^2").unwrap(),
            F3D::from_str("12y^2").unwrap(),
            F3D::from_str("2xz").unwrap(),
            F3D::from_str("2yz").unwrap(),
            F3D::from_str("2xz").unwrap(),
            F3D::from_str("2xy").unwrap(),
        ],
        3,
        3,
    );
    assert_eq!(result, hessian);
}

#[test]
fn test_integration() {
    use crate::approx;
    use std::str::FromStr;
    let func = F1D::from_str("x^3").unwrap();
    assert_eq!(approx(func.integrate(-1., 1.5, 10_000), 4), 1.0156);

    let func = F1D::from_str("sin(x)^2").unwrap();
    assert_eq!(
        approx(func.integrate(0., 2. * std::f64::consts::PI, 10_000), 8),
        3.14159265
    );
}
