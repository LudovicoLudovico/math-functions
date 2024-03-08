use crate::Operation;
use std::error::Error;
use std::fmt::Display;

pub(crate) fn split(input: &str) -> Result<Split, ParsingError> {
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
            || is_close_par(current)
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

    if input.is_empty() || !matches!(&input[0..1], "(" | "[" | "{") {
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
pub(crate) struct Split<'a> {
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
    CantUseHigherDimensionsFunc,
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownToken(invalid_token) => write!(f, "Token: {invalid_token} is not valid"),
            Self::MismatchedParenthesis => write!(f, "Mismatched Parenthesis"),
            Self::EmptyInput => write!(f, "Input is empty"),
            Self::InvalidInput => write!(f, "Invalid input"),
            Self::CantUseHigherDimensionsFunc => write!(
                f,
                "Can't mix higer dimensions functions and lower dimensions"
            ),
        }
    }
}
impl Error for ParsingError {}

#[test]
fn test_tokenzier() {
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
