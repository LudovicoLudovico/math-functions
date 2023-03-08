#![deny(warnings, missing_docs)]
pub use self::context::Context;
use self::parser::parse;
use self::splitter::{split, ParsingError};
use std::fmt::Display;
use std::str::FromStr;

/// This modules contains the context related code
pub mod context;
/// This modules contains the implementations of the traits Add, Sub, Mul, Div on functions and
/// between functiosn and f64
pub mod operators;
mod parser;
mod splitter;
mod tests;

/// Enum describing the supported functions
#[derive(PartialEq, Debug, Clone)]
pub enum Function {
    /// Variable
    Var,
    /// Euler's Number
    E,
    /// Pi
    PI,
    /// A Generic Number
    Num(f64),
    /// Natural Logarithm
    Ln {
        /// Argument of the Logarithm
        argument: Box<Function>,
    },
    /// Sine
    Sin {
        /// Argument of the sine
        argument: Box<Function>,
    },
    /// Cosine
    Cos {
        /// Argument of the cosine
        argument: Box<Function>,
    },
    /// Tangent
    Tan {
        /// Argument of the tangent
        argument: Box<Function>,
    },
    /// Cotangent
    Cot {
        /// Argument of the cotangent
        argument: Box<Function>,
    },
    /// Secant
    Sec {
        /// Argument of the secant
        argument: Box<Function>,
    },
    /// Cosecant
    Csc {
        /// Argument of the cosecant
        argument: Box<Function>,
    },
    /// Arccos
    ACos {
        /// Argument of the arccos
        argument: Box<Function>,
    },
    /// Arcsin
    ASin {
        /// Argument of the arcsin
        argument: Box<Function>,
    },
    /// Arctan
    ATan {
        /// Argument of the arctan
        argument: Box<Function>,
    },
    /// Hyperbolic Sin
    Sinh {
        /// Argument of the hyperbolic sin
        argument: Box<Function>,
    },
    /// Hypberbolic Cos
    Cosh {
        /// Argument of the hyperbolic cos
        argument: Box<Function>,
    },
    /// Hyperbolic Tangent
    Tanh {
        /// Argument of the hyperbolic tangent
        argument: Box<Function>,
    },
    /// Hyperbolic Cotangent
    Coth {
        /// Argument of the hyperbolic cotangent
        argument: Box<Function>,
    },
    /// Hyperbolic Secant
    Sech {
        /// Argument of the hyperbolic secant
        argument: Box<Function>,
    },
    /// Hyperbolic Cosecant
    Csch {
        /// Argument of the hyperbolic cosecant
        argument: Box<Function>,
    },
    /// Inverse of the hyperbolic sin
    ASinh {
        /// Argument of the inverse of the hyperbolic sin
        argument: Box<Function>,
    },
    /// Inverse of the hyperbolic cos
    ACosh {
        /// Argument of the inverse hyperbolic cos
        argument: Box<Function>,
    },
    /// Inverse of the hyperbolic tangent
    ATanh {
        /// Argument of the hyperbolic tangent
        argument: Box<Function>,
    },
    /// Absolute value
    Abs {
        /// Argument of the aboslute value
        argument: Box<Function>,
    },
    /// Composite function:
    Comp {
        /// Tuple with 2 pointers to other functions
        terms: (Box<Function>, Box<Function>),
        /// Opertor between the functions (+, -, /, *, ^, composition)
        operator: FunctionOperator,
    },
}

/// Enum that describes the different types of operations that can be performed between two
/// functions
#[derive(PartialEq, Debug, Clone)]
pub enum FunctionOperator {
    /// Addition
    Add,
    /// Subtraction
    Sub,
    /// Multiplication
    Mul,
    /// Division
    Div,
    /// Raising to power
    Pow,
    /// Composition
    Comp,
}
impl FunctionOperator {
    fn priority(&self) -> u8 {
        match self {
            FunctionOperator::Add => 4,
            FunctionOperator::Sub => 4,
            FunctionOperator::Mul => 3,
            FunctionOperator::Div => 3,
            FunctionOperator::Pow => 2,
            FunctionOperator::Comp => 1,
        }
    }
}

