pub mod matrix;
pub mod operations;
pub mod operators;
mod parser;
mod splitter;
mod tests;
use parser::parse;
use splitter::{split, ParsingError};
use std::str::FromStr;

#[derive(Clone, PartialEq, Debug)]
pub enum Function {
    X,
    Y,
    Z,
    E,
    PI,
    Num(f64),
    Binary {
        operation: Operation,
        terms: (Box<Self>, Box<Self>),
    },
    Special {
        kind: FunctionType,
        argument: Box<Self>,
    },
}

#[derive(Debug, PartialEq)]
pub struct F1D(Function);
#[derive(Debug, PartialEq)]
pub struct F2D(Function);
#[derive(PartialEq, Debug)]
pub struct F3D(Function);

#[derive(Clone, PartialEq, Debug)]
pub enum FunctionType {
    Sin,
    Cos,
    Tan,
    Cot,
    Sec,
    Csc,
    ACos,
    ASin,
    ATan,
    Sinh,
    Cosh,
    Tanh,
    Coth,
    Sech,
    Csch,
    ASinh,
    ACosh,
    ATanh,
    Abs,
    Ln,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Operation {
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

pub fn approx(num: f64, digits: u32) -> f64 {
    let y = 10i32.pow(digits) as f64;
    (num * y).round() / y
}

impl FromStr for F1D {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse(split(s)?, false, false) {
            Ok(val) => Ok(F1D(val)),
            Err(err) => Err(err),
        }
    }
}
impl FromStr for F2D {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse(split(s)?, true, false) {
            Ok(val) => Ok(F2D(val)),
            Err(err) => Err(err),
        }
    }
}
impl FromStr for F3D {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse(split(s)?, true, true) {
            Ok(val) => Ok(F3D(val)),
            Err(err) => Err(err),
        }
    }
}
