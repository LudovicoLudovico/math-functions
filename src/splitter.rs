use super::Operation;
use std::error::Error;
use std::fmt::Display;

pub fn split(input: &str) -> Result<Split, ParsingError> {
    if input.is_empty() {
        return Err(ParsingError::EmptyInput);
    }

    let mut par_counter = 0;
    let mut split = Split::init(input);

    let mut chars = input.chars().enumerate().peekable();

    while let Some(char) = chars.next() {
        let index = char.0;
        let char = char.1;

        if is_close_par(char) {
            par_counter -= 1;
            if par_counter < 0 {
                return Err(ParsingError::MismatchedParenthesis);
            }
        }

        if par_counter == 0 {
            if is_open_par(char) {
                split.update(input, index, index + 1, '(');
            } else if is_operator(char) {
                split.update(input, index, index + 1, char);
            } else {
                try_implicit_mul(input, char, index, chars.peek(), &mut split);
            }
        }

        if is_open_par(char) {
            par_counter += 1;
        }
    }

    Ok(split)
}

fn try_implicit_mul<'a>(
    input: &'a str,
    current: char,
    index: usize,
    next: Option<&(usize, char)>,
    split: &mut Split<'a>,
) {
    if let Some(next) = next {
        if (current.is_numeric()
            || current == ')'
            || current == 'x'
            || current == 'y'
            || current == 'z')
            && (!next.1.is_numeric() && next.1 != '.' && !is_operator(next.1))
        {
            split.update(input, index + 1, index + 1, '*');
        }
    }
}

fn is_operator(char: char) -> bool {
    matches!(char, '+' | '-' | '*' | '/' | '^')
}

fn is_close_par(char: char) -> bool {
    matches!(char, ')' | ']' | '}')
}
fn is_open_par(char: char) -> bool {
    matches!(char, '(' | '[' | '{')
}
fn unwrap_par(input: &str) -> &str {
    let mut par_counter = 0;

    if input.is_empty() || !input[0..1].starts_with('(') {
        return input;
    }

    for (idx, char) in input.chars().enumerate() {
        if is_open_par(char) {
            par_counter += 1;
        } else if is_close_par(char) {
            par_counter -= 1;
        }

        if par_counter == 0 && idx != input.len() - 1 {
            return input;
        }
    }

    if input.len() > 1 {
        &input[1..input.len() - 1]
    } else {
        input
    }
}

#[derive(PartialEq, Debug)]
pub struct Split<'a> {
    pub first_operand: &'a str,
    pub second_operand: Option<&'a str>,
    pub operator: Operation,
}

impl<'a> Split<'a> {
    fn init(init: &'a str) -> Split<'a> {
        Split {
            first_operand: &init[0..init.len()],
            second_operand: None,
            operator: Operation::Comp,
        }
    }
    fn update(&mut self, input: &'a str, first_idx: usize, second_idx: usize, operator: char) {
        let end_offset = if operator == '(' { 1 } else { 0 };

        let new = match operator {
            '+' => Operation::Add,
            '-' => Operation::Sub,
            '*' => Operation::Mul,
            '/' => Operation::Div,
            '^' => Operation::Pow,
            '(' => Operation::Comp,
            _ => panic!("Called update with invalid operator"),
        };

        if new.priority() >= self.operator.priority() {
            if first_idx == 0 && operator == '-' {
                self.first_operand = unwrap_par(&input[1..input.len()]);
                self.second_operand = None;
                self.operator = new;
            } else {
                self.first_operand = unwrap_par(&input[0..first_idx]);
                self.second_operand =
                    Some(unwrap_par(&input[second_idx..input.len() - end_offset]));
                self.operator = new;
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParsingError {
    UnknownToken(String),
    MismatchedParenthesis,
    EmptyInput,
    InvalidInput,
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownToken(invalid_token) => write!(f, "Token: {invalid_token} is not valid"),
            Self::MismatchedParenthesis => write!(f, "Mismatched Parenthesis"),
            Self::EmptyInput => write!(f, "Input is empty"),
            Self::InvalidInput => write!(f, "Invalid input"),
        }
    }
}
impl Error for ParsingError {}
