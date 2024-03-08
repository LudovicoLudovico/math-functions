use super::splitter::{split, ParsingError, Split};
use crate::algebra::rational::Rational;
use crate::context::Context;
use crate::{Function, FunctionType, Operation};

pub(crate) fn parse(input: Split, ctx: &Context, dim: usize) -> Result<Function, ParsingError> {
    let second_operand = input.second_operand;
    let first_operand = input.first_operand;

    if let Some(second_operand) = second_operand {
        if let Operation::Comp = input.operator {
            let res = construct_composite(first_operand, second_operand, ctx, dim);

            match res {
                Ok(_) => res,
                Err(err) => {
                    if let Some(func) = ctx.get_func(first_operand) {
                        Ok((*func.0).clone())
                    } else {
                        Err(err)
                    }
                }
            }
        } else {
            let f = parse(split(first_operand)?, ctx, dim)?;
            let g = parse(split(second_operand)?, ctx, dim)?;
            match input.operator {
                Operation::Add => Ok(f + g),
                Operation::Sub => Ok(f - g),
                Operation::Mul => Ok(f * g),
                Operation::Div => Ok(f / g),
                Operation::Pow => Ok(f.pow(g)),
                Operation::Comp => Err(ParsingError::InvalidInput),
            }
        }
    } else {
        if let Operation::Sub = input.operator {
            return Ok(-1 * parse(split(first_operand)?, ctx, dim)?);
        }

        match first_operand {
            "x" => Ok(Function::X),
            "e" => Ok(Function::E),
            "pi" => Ok(Function::PI),
            "y" => {
                if dim >= 2 {
                    Ok(Function::Y)
                } else {
                    Err(ParsingError::UnknownToken("y".to_string()))
                }
            }
            "z" => {
                if dim == 3 {
                    Ok(Function::Z)
                } else {
                    Err(ParsingError::UnknownToken("z".to_string()))
                }
            }
            _ => {
                if first_operand.parse::<i32>().is_ok() {
                    return Ok(Function::Rational(Rational::new_from_int(
                        first_operand.parse::<i32>().unwrap(),
                    )));
                }
                // if let Some(symbol) = ctx.get_symbol(first_operand) {
                //     return Ok(Function::Num(*symbol));
                // }
                if let Some(func) = ctx.get_func(first_operand) {
                    if func.1 <= dim {
                        return Ok((*func.0).clone());
                    }
                    return Err(ParsingError::CantUseHigherDimensionsFunc);
                }

                Err(ParsingError::UnknownToken(first_operand.to_string()))
            }
        }
    }
}

fn construct_composite(
    first_operand: &str,
    second_operand: &str,
    ctx: &Context,
    dim: usize,
) -> Result<Function, ParsingError> {
    let argument = Box::new(parse(split(second_operand)?, ctx, dim)?);

    let func_type = match_str_type(first_operand);

    if let Ok(val) = func_type {
        Ok(Function::Special {
            kind: val,
            argument,
        })
    } else {
        Err(ParsingError::UnknownToken(first_operand.to_string()))
    }
}

fn match_str_type(input: &str) -> Result<FunctionType, ParsingError> {
    match input {
        "sin" => Ok(FunctionType::Sin),
        "cos" => Ok(FunctionType::Cos),
        "tan" => Ok(FunctionType::Tan),
        "cot" => Ok(FunctionType::Cot),
        "sec" => Ok(FunctionType::Sec),
        "csc" => Ok(FunctionType::Csc),
        "asin" => Ok(FunctionType::ASin),
        "acos" => Ok(FunctionType::ACos),
        "atan" => Ok(FunctionType::ATan),
        "sinh" => Ok(FunctionType::Sinh),
        "cosh" => Ok(FunctionType::Cosh),
        "tanh" => Ok(FunctionType::Tanh),
        "coth" => Ok(FunctionType::Coth),
        "sech" => Ok(FunctionType::Sech),
        "csch" => Ok(FunctionType::Csch),
        "asinh" => Ok(FunctionType::ASinh),
        "acosh" => Ok(FunctionType::ACosh),
        "atanh" => Ok(FunctionType::ATanh),
        "abs" => Ok(FunctionType::Abs),
        "ln" => Ok(FunctionType::Ln),
        _ => Err(ParsingError::UnknownToken(input.to_string())),
    }
}

#[test]
fn test_parser() {
    use crate::F1D;
    use std::str::FromStr;

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
                Box::new(2 * Function::X.powr(Rational::new_from_int(3)))
            ),
            operation: Operation::Add
        })
    );

    assert_eq!(
        F1D::from_str("-x^2").unwrap(),
        F1D(-1 * Function::X.powr(Rational::new_from_int(2)))
    );

    assert_eq!(
        F1D::from_str("e^(x^2)").unwrap(),
        F1D(Function::E.pow(Function::X.powr(Rational::new_from_int(2))))
    );

    assert_eq!(F1D::from_str("3+x").unwrap(), F1D(3 + Function::X));

    assert_eq!(
        F1D::from_str("x^x").unwrap(),
        F1D(Function::X.pow(Function::X))
    );
}
