/*! AST representation of Rever. */

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

pub use self::expr::{Expr, BlockExpr, BinOp};
pub use self::function::Function;
pub use self::item::Item;
pub use self::literal::Literal;
pub use self::lvalue::{Deref, LValue};
pub use self::module::Module;
pub use self::procedure::{Param, Procedure};
pub use self::statement::Statement;
pub use self::term::Term;
pub use self::types::Type;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, Clone)]
pub enum ParseError {
	/// Parser reached an unexpected end-of-file.
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

impl From<&'static str> for ParseError {
	fn from(msg: &'static str) -> Self { ParseError::Msg(msg) }
}

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
		if self.peek.is_none() {
			self.peek = self.tokens.next();
		}
		self.peek.as_ref()
	}
	
	pub fn next(&mut self) -> Option<Token> {
		let token = match self.peek {
			None => self.tokens.next(),
			Some(_) => self.peek.take(),
		};
		
		if token == Some(Token::Newline) {
			self.line += 1;
		}
		
		token
	}
	
	pub fn expect(&mut self, tok: Token) -> Option<Token> {
		if self.peek() == Some(&tok) {
			self.next()
		} else {
			None
		}
	}
	
	pub fn skip_newlines(&mut self) {
		while self.expect(Token::Newline).is_some() {}
	}
	
	/// Returns the next identifier if any, and advances the iterator if found.
	pub fn expect_ident(&mut self) -> Option<String> {
		self.expect(Token::Ident)
			.map(|_| self.slice().to_string())
	}
	
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
