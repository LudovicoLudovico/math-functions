use crate::functions::Function;
use std::collections::HashMap;

/// Context holds two hash tables:
/// - functions (&str, &Function)
/// - symbols (&str, f64)
/// And allows new functions to use the symbols and functions defined in the Context
/// ```
/// use math_functions::functions::Function;
/// use math_functions::functions::Context;
/// use std::str::FromStr;
///
/// let f1 = Function::from_str("e^x").unwrap();
/// let mut ctx = Context::new();
/// ctx.add_func("EXP", &f1);
/// ctx.add_symbol("L", 0.45);
/// let f2 = Function::build("L*EXP(x)", &ctx).unwrap();
/// assert_eq!(f2, Function::Num(0.45) * Function::E.pow(Function::Var) );
/// ```
#[derive(Debug, Default)]
pub struct Context<'a> {
    functions: HashMap<&'a str, &'a Function>,
    symbols: HashMap<&'a str, f64>,
}

impl<'a> Context<'a> {
    /// Initialize and empty context
    pub fn new() -> Context<'a> {
        Context {
            functions: HashMap::new(),
            symbols: HashMap::new(),
        }
    }

    /// Adds a &Function to the context, with an associated name
    pub fn add_func(&mut self, name: &'a str, new: &'a Function) {
        self.functions.insert(name, new);
    }

    /// Retrieve a &&Function from the context
    pub fn get_func(&self, name: &'a str) -> Option<&&Function> {
        self.functions.get(name)
    }

    /// Adds a symbol to the context, with an associated name
    pub fn add_symbol(&mut self, name: &'a str, value: f64) {
        self.symbols.insert(name, value);
    }
    /// Retrieve a &f64 from the context
    pub fn get_symbol(&self, name: &'a str) -> Option<&f64> {
        self.symbols.get(name)
    }
}

