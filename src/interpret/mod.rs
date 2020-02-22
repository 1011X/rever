mod ast;
mod value;

use std::fs::File;

use self::ast::{Item, ParseResult, Tokens};

pub use self::value::Value;

pub fn parse_items(tokens: &mut Tokens) -> ParseResult<Vec<Item>> {
    let mut items = Vec::new();
    
	while tokens.len() > 0 {
		items.push(Item::parse(tokens)?);
	}
	
	Ok(items)
}


pub type Scope = Vec<(String, Value)>;

#[derive(Clone)]
pub struct StackFrame {
    args: Vec<Value>,
    locals: Vec<(String, Value)>,
}

pub type Stack = Vec<StackFrame>;

/*
// TODO: ensure reversibility of files and streams
struct IoStack<T: Read + Write> {
	
}
*/