impl Function {
    /// Builds a functions from a string and a Context
    /// ```
    /// use math_functions::functions::{Context, Function};
    ///
    /// let mut ctx = Context::new();
    /// ctx.add_symbol("R", 4.16);
    /// let func = Function::build("pi*R^2", &ctx).unwrap();
    /// assert_eq!(func, Function::PI * Function::Num(4.16).powf(2.));
    /// ```
    pub fn build(func: &str, ctx: &Context) -> Result<Function, ParsingError> {
        parse(split(func)?, ctx)
    }

    /// Raise a Function to a Function
    /// ```
    /// use std::str::FromStr;
    /// use math_functions::functions::{Function,FunctionOperator};
    /// let f1 = Function::from_str("2.0").unwrap();
    /// let f2 = Function::from_str("x").unwrap();
    ///
    /// let res = f1.pow(f2);
    /// assert_eq!(res, Function::Comp { terms: (Box::new(Function::Num(2.)), Box::new(Function::Var)), operator: FunctionOperator::Pow})
    /// ```
    pub fn pow(self, rhs: Self) -> Self {
        Function::Comp {
            terms: (Box::new(self), Box::new(rhs)),
            operator: FunctionOperator::Pow,
        }
    }
    /// Raise a Function to a f64
    /// ```
    /// use math_functions::functions::{Function, FunctionOperator};
    /// use std::str::FromStr;
    ///
    /// let f1 = Function::from_str("x").unwrap();
    /// let f2 = Function::from_str("2.0").unwrap();
    ///
    /// let res = f1.pow(f2);
    /// assert_eq!(res, Function::Comp { terms: (Box::new(Function::Var), Box::new(Function::Num(2.))), operator: FunctionOperator::Pow})
    /// ```
    pub fn powf(self, rhs: f64) -> Self {
        if rhs == 1. {
            return self;
        }

        if rhs == 0. {
            return Function::Num(1.);
        }

        Function::Comp {
            terms: (Box::new(self), Box::new(Function::Num(rhs))),
            operator: FunctionOperator::Pow,
        }
    }

    /// Evaluates the function at a given point x
    /// ```
    /// use std::str::FromStr;
    /// use math_functions::functions::Function;
    /// let f1 = Function::from_str("e^(x^2)").unwrap();
    /// assert_eq!(f1.eval(2.3), 198.34342540938096)
    /// ```
    pub fn eval(&self, x: f64) -> f64 {
        match self {
            Self::Var => x,
            Self::Num(val) => *val,
            Self::E => std::f64::consts::E,
            Self::PI => std::f64::consts::PI,
            Self::Ln { argument } => argument.eval(x).ln(),
            Self::Cos { argument } => argument.eval(x).cos(),
            Self::Sin { argument } => argument.eval(x).sin(),
            Self::Tan { argument } => argument.eval(x).tan(),
            Self::Cot { argument } => 1. / argument.eval(x).tan(),
            Self::Sec { argument } => 1. / argument.eval(x).cos(),
            Self::Csc { argument } => 1. / argument.eval(x).sin(),
            Self::Sinh { argument } => argument.eval(x).sinh(),
            Self::Cosh { argument } => argument.eval(x).cosh(),
            Self::Tanh { argument } => argument.eval(x).tanh(),
            Function::Coth { argument } => 1. / (argument.eval(x)).tanh(),
            Function::Sech { argument } => {
                2. / (Function::E.powf(argument.eval(x)) + Function::E.powf(argument.eval(-x)))
                    .eval(x)
            }
            Function::Csch { argument } => {
                1. / Function::Sinh {
                    argument: argument.clone(),
                }
                .eval(x)
            }
            Self::ASin { argument } => argument.eval(x).asin(),
            Self::ACos { argument } => argument.eval(x).acos(),
            Self::ATan { argument } => argument.eval(x).atan(),
            Self::ASinh { argument } => argument.eval(x).asinh(),
            Self::ACosh { argument } => argument.eval(x).acosh(),
            Self::ATanh { argument } => argument.eval(x).atanh(),
            Self::Abs { argument } => {
                let mut result = argument.eval(x);
                if result < 0. {
                    result *= -1.;
                }

                result
            }
            Self::Comp { terms, operator } => {
                let f = terms.0.eval(x);
                let g = terms.1.eval(x);

                match operator {
                    FunctionOperator::Add => f + g,
                    FunctionOperator::Sub => f - g,
                    FunctionOperator::Mul => f * g,
                    FunctionOperator::Div => f / g,
                    FunctionOperator::Pow => f.powf(g),
                    FunctionOperator::Comp => panic!("Found Function of function"),
                }
            }
        }
    }

