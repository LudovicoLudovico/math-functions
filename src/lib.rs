#![deny(warnings, missing_docs)]
//! Crate for math functions

/// Contains context
pub mod context;
/// Operations
pub mod operations;
/// Operators
pub mod operators;
mod parser;
/// Polynomials
pub mod polynomials;
mod splitter;
mod tests;
use context::Context;
/// Matrix
pub mod matrix;
pub use matrix::{Matrix, Vec2, Vec3};
use parser::parse;
use splitter::{split, ParsingError};
use std::str::FromStr;

/// Representation of a Function
#[derive(Clone, PartialEq, Debug)]
pub(crate) enum Function {
    /// X Variable
    X,
    /// Y Variable
    Y,
    /// Z Variable
    Z,
    /// Euler's number
    E,
    /// Pi
    PI,
    /// Generic numeric constant
    Num(f64),
    /// Represent a binary operation between two functions
    Binary {
        /// Operation between the two functions
        operation: Operation,
        /// Two operands
        terms: (Box<Self>, Box<Self>),
    },
    /// Represent a special function such as sin, cos... with its argument
    Special {
        /// Built-in funciton types
        kind: FunctionType,
        /// Argument of the function (sin(4x), 4x is the argument)
        argument: Box<Self>,
    },
}

#[derive(Debug, PartialEq)]
/// Representation of a function with 1 variable
pub struct F1D(Function);
#[derive(Debug, PartialEq)]
/// Representation of a function with 2 variables
pub struct F2D(Function);
#[derive(PartialEq, Debug)]
/// Representation of a function with 3 variables
pub struct F3D(Function);

#[derive(Clone, PartialEq, Debug)]
/// Types of special built-in functions
pub(crate) enum FunctionType {
    /// Sine
    Sin,
    /// Cosine
    Cos,
    /// Tangent
    Tan,
    /// Cotangent
    Cot,
    /// Secant
    Sec,
    /// Cosecant
    Csc,
    /// Inverse of sine
    ASin,
    /// Inverse of cosine
    ACos,
    /// Inverse of tangent
    ATan,
    /// Hyperbolic sine
    Sinh,
    /// Hyperbolic cosine
    Cosh,
    /// Hyperbolic tangent
    Tanh,
    /// Hyperbolic cotangent
    Coth,
    /// Hyperbolic secant
    Sech,
    /// Hyperbolic cosecant
    Csch,
    /// Inverse of hyperbolic sine
    ASinh,
    /// Inverse of hyperbolic cosine
    ACosh,
    /// Inverse of hyperbolic tangent
    ATanh,
    /// Absolute value
    Abs,
    /// Natural logarithm
    Ln,
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Comp,
}

impl Operation {
    fn priority(&self) -> u8 {
        match self {
            Operation::Add => 4,
            Operation::Sub => 4,
            Operation::Mul => 3,
            Operation::Div => 3,
            Operation::Pow => 2,
            Operation::Comp => 1,
        }
    }
}

/// Function to approximate f64 to the nth decimal place
pub fn approx(num: f64, digits: u32) -> f64 {
    let y = 10i32.pow(digits) as f64;
    (num * y).round() / y
}

impl FromStr for F1D {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse(split(s)?, &Context::new(), 1) {
            Ok(val) => Ok(F1D(val)),
            Err(err) => Err(err),
        }
    }
}
impl FromStr for F2D {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse(split(s)?, &Context::new(), 2) {
            Ok(val) => Ok(F2D(val)),
            Err(err) => Err(err),
        }
    }
}
impl FromStr for F3D {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse(split(s)?, &Context::new(), 3) {
            Ok(val) => Ok(F3D(val)),
            Err(err) => Err(err),
        }
    }
}
