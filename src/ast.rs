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
use crate::tokenize::{Token, TokenStream};
use crate::interpret::{Eval, EvalResult, Scope, Value};

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
pub use self::expr::BlockExpr;
pub use self::expr::BinOp;
pub use self::function::Function;
pub use self::item::Item;
pub use self::literal::Literal;
pub use self::lvalue::LValue;
pub use self::lvalue::Deref;
pub use self::module::Module;
pub use self::procedure::{Param, Procedure};
pub use self::statement::Statement;
pub use self::term::Term;
pub use self::types::Type;

pub type ParseResult<T> = Result<T, &'static str>;

#[derive(Debug, Clone)]
enum ParseError {
	Eof,
	Empty,
	Msg(&'static str),
}

impl From<&'static str> for ParseError {
	fn from(msg: &'static str) -> Self {
		ParseError::Msg(msg)
	}
}
/*
struct Span<T> {
	data: T,
	span: Range<usize>,
}

impl<T> Span<T> {
	fn merge(&mut self, other: Span<T>) -> Span<T> {
		
	}
}
*/
#[derive(Debug, Clone)]
pub struct Parser {
	pub tokens: TokenStream,
	line: usize,
}

impl Parser {
	pub fn new(tokens: TokenStream) -> Self {
		Parser { tokens, line: 1 }
	}
	
	pub fn is_empty(&self) -> bool {
		self.tokens.is_empty()
	}
	
	pub fn peek(&self) -> Option<&Token> {
		self.tokens.peek()
	}
	
	pub fn next(&mut self) -> Option<Token> {
		let token = self.tokens.next();
		// update parser location
		if let Some(Token::Newline) = token {
			self.line += 1;
		}
		token
	}
	
	pub fn next_if(&mut self, f: impl FnOnce(&Token) -> bool) -> Option<Token> {
		match self.peek() {
			Some(token) if f(&token) => self.next(),
			_ => None,
		}
	}
	
	pub fn expect(&mut self, tok: &Token) -> Option<Token> {
		if self.peek() == Some(tok) {
			self.next()
		} else {
			None
		}
	}
	
	pub fn expect_ident(&mut self) -> Option<String> {
		if let Some(Token::Ident(_)) = self.peek() {
			self.next().map(|token| match token {
				Token::Ident(id) => id,
				_ => unreachable!(),
			})
		} else {
			None
		}
	}
	/*
	pub fn parse_with(&mut self, f: _) -> ParseResult<_> {
		todo!()
		// hint: u can use `peek()` and `next()` to track spans of consumed tokens
	}
	*/
	pub fn parse_file_module(&mut self) -> ParseResult<Vec<Item>> {
		let mut items = Vec::new();
		
		while ! self.is_empty() {
			match self.peek().unwrap() {
				Token::Newline => {
					self.next();
					continue;
				}
				_ => {}
			}
			items.push(self.parse_item()?);
		}
		
		Ok(items)
	}
}