    /// Returns the derivative of the given function
    pub fn derivative(&self) -> Function {
        match self {
            Function::E | Function::PI => Function::Num(0.),
            Function::Num(_val) => Function::Num(0.),
            Function::Var => Function::Num(1.),
            Function::Ln { argument } => argument.derivative() / *argument.clone(),
            Function::Sin { argument } => {
                argument.derivative()
                    * Function::Cos {
                        argument: Box::new(*argument.clone()),
                    }
            }
            Function::Cos { argument } => {
                argument.derivative()
                    * Function::Num(-1.)
                    * Function::Cos {
                        argument: Box::new(*argument.clone()),
                    }
            }
            Function::Tan { argument } => {
                argument.derivative()
                    * Function::Sec {
                        argument: Box::new(*argument.clone()),
                    }
                    .powf(2.)
            }
            Function::Cot { argument } => {
                -1. * argument.derivative()
                    / (Function::Sin {
                        argument: Box::new(*argument.clone()),
                    })
                    .powf(2.)
            }
            Function::Sec { argument } => {
                argument.derivative()
                    * Function::Sec {
                        argument: argument.clone(),
                    }
                    * Function::Tan {
                        argument: argument.clone(),
                    }
            }
            Function::Csc { argument } => {
                -1. * argument.derivative()
                    * Function::Cot {
                        argument: argument.clone(),
                    }
                    * Function::Csc {
                        argument: argument.clone(),
                    }
            }
            Function::ASin { argument } => {
                argument.derivative() / (1. - argument.clone().powf(2.)).powf(0.5)
            }
            Function::ACos { argument } => {
                -1. * argument.derivative() / (1. - argument.clone().powf(2.)).powf(0.5)
            }
            Function::ATan { argument } => argument.derivative() / (1. + argument.clone().powf(2.)),
            Function::Sinh { argument } => {
                argument.derivative()
                    * Function::Cosh {
                        argument: argument.clone(),
                    }
            }
            Function::Cosh { argument } => {
                argument.derivative()
                    * Function::Sinh {
                        argument: argument.clone(),
                    }
            }
            Function::Tanh { argument } => {
                argument.derivative()
                    * Function::Sech {
                        argument: argument.clone(),
                    }
                    .powf(2.)
            }
            Function::Coth { argument } => {
                -1. * argument.derivative()
                    * Function::Csch {
                        argument: argument.clone(),
                    }
                    .powf(2.)
            }
            Function::Sech { argument } => {
                -1. * argument.derivative()
                    * Function::Sech {
                        argument: argument.clone(),
                    }
                    * Function::Tanh {
                        argument: argument.clone(),
                    }
            }
            Function::Csch { argument } => {
                -1. * argument.derivative()
                    * Function::Csch {
                        argument: argument.clone(),
                    }
                    * Function::Coth {
                        argument: argument.clone(),
                    }
            }
            Function::ASinh { argument } => {
                argument.derivative() / (1. + argument.clone().powf(2.)).powf(0.5)
            }
            Function::ACosh { argument } => {
                argument.derivative() * (argument.clone().powf(2.) - 1.).powf(0.5)
            }
            Function::ATanh { argument } => {
                argument.derivative() / (1. - argument.clone().powf(2.))
            }
            Function::Abs { argument } => {
                argument.derivative() * *argument.clone()
                    / Function::Abs {
                        argument: argument.clone(),
                    }
            }
            Function::Comp { terms, operator } => match operator {
                FunctionOperator::Add => terms.0.derivative() + terms.1.derivative(),
                FunctionOperator::Sub => terms.0.derivative() - terms.1.derivative(),
                FunctionOperator::Mul => {
                    terms.0.derivative() * *terms.1.clone()
                        + *terms.0.clone() * terms.1.derivative()
                }
                FunctionOperator::Div => {
                    (terms.0.derivative() * *terms.1.clone()
                        - *terms.0.clone() * terms.1.derivative())
                        / (*terms.1.clone() * *terms.1.clone())
                }
                FunctionOperator::Pow => {
                    if let Function::Num(val) = *terms.1 {
                        if let Function::Var = *terms.0 {
                            return val * Function::Var.powf(val - 1.);
                        }

                        terms.0.derivative() * (val * terms.1.clone().powf(val - 1.))
                    } else if let Function::E = *terms.0 {
                        terms.1.derivative() * (terms.0.clone().pow(*terms.1.clone()))
                    } else {
                        Function::E
                            .pow(
                                *terms.1.clone()
                                    * Function::Ln {
                                        argument: terms.0.clone(),
                                    },
                            )
                            .derivative()
                    }
                }
                FunctionOperator::Comp => panic!("Something went wrong while parsing"),
            },
        }
    }

