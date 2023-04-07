use super::{Function, F1D, F2D, F3D};
use std::collections::HashMap;

#[derive(Debug, Default)]
/// Context for stroing functions and symbols
pub struct Context<'a> {
    functions: HashMap<&'a str, (&'a Function, usize)>,
    symbols: HashMap<&'a str, f64>,
}

impl<'a> Context<'a> {
    /// Create a new empty context
    pub fn new() -> Context<'a> {
        Context {
            functions: HashMap::new(),
            symbols: HashMap::new(),
        }
    }

    /// Add F1D to context
    pub fn add_f1d(&mut self, name: &'a str, new: &'a F1D) {
        self.functions.insert(name, (&new.0, 1));
    }
    /// Add F2D to context
    pub fn add_f2d(&mut self, name: &'a str, new: &'a F2D) {
        self.functions.insert(name, (&new.0, 2));
    }
    /// Add F3D to context
    pub fn add_f3d(&mut self, name: &'a str, new: &'a F3D) {
        self.functions.insert(name, (&new.0, 3));
    }

    pub(crate) fn get_func(&self, name: &'a str) -> Option<&(&Function, usize)> {
        self.functions.get(name)
    }

    /// Add symbol representing a fixed value
    pub fn add_symbol(&mut self, name: &'a str, value: f64) {
        self.symbols.insert(name, value);
    }
    pub(crate) fn get_symbol(&self, name: &'a str) -> Option<&f64> {
        self.symbols.get(name)
    }
}
