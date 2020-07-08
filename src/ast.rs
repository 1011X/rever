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
use crate::span::Span;
use crate::tokenize::{Token, Tokens, TokenStream};

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

pub type ParseResult<T> = Result<(T, Span), ParseError>;

#[derive(Debug, Clone)]
pub enum ParseError {
	Eof,
	Expected(&'static str),
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

impl From<&'static str> for ParseError {
	#[inline]
	fn from(msg: &'static str) -> Self { ParseError::Expected(msg) }
}

#[derive(Debug, Clone)]
pub struct Parser {
	pub tokens: TokenStream,
	line: usize,
	col: usize,
}

impl Parser {
	pub fn new(tokens: Tokens) -> Self {
		Parser { tokens, line: 1, col: 1 }
	}
	
	pub fn is_empty(&self) -> bool {
		self.tokens.is_empty()
	}
	
	pub fn peek(&self) -> Option<&Token> {
		self.tokens.peek().map(|(t, _)| t)
	}
	
	pub fn next(&mut self) -> Option<(Token, Span)> {
		let token = self.tokens.next();
		// update parser location
		if let Some((token, span)) = &token {
			if *token == Token::Newline {
				self.line += 1;
				self.col = 1;
			} else {
				// FIXME
				//self.col = span.end - span.start;
			}
		}
		token
	}
	
	pub fn next_if(&mut self, f: impl FnOnce(&Token) -> bool) -> Option<(Token, Span)> {
		match self.peek() {
			Some(token) if f(&token) => self.next(),
			_ => None,
		}
	}
	
	pub fn expect(&mut self, tok: &Token) -> Option<(Token, Span)> {
		if self.peek() == Some(tok) {
			self.next()
		} else {
			None
		}
	}
	
	pub fn expect_ident(&mut self) -> Option<String> {
		self.expect_ident_span().map(|(id, _)| id)
	}
	
	pub fn expect_ident_span(&mut self) -> Option<(String, Span)> {
		if let Some(Token::Ident(_)) = self.peek() {
			self.next().map(|(token, span)| match token {
				Token::Ident(id) => (id, span),
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
}

pub fn parse_file_module(tokens: Tokens) -> Result<Vec<Item>, ParseError> {
	let mut parser = Parser::new(tokens);
	let mut items = Vec::new();
	
	while ! parser.is_empty() {
		let (item, _) = parser.parse_item()?;
		items.push(item);
	}
	
	Ok(items)
}
