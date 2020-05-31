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
use crate::tokenize::{Token, Tokens};

mod expr;
mod function;
mod item;
mod literal;
mod lvalue;
mod module;
mod procedure;
mod statement;
mod term;
mod types;

pub use self::expr::Expr;
pub use self::function::Function;
pub use self::item::Item;
pub use self::literal::Literal;
pub use self::lvalue::LValue;
pub use self::module::Module;
pub use self::procedure::{Param, Procedure};
pub use self::statement::Statement;
pub use self::term::Term;
pub use self::types::Type;

pub type ParseResult<T> = Result<T, &'static str>;

pub trait Parse {
	fn parse(tokens: &mut Tokens) -> ParseResult<Self>
	where Self: std::marker::Sized;
}

pub fn parse_file_module(tokens: &mut Tokens) -> ParseResult<Vec<Item>> {
    let mut items = Vec::new();
    
	while tokens.len() > 0 {
		items.push(Item::parse(tokens)?);
	}
	
	Ok(items)
}
