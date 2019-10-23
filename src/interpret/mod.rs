use std::collections::HashMap;

mod ast;
mod value;

use super::tokenize::Token;
use self::ast::{Item, Module, ParseResult, Type};
pub use self::value::Value;

pub fn parse_module(tokens: &[Token]) -> ParseResult<Module> {
    Module::parse(tokens)
}

pub fn parse_item(tokens: &[Token]) -> ParseResult<Item> {
    Item::parse(tokens)
}


#[derive(Clone)]
pub struct ScopeTable {
    pub procedures: HashMap<String, Vec<(bool, Type)>>,
    //functions: HashMap<String, Function>,
    pub locals: HashMap<String, Value>,
}

#[derive(Clone)]
pub struct StackFrame {
    args: Vec<Value>,
    locals: Vec<(String, Value)>,
}

pub type Stack = Vec<StackFrame>;
