use crate::algebra::rational::Rational;
use crate::FunctionType;

use super::{Function, Operation, F1D, F2D, F3D};
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

macro_rules! impl_ops{
    (for $($t:ty),+) => {
        $(impl Add for $t {
            type Output = Self;
            fn add(self, rhs: Self) -> Self
            {
                Self(self.0 + rhs.0)
            }
        })*
        $(impl Sub for $t {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self
            {
                Self(self.0 - rhs.0)
            }
        })*
        $(impl Mul for $t {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self
            {
                Self(self.0 * rhs.0)
            }
        })*
        $(impl Div for $t {
            type Output = Self;
            fn div(self, rhs: Self) -> Self
            {
                Self(self.0 / rhs.0)
            }
        })*
        $(impl Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        })*
        $(impl $t {
            /// Raise function to a f64
            pub fn powf(self, exp: f64) -> Self {
                Self(self.0.powf(exp))
            }
        })*

    }
}

impl_ops!(for F1D, F2D, F3D);

impl Add for Function {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if let Function::Num(val) = self {
            if val == 0. {
                return rhs;
            }

            if let Function::Num(rhs_val) = rhs {
                return Function::Num(val + rhs_val);
            }
        }

        if let Function::Num(val) = rhs {
            if val == 0. {
                return self;
            }

            if let Function::Num(self_val) = self {
                return Function::Num(val + self_val);
            }
        }

        if self == rhs {
            return 2. * self;
        }

        if let Function::Binary {
            operation: Operation::Add,
            terms,
        } = &self
        {
            if *terms.0 == rhs {
                return 2. * *terms.0.clone() + *terms.1.clone();
            } else if *terms.1 == rhs {
                return 2. * *terms.1.clone() + *terms.0.clone();
            }
        }

        if let Function::Binary {
            operation: Operation::Sub,
            terms,
        } = &self
        {
            if *terms.0 == rhs {
                return 2. * *terms.0.clone() - *terms.1.clone();
            } else if *terms.1 == rhs {
                return *terms.0.clone();
            }
        }

        if let Function::Binary {
            operation: Operation::Mul,
            terms,
        } = &self
        {
            if let Function::Num(coefficient) = *terms.0 {
                if *terms.1 == rhs {
                    return (coefficient + 1.) * rhs;
                }
            }
        }

        Function::Binary {
            terms: (Box::new(self), Box::new(rhs)),
            operation: Operation::Add,
        }
    }
}

impl Sub for Function {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if let Function::Num(val) = self {
            if let Function::Num(rhs_val) = rhs {
                return Function::Num(val - rhs_val);
            }
        }

        if let Function::Num(val) = rhs {
            if val == 0. {
                return self;
            }

            if let Function::Num(self_val) = rhs {
                return Function::Num(val - self_val);
            }
        }

        if self == rhs {
            return Function::Num(0.);
        }

        if let Function::Num(first) = &self {
            if first == &0. {
                return -1. * rhs;
            }
        }
        if let Function::Binary {
            operation: Operation::Add,
            terms,
        } = &self
        {
            if *terms.0 == rhs {
                return *terms.1.clone();
            } else if *terms.1 == rhs {
                return *terms.0.clone();
            }
        }

        if let Function::Binary {
            operation: Operation::Sub,
            terms,
        } = &self
        {
            if *terms.0 == rhs {
                return -1. * *terms.1.clone();
            } else if *terms.1 == rhs {
                return *terms.0.clone() - 2. * rhs;
            }
        }

        Function::Binary {
            terms: (Box::new(self), Box::new(rhs)),
            operation: Operation::Sub,
        }
    }
}
impl Sub<f64> for Function {
    type Output = Function;

    fn sub(self, rhs: f64) -> Self::Output {
        self - Function::Num(rhs)
    }
}

impl Mul for Function {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self == rhs {
            return self.powf(2.);
        }

        if let &Function::Num(val) = &self {
            if val == 0. {
                return Function::Num(0.);
            }
            if val == 1. {
                return rhs;
            }

            if let Function::Binary {
                terms,
                operation: Operation::Mul,
            } = &rhs
            {
                if let Function::Num(val_2) = *terms.0 {
                    return (val * val_2) * *terms.1.clone();
                }
            }

            if let Function::Num(val_2) = &rhs {
                return Function::Num(val * val_2);
            }
        }

        if let Function::Binary {
            terms,
            operation: Operation::Div,
        } = &rhs
        {
            if *terms.1 == self {
                return *terms.0.clone();
            }
        }

        if let Function::Binary {
            terms,
            operation: Operation::Div,
        } = &self
        {
            if *terms.1 == rhs {
                return *terms.0.clone();
            }
        }

