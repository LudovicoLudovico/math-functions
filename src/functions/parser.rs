use crate::functions::context::Context;
use crate::functions::splitter::{split, ParsingError, Split};
use crate::functions::{Function, FunctionOperator};

pub fn parse(input: Split, ctx: &Context) -> Result<Function, ParsingError> {
    let second_operand = input.second_operand;
    let first_operand = input.first_operand;

    if let Some(second_operand) = second_operand {
        if let FunctionOperator::Comp = input.operator {
            construct_composite(first_operand, second_operand, ctx)
        } else {
            let f = parse(split(first_operand)?, ctx)?;
            let g = parse(split(second_operand)?, ctx)?;
            match input.operator {
                FunctionOperator::Add => Ok(f + g),
                FunctionOperator::Sub => Ok(f - g),
                FunctionOperator::Mul => Ok(f * g),
                FunctionOperator::Div => Ok(f / g),
                FunctionOperator::Pow => Ok(f.pow(g)),
                FunctionOperator::Comp => Err(ParsingError::InvalidInput),
            }
        }
    } else {
        if let FunctionOperator::Sub = input.operator {
            return Ok(-1. * parse(split(first_operand)?, ctx)?);
        }

        match first_operand {
            "x" => Ok(Function::Var),
            "e" => Ok(Function::E),
            "pi" => Ok(Function::PI),
            _ => {
                if first_operand.parse::<f64>().is_ok() {
                    return Ok(Function::Num(first_operand.parse::<f64>().unwrap()));
                } else if let Some(value) = ctx.get_symbol(first_operand) {
                    return Ok(Function::Num(*value));
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
) -> Result<Function, ParsingError> {
    let argument = Box::new(parse(split(second_operand)?, ctx)?);

    match first_operand {
        "sin" => Ok(Function::Sin { argument }),
        "cos" => Ok(Function::Cos { argument }),
        "ln" => Ok(Function::Ln { argument }),
        "tan" => Ok(Function::Tan { argument }),
        _ => {
            if let Some(func) = ctx.get_func(first_operand) {
                return Ok((*func).clone());
            }
            Err(ParsingError::UnknownToken(first_operand.to_string()))
        }
    }
}
