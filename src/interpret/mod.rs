mod ast;
mod value;

use std::fs::File;

use super::tokenize::Token;
use self::ast::Item;
use self::ast::ParseResult;

pub use self::value::Value;

pub fn parse_items(mut tokens: &[Token]) -> ParseResult<Vec<Item>> {
    let mut items = Vec::new();
    
	while ! tokens.is_empty() {
		let (item, t) = Item::parse(tokens)?;
		tokens = t;
		items.push(item);
	}
	
	Ok((items, tokens))
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
