use super::splitter::{split, ParsingError, Split};
use super::{Function, FunctionType, Operation};

pub fn parse(input: Split, allow_y: bool, allow_z: bool) -> Result<Function, ParsingError> {
    let second_operand = input.second_operand;
    let first_operand = input.first_operand;

    if let Some(second_operand) = second_operand {
        if let Operation::Comp = input.operator {
            construct_composite(first_operand, second_operand, allow_y, allow_z)
        } else {
            let f = parse(split(first_operand)?, allow_y, allow_z)?;
            let g = parse(split(second_operand)?, allow_y, allow_z)?;
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
            return Ok(-1. * parse(split(first_operand)?, allow_y, allow_z)?);
        }

        match first_operand {
            "x" => Ok(Function::X),
            "e" => Ok(Function::E),
            "pi" => Ok(Function::PI),
            "y" => {
                if allow_y {
                    Ok(Function::Y)
                } else {
                    Err(ParsingError::UnknownToken("y".to_string()))
                }
            }
            "z" => {
                if allow_z {
                    Ok(Function::Z)
                } else {
                    Err(ParsingError::UnknownToken("z".to_string()))
                }
            }
            _ => {
                if first_operand.parse::<f64>().is_ok() {
                    return Ok(Function::Num(first_operand.parse::<f64>().unwrap()));
                }
                // else if let Some(value) = ctx.get_symbol(first_operand) {
                //     return Ok(Function::Num(*value));
                // }

                Err(ParsingError::UnknownToken(first_operand.to_string()))
            }
        }
    }
}

fn construct_composite(
    first_operand: &str,
    second_operand: &str,
    allow_y: bool,
    allow_z: bool,
) -> Result<Function, ParsingError> {
    let argument = Box::new(parse(split(second_operand)?, allow_y, allow_z)?);

    match first_operand {
        "sin" => Ok(Function::Special {
            kind: FunctionType::Sin,
            argument,
        }),

        "cos" => Ok(Function::Special {
            kind: FunctionType::Cos,
            argument,
        }),
        "ln" => Ok(Function::Special {
            kind: FunctionType::Ln,
            argument,
        }),
        "tan" => Ok(Function::Special {
            kind: FunctionType::Tan,
            argument,
        }),
        // "cos" => Ok(Function::Cos { argument }),
        // "ln" => Ok(Function::Ln { argument }),
        // "tan" => Ok(Function::Tan { argument }),
        _ => {
            // if let Some(func) = ctx.get_func(first_operand) {
            //     return Ok((*func).clone());
            // }
            Err(ParsingError::UnknownToken(first_operand.to_string()))
        }
    }
}
