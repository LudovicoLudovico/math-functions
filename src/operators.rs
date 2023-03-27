use super::{Function, Operation, F1D, F2D, F3D};
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
        $(impl $t {
            pub fn powf(self, exp: f64) -> Self {
                Self(
                    Function::Binary {
                        operation: Operation::Pow,
                        terms: (Box::new(self.0), Box::new(Function::Num(exp)))
                    }
                )
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
                    return Function::Binary {
                        terms: (
                            Box::new(Function::Num(val * val_2)),
                            Box::new(*terms.1.clone()),
                        ),
                        operation: Operation::Mul,
                    };
                }
            }
        }
        if let Function::Num(val) = rhs {
            if val == 0. {
                return Function::Num(0.);
            }
            if val == 1. {
                return self;
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

        Function::Binary {
            terms: (Box::new(self), Box::new(Function::Num(rhs))),
            operation: Operation::Pow,
        }
    }
}
