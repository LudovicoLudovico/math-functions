#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;
    use crate::splitter::{split, Split};
    use crate::{Function, FunctionType, Operation, F1D, F3D};
    use std::str::FromStr;

    impl<'a> Split<'a> {
        fn build(
            first_operand: &'a str,
            second_operand: Option<&'a str>,
            operation: char,
        ) -> Split<'a> {
            let operator = match operation {
                '+' => Operation::Add,
                '-' => Operation::Sub,
                '*' => Operation::Mul,
                '/' => Operation::Div,
                '^' => Operation::Pow,
                '(' => Operation::Comp,
                _ => panic!("Called update with invalid operation"),
            };

            Split {
                first_operand,
                second_operand,
                operator,
            }
        }
    }

    #[test]
    fn test_tokenzier() {
        assert_eq!(split("x^0.5").unwrap(), Split::build("x", Some("0.5"), '^'));
        assert_eq!(split("3x").unwrap(), Split::build("3", Some("x"), '*'));
        assert_eq!(
            split("2.45x^0.5").unwrap(),
            Split::build("2.45", Some("x^0.5"), '*')
        );
        assert_eq!(
            split("2cos(x)^2").unwrap(),
            Split::build("2", Some("cos(x)^2"), '*')
        );
        assert_eq!(
            split("3x^2+e+7").unwrap(),
            Split::build("3x^2+e", Some("7"), '+')
        );
        assert_eq!(
            split("2.36*cos(4x^3)").unwrap(),
            Split::build("2.36", Some("cos(4x^3)"), '*')
        );
        assert_eq!(
            split("tan(cot(x))").unwrap(),
            Split::build("tan", Some("cot(x)"), '(')
        );
        assert_eq!(split("pix^2").unwrap(), Split::build("pix", Some("2"), '^'));
        assert_eq!(
            split("3.4e^9.2").unwrap(),
            Split::build("3.4", Some("e^9.2"), '*')
        );
        assert_eq!(
            split("3.4+cos(3x)").unwrap(),
            Split::build("3.4", Some("cos(3x)"), '+')
        );
        assert_eq!(
            split("cos(x)+sin(2x)").unwrap(),
            Split::build("cos(x)", Some("sin(2x)"), '+')
        );
        assert_eq!(
            split("cos(x)-sin(2x)").unwrap(),
            Split::build("cos(x)", Some("sin(2x)"), '-')
        );
        assert_eq!(
            split("cos(x)").unwrap(),
            Split::build("cos", Some("x"), '(')
        );
        assert_eq!(
            split("cos(3x)+3.4").unwrap(),
            Split::build("cos(3x)", Some("3.4"), '+')
        );
        assert_eq!(
            split("cos(3x)^2.3").unwrap(),
            Split::build("cos(3x)", Some("2.3"), '^')
        );
        assert_eq!(split("-x^2").unwrap(), Split::build("x^2", None, '-'));
        assert_eq!(
            split("sqrt(4)*cos(3x+5)").unwrap(),
            Split::build("sqrt(4)", Some("cos(3x+5)"), '*')
        );
        assert_eq!(
            split("e^(x^2)").unwrap(),
            Split::build("e", Some("x^2"), '^')
        );
        assert_eq!(split("x").unwrap(), Split::build("x", None, '('));
        assert_eq!(split("e").unwrap(), Split::build("e", None, '('));
        assert_eq!(
            split("cot(x)/2x").unwrap(),
            Split::build("cot(x)/2", Some("x"), '*')
        );
        assert_eq!(
            split("cot(x)/(2x)").unwrap(),
            Split::build("cot(x)", Some("2x"), '/')
        );
        assert_eq!(
            split("cot(x)^(2x)").unwrap(),
            Split::build("cot(x)", Some("2x"), '^')
        );
        assert_eq!(
            split("asin(x/4)").unwrap(),
            Split::build("asin", Some("x/4"), '(')
        );
        assert_eq!(
            split("(x+2)(x-2)").unwrap(),
            Split::build("x+2", Some("x-2"), '*'),
        );
        assert_eq!(
            split("sin(x)cos(x)").unwrap(),
            Split::build("sin(x)", Some("cos(x)"), '*')
        );
        assert_eq!(
            split("2cos(x)").unwrap(),
            Split::build("2", Some("cos(x)"), '*')
        );
        assert_eq!(
            split("sin(x)cos(x)").unwrap(),
            Split::build("sin(x)", Some("cos(x)"), '*')
        );
        assert_eq!(
            split("sin(x)").unwrap(),
            Split::build("sin", Some("x"), '(')
        );
        assert_eq!(split("sin").unwrap(), Split::build("sin", None, '('));
        assert_eq!(split("4^x").unwrap(), Split::build("4", Some("x"), '^'));
        assert_eq!(split("x/4").unwrap(), Split::build("x", Some("4"), '/'));
        assert_eq!(split("x^x").unwrap(), Split::build("x", Some("x"), '^'));
        assert_eq!(split("3x+7").unwrap(), Split::build("3x", Some("7"), '+'));
        assert_eq!(split("3x").unwrap(), Split::build("3", Some("x"), '*'));
        assert_eq!(
            split("(log(x)+1)*e^(xlog(x))").unwrap(),
            Split::build("log(x)+1", Some("e^(xlog(x))"), '*')
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            F1D::from_str("cos(x)").unwrap(),
            F1D(Function::Special {
                kind: FunctionType::Cos,
                argument: Box::new(Function::X)
            })
        );

        assert_eq!(
            F1D::from_str("cos(x)+2x^3").unwrap(),
            F1D(Function::Binary {
                terms: (
                    Box::new(Function::Special {
                        kind: FunctionType::Cos,
                        argument: Box::new(Function::X)
                    }),
                    Box::new(2. * Function::X.powf(3.))
                ),
                operation: Operation::Add
            })
        );

        assert_eq!(
            F1D::from_str("-x^2").unwrap(),
            F1D(-1. * (Function::X.powf(2.)))
        );

        assert_eq!(
            F1D::from_str("e^(x^2)").unwrap(),
            F1D(Function::E.pow(Function::X.powf(2.)))
        );

        assert_eq!(F1D::from_str("3+x").unwrap(), F1D(3. + Function::X));

        assert_eq!(
            F1D::from_str("x^x").unwrap(),
            F1D(Function::X.pow(Function::X))
        );

        // let mut ctx = Context::default();
        // let my_func = Function::from_str("e^(x^2)").unwrap();
        // ctx.add_symbol("YT", 69.);
        // ctx.add_func("MYFUNC", &my_func);
        // assert_eq!(
        //     Function::build("YT*x+MYFUNC(x)", &ctx).unwrap(),
        //     Function::Num(69.) * Function::X + (Function::E.pow(Function::X.powf(2.)))
        // )
    }
    #[test]
    fn test_derivative() {
        let func = F1D::from_str("3x+7+e").unwrap();
        assert_eq!(func.derivative(), F1D(Function::Num(3.)));

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
            F1D((Function::Num(2.) * Function::X)
                * Function::Special {
                    kind: FunctionType::Sec,
                    argument: Box::new(Function::X.powf(2.))
                }
                .powf(2.))
        );

        let func = F1D::from_str("x^x").unwrap();
        assert_eq!(
            func.derivative(),
            F1D::from_str("(ln(x)+1)*e^(x*ln(x))").unwrap()
        );

        let func = F3D::from_str("xyz^2").unwrap();
        assert_eq!(
            func.derivative(),
            (
                F3D::from_str("yz^2").unwrap(),
                F3D::from_str("xz^2").unwrap(),
                F3D::from_str("xy(2*z)").unwrap()
            )
        )
    }

    #[test]
    fn test_hessian() {
        let func = F3D::from_str("3x^2+y^4+xyz^2").unwrap();
        let hessian = func.hessian();

        let result: Matrix<F3D> = Matrix {
            mat: vec![
                F3D::from_str("6").unwrap(),
                F3D::from_str("z^2").unwrap(),
                F3D::from_str("y(2z)").unwrap(),
                F3D::from_str("z^2").unwrap(),
                F3D::from_str("12y^2").unwrap(),
                F3D::from_str("x(2z)").unwrap(),
                F3D::from_str("y(2z)").unwrap(),
                F3D::from_str("x(2z)").unwrap(),
                F3D::from_str("xy(2)").unwrap(),
            ],
            n_col: 3,
            n_row: 3,
        };
        assert_eq!(result, hessian);
    }

    #[test]
    fn test_mat() {
        let mat = Matrix {
            mat: vec![1., 2., 3., 4., 5., 6., 7., 8., 9.],
            n_col: 3,
            n_row: 3,
        };

        assert_eq!(3., *mat.get(1, 3));

        assert_eq!(9., *mat.get(3, 3))
    }
    // #[test]
    // fn test_integration() {
    //     let func = F1D::from_str("x^3").unwrap();
    //     assert_eq!(approx(func.integrate(-1., 1.5), 4), 1.0156);
    //
    //     let func = Function::from_str("sin(x)^2").unwrap();
    //     assert_eq!(
    //         approx(func.integrate(0., 2. * std::f64::consts::PI), 8),
    //         3.14159265
    //     );
    // }Function
}