        if let Function::Num(val) = rhs {
            if val == 0. {
                return Function::Num(0.);
            }
            if val == 1. {
                return self;
            }
            if let Function::Binary {
                terms,
                operation: Operation::Mul,
            } = &self
            {
                if let Function::Num(val_2) = *terms.0 {
                    return (val * val_2) * *terms.1.clone();
                }
            }

            if let Function::Num(val_2) = &self {
                return Function::Num(val * val_2);
            }
        }

        if let Function::Binary {
            operation: Operation::Div,
            terms,
        } = &rhs
        {
            return self * *terms.0.clone() / *terms.1.clone();
        }
        if let Function::Binary {
            operation: Operation::Mul,
            terms,
        } = &rhs
        {
            if let Function::Num(val) = *terms.0 {
                return val * (self * *terms.1.clone());
            }
        }

        if let Function::Binary {
            operation: _,
            terms,
        } = &self
        {
            if let Function::Num(val) = *terms.0 {
                return val * (*terms.1.clone() * rhs);
            }
        }

        if let Function::Binary {
            operation: Operation::Pow,
            terms,
        } = &self
        {
            if let Function::Num(exponent) = *terms.1 {
                if *terms.0 == rhs {
                    return terms.0.clone().powf(exponent + 1.);
                }

                let base = &terms.0;
                if let Function::Binary {
                    operation: Operation::Pow,
                    terms,
                } = &rhs
                {
                    if let Function::Num(exp_2) = *terms.1 {
                        if base == &terms.0 {
                            return terms.0.clone().powf(exponent + exp_2);
                        }
                    }
                }
            }
        }
        Function::Binary {
            terms: (Box::new(self), Box::new(rhs)),
            operation: Operation::Mul,
        }
    }
}

impl Div for Function {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if self == rhs {
            return Function::Num(1.);
        }

        if let Function::Num(val) = rhs {
            if val == 1. {
                return self;
            }
        }
        if let Function::Num(val1) = self {
            if val1 == 0. {
                return Function::Num(0.);
            }
            if let Function::Num(val2) = rhs {
                if val1.fract() == 0.0 && val2.fract() == 0.0 {
                    return Function::Rational {
                        val: Rational::new(val1 as i32, val2 as i32),
                    };
                }

                return Function::Num(val1 / val2);
            }
        }

        Function::Binary {
            terms: (Box::new(self), Box::new(rhs)),
            operation: Operation::Div,
        }
    }
}

impl Add<Function> for f64 {
    type Output = Function;

    fn add(self, rhs: Function) -> Self::Output {
        Function::Num(self) + rhs
    }
}
impl Sub<Function> for f64 {
    type Output = Function;

    fn sub(self, rhs: Function) -> Self::Output {
        Function::Num(self) - rhs
    }
}

impl Mul<Function> for f64 {
    type Output = Function;

    fn mul(self, rhs: Function) -> Self::Output {
        Function::Num(self) * rhs
    }
}

impl Function {
    pub fn pow(self, rhs: Self) -> Self {
        if let Function::Num(val_1) = &self {
            if let Function::Num(val_2) = &rhs {
                return Function::Num(val_1.powf(*val_2));
            }
        }

        if let Function::E = &self {
            if let Function::Special {
                kind: FunctionType::Ln,
                argument,
            } = &rhs
            {
                return *argument.clone();
            }

            if let Function::Binary {
                operation: Operation::Mul,
                terms,
            } = &rhs
            {
                if let Function::Num(val) = *terms.0 {
                    if let Function::Special {
                        kind: FunctionType::Ln,
                        argument,
                    } = &*terms.1
                    {
                        return argument.clone().powf(val);
                    }
                }
            }
        }

        if let Function::Binary {
            operation: Operation::Pow,
            terms,
        } = &self
        {
            let base = &terms.0;

            if let Function::Num(first_exp) = *terms.1 {
                if let Function::Num(second_exp) = &rhs {
                    return base.clone().powf(first_exp * second_exp);
                }
            }
            if let Function::Rational { val } = &*terms.1 {
                if let Function::Rational { val: val2 } = &rhs {
                    return base.clone().powf(*val2 * *val);
                }
            }
        }

        Function::Binary {
            terms: (Box::new(self), Box::new(rhs)),
            operation: Operation::Pow,
        }
    }
    pub fn powf(self, rhs: f64) -> Self {
        if rhs == 1. {
            return self;
        }

        if rhs == 0. {
            return Function::Num(1.);
        }

        if let Function::Binary {
            operation: Operation::Pow,
            terms,
        } = &self
        {
            let base = &terms.0;

            if let Function::Num(first_exp) = *terms.1 {
                return base.clone().powf(first_exp * rhs);
            }
        }

        Function::Binary {
            terms: (Box::new(self), Box::new(Function::Num(rhs))),
            operation: Operation::Pow,
        }
    }
}
