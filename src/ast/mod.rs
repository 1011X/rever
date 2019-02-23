/*!
AST representation of Rever.
*/

/*
TODO: what does a complete program even look like?

List of state given to program:
* return code
* cli args
* env vars
* heap/memory store

"Devices" to handle:
* filesystem
* stdio

*/

mod expr;
mod factor;
//mod function;
mod item;
mod literal;
mod lvalue;
mod param;
mod procedure;
mod program;
mod statement;
mod types;

pub use self::param::Param;
pub use self::expr::Expr;
pub use self::factor::Factor;
//pub use self::function::Function;
pub use self::procedure::Procedure;
pub use self::item::Item;
pub use self::literal::Literal;
pub use self::lvalue::LValue;
pub use self::program::Program;
pub use self::statement::Statement;
pub use self::types::Type;

use std::str;
use std::collections::HashMap;

pub type ParseResult<'a, T> = Result<(T, &'a str), String>;

// ident ::= [_A-Za-z][_A-Za-z0-9]*
pub fn ident(i: &str) -> ParseResult<&str> {
	let mut idx = 0;
	
	if i.is_empty() {
		return Err("reached eof".to_string());
	}
	
	// [A-Za-z_]
	if !i.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_') {
		return Err("doesn't start with valid character".to_string());
	}
	idx += 1;
	
	// [A-Za-z0-9_]*
	while i[idx..].starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_') {
		idx += 1;
	}
	
	Ok((&i[..idx], &i[idx..]))
}


pub struct ScopeTable {
    procedures: HashMap<String, Vec<(bool, Type)>>,
    //functions: HashMap<String, Function>,
    locals: HashMap<String, Value>,
}

pub struct StackFrame {
    args: Vec<Value>,
    locals: Vec<(String, Value)>,
}

pub type Stack = Vec<StackFrame>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Bool(bool),
    Int(i32),
}

impl Value {
    fn get_type(&self) -> Type {
        match self {
            Value::Bool(_) => Type::Bool,
            Value::Int(_) => Type::I32,
        }
    }
}

impl From<Literal> for Value {
    fn from(l: Literal) -> Self {
        match l {
            Literal::Bool(b) => Value::Bool(b),
            Literal::Num(n) => Value::Int(n),
        }
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self { Value::Bool(b) }
}

impl From<i32> for Value {
    fn from(n: i32) -> Self { Value::Int(n) }
}
