use crate::functions::{Function, FunctionOperator};
use std::ops::{Add, Div, Mul, Sub};

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

        Function::Comp {
            terms: (Box::new(self), Box::new(rhs)),
            operator: FunctionOperator::Add,
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

        Function::Comp {
            terms: (Box::new(self), Box::new(rhs)),
            operator: FunctionOperator::Sub,
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

            if let Function::Comp { terms, operator } = &rhs {
                if let FunctionOperator::Mul = operator {
                    if let Function::Num(val_2) = *terms.0 {
                        return Function::Comp {
                            terms: (
                                Box::new(Function::Num(val * val_2)),
                                Box::new(*terms.1.clone()),
                            ),
                            operator: FunctionOperator::Mul,
                        };
                    }
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

        if let Function::Comp {
            terms,
            operator: FunctionOperator::Div,
        } = &rhs
        {
            if *terms.1 == self {
                return *terms.0.clone();
            }
        }

        if let Function::Comp {
            terms,
            operator: FunctionOperator::Div,
        } = &self
        {
            if *terms.1 == rhs {
                return *terms.0.clone();
            }
        }

        Function::Comp {
            terms: (Box::new(self), Box::new(rhs)),
            operator: FunctionOperator::Mul,
        }
    }
}

impl Div for Function {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if self == rhs {
            return Function::Num(1.);
        }

        Function::Comp {
            terms: (Box::new(self), Box::new(rhs)),
            operator: FunctionOperator::Div,
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