    /// Return the definite integral of the function between a and b
    /// ```
    /// use std::str::FromStr;
    /// use math_functions::functions::{Function, approx};
    /// let f1 = Function::from_str("x^2").unwrap();
    /// assert_eq!(approx(f1.integrate(0.0,1.0), 5), 0.33333);
    /// ```
    pub fn integrate(&self, a: f64, b: f64) -> f64 {
        let mut result = 0.;
        let number_of_steps = ((b - a) / 0.00002) as i32;

        for i in 1..=number_of_steps {
            // Evaluating function at midpoint of dx
            result += self.eval(a + ((b - a) / number_of_steps as f64) * (i as f64 - 0.5));
        }

        ((b - a) / number_of_steps as f64) * result
    }
}

/// Approximates an f64 to the nth decimal digit
pub fn approx(num: f64, digits: u32) -> f64 {
    let y = 10i32.pow(digits) as f64;
    (num * y).round() / y
}

impl FromStr for Function {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(split(s)?, &Context::new())
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::E => write!(f, "e"),
            Self::PI => write!(f, "𝜋"),
            Self::Var => write!(f, "x"),
            Self::Num(val) => write!(f, "{val}"),
            Self::Ln { argument } => write!(f, "ln({argument})"),
            Self::Sin { argument } => write!(f, "sin({argument})"),
            Self::Cos { argument } => write!(f, "cos({argument})"),
            Self::Tan { argument } => write!(f, "tan({argument})"),
            Self::Cot { argument } => write!(f, "cot({argument})"),
            Self::Sec { argument } => write!(f, "sec({argument})"),
            Self::Csc { argument } => write!(f, "csc({argument})"),
            Self::ACos { argument } => write!(f, "acos({argument})"),
            Self::ASin { argument } => write!(f, "asin({argument})"),
            Self::ATan { argument } => write!(f, "atan({argument})"),
            Self::Sinh { argument } => write!(f, "sinh({argument})"),
            Self::Cosh { argument } => write!(f, "cosh({argument})"),
            Self::Tanh { argument } => write!(f, "tanh({argument})"),
            Self::Coth { argument } => write!(f, "coth({argument})"),
            Self::Sech { argument } => write!(f, "sech({argument})"),
            Self::Csch { argument } => write!(f, "csch({argument})"),
            Self::ASinh { argument } => write!(f, "asinh({argument})"),
            Self::ACosh { argument } => write!(f, "acosh({argument})"),
            Self::ATanh { argument } => write!(f, "atanh({argument})"),
            Self::Abs { argument } => write!(f, "|{argument}|"),
            Self::Comp { terms, operator } => {
                let operator = match operator {
                    FunctionOperator::Add => '+',
                    FunctionOperator::Sub => '-',
                    FunctionOperator::Mul => '*',
                    FunctionOperator::Div => '/',
                    FunctionOperator::Pow => '^',
                    FunctionOperator::Comp => panic!("Something went wrong"),
                };

                write!(f, "({}{}{})", terms.0, operator, terms.1)
            }
        }
    }
}
