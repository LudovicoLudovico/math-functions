use super::{Function, FunctionType, Operation, F1D, F2D, F3D};

impl Function {
    fn evaluate(&self, inputs: &[f64]) -> f64 {
        match &self {
            Self::X => inputs[0],
            Self::Y => inputs[1],
            Self::Z => inputs[2],
            Self::Num(num) => *num,
            Self::E => std::f64::consts::E,
            Self::PI => std::f64::consts::PI,
            Self::Binary { operation, terms } => {
                let left = terms.0.evaluate(&inputs);
                let right = terms.1.evaluate(&inputs);
                eval_ops(operation, left, right)
            }
            Self::Special { kind, argument } => {
                let argument = argument.evaluate(&inputs);
                eval_trascendental(kind, argument)
            }
        }
    }

    fn derivative(&self, on_x: f64, on_y: f64, on_z: f64) -> Self {
        match self {
            Self::X => Self::Num(on_x),
            Self::Y => Self::Num(on_y),
            Self::Z => Self::Num(on_z),
            Self::Num(_) => Self::Num(0.),
            Self::E | Self::PI => Self::Num(0.),
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
                        / (terms.1.clone().powf(2.))
                }
                Operation::Pow => {
                    if let Function::Num(val) = *terms.1 {
                        if on_x != 0. {
                            if let Function::X = *terms.0 {
                                return val * Function::X.powf(val - 1.);
                            }
                        } else if on_y != 0. {
                            if let Function::Y = *terms.0 {
                                return val * Function::Y.powf(val - 1.);
                            }
                        } else {
                            if let Function::Z = *terms.0 {
                                return val * Function::Z.powf(val - 1.);
                            }
                        };

                        terms.0.derivative(on_x, on_y, on_z)
                            * (val * terms.1.clone().powf(val - 1.))
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
                        -1. * arg
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
                        .powf(2.)
                    }
                    FunctionType::Cot => {
                        -1. * arg
                            / (Function::Special {
                                kind: FunctionType::Sin,
                                argument,
                            })
                            .powf(2.)
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
                        -1. * arg
                            * Function::Special {
                                kind: FunctionType::Cot,
                                argument: argument.clone(),
                            }
                            * Function::Special {
                                kind: FunctionType::Csc,
                                argument: argument.clone(),
                            }
                    }
                    FunctionType::ASin => arg / (1. - argument.powf(2.)).powf(0.5),
                    FunctionType::ACos => -1. * arg / (1. - argument.powf(2.)).powf(0.5),
                    FunctionType::ATan => arg / (1. + argument.powf(2.)),
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
                        .powf(2.)
                    }
                    FunctionType::Coth => {
                        -1. * arg
                            * Function::Special {
                                kind: FunctionType::Csch,
                                argument,
                            }
                            .powf(2.)
                    }
                    FunctionType::Sech => {
                        -1. * arg
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
                        -1. * arg
                            * Function::Special {
                                kind: FunctionType::Csch,
                                argument: argument.clone(),
                            }
                            * Function::Special {
                                kind: FunctionType::Coth,
                                argument: argument.clone(),
                            }
                    }
                    FunctionType::ASinh => arg / (1. + argument.powf(2.)).powf(0.5),
                    FunctionType::ACosh => arg * (argument.powf(2.) - 1.).powf(0.5),
                    FunctionType::ATanh => arg / (1. - argument.powf(2.)),
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
    pub fn eval(&self, x: f64) -> f64 {
        self.0.evaluate(&[x, 0., 0.])
    }

    pub fn derivative(&self) -> Self {
        F1D(self.0.derivative(1., 0., 0.))
    }
    pub fn add(self, rhs: Self) -> Self {
        self + rhs
    }
}

impl F2D {
    pub fn eval(&self, x: f64, y: f64) -> f64 {
        self.0.evaluate(&[x, y, 0.])
    }

    pub fn derivative(&self) -> (Self, Self) {
        (
            F2D(self.0.derivative(1., 0., 0.)),
            F2D(self.0.derivative(0., 1., 0.)),
        )
    }
}
impl F3D {
    pub fn eval(&self, x: f64, y: f64, z: f64) -> f64 {
        self.0.evaluate(&[x, y, z])
    }

    pub fn derivative(&self) -> (Self, Self, Self) {
        (
            F3D(self.0.derivative(1., 0., 0.)),
            F3D(self.0.derivative(0., 1., 0.)),
            F3D(self.0.derivative(0., 0., 1.)),
        )
    }
}

fn eval_trascendental(kind: &FunctionType, arg: f64) -> f64 {
    match kind {
        FunctionType::Sin => arg.sin(),
        FunctionType::Cos => arg.cos(),
        _ => todo!(),
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
