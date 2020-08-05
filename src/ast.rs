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
use std::fmt;

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

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, Clone)]
pub enum ParseError {
	Eof,
	Empty,
	Msg(&'static str),
	InvalidChar,
}

impl fmt::Display for ParseError {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ParseError::Eof => fmt.write_str("not end-of-file"),
			ParseError::Empty => todo!(),
			ParseError::Msg(s) => fmt.write_str(s),
			ParseError::InvalidChar => fmt.write_str("valid character literal"),
		}
	}
}

impl fmt::Display for ParseError {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ParseError::Eof =>
				fmt.write_str("reached end of file"),
			ParseError::Expected(msg) => {
				fmt.write_str("expected ")?;
				fmt.write_str(msg)
			}
		}
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
#[derive(Clone)]
pub struct Parser<'src> {
	pub tokens: TokenStream<'src>,
	peek: Option<Token>,
	line: usize,
}

impl<'src> Parser<'src> {
	pub fn new(tokens: TokenStream<'src>) -> Self {
		Parser { tokens, peek: None, line: 1 }
	}
	
	pub fn slice(&self) -> &str {
		self.tokens.slice()
	}
	
	pub fn peek(&mut self) -> Option<&Token> {
		//self.tokens.peek()
		if self.peek.is_none() {
			self.peek = self.tokens.next();
		}
		self.peek.as_ref()
	}
	
	pub fn next(&mut self) -> Option<Token> {
		/*
		let token = self.tokens.next();
		// update parser location
		if let Some(Token::Newline) = token {
			self.line += 1;
		}
		token
		*/
		match self.peek {
			None => self.tokens.next(),
			Some(_) => self.peek.take(),
		}
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
		
		while self.peek().is_some() {
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
