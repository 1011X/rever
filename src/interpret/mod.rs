use std::collections::HashMap;

mod ast;
mod value;

use super::tokenize::Token;
use self::ast::Module;
use self::ast::ParseResult;

pub use self::value::Value;

pub fn parse_module(tokens: &[Token]) -> ParseResult<Module> {
    Module::parse(tokens)                                                                                          
}


pub type Scope = Vec<(String, Value)>;

#[derive(Clone)]
pub struct StackFrame {
    args: Vec<Value>,
    locals: Vec<(String, Value)>,
}

pub type Stack = Vec<StackFrame>;
