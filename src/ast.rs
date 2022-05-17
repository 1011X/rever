/*! AST representation of Rever. */

use std::fmt;

use logos::Span;

use crate::token::{Token, TokenStream};
use crate::interpret::{
	Eval, EvalError, EvalResult,
	StackFrame, Value,
};

mod expression;
mod block_expr;
mod function;
mod item;
mod literal;
mod lvalue;
mod module;
mod procedure;
mod statement;
mod types;

pub use self::expression::{BinOp, Expr, ExprErr};
pub use self::block_expr::{BlockExpr, BlockExprErr};
pub use self::function::Function;
pub use self::item::Item;
pub use self::literal::Literal;
pub use self::lvalue::{Deref, LValue, LValErr};
pub use self::module::Module;
pub use self::procedure::{Param, Procedure, ProcDef};
pub use self::statement::Stmt;
pub use self::types::{Type, TypeErr};

pub type ParseResult<T> = Result<T, ParseError>;

pub trait AstNode: Sized {
	type Error: std::error::Error;
	
	fn parse(p: &mut Parser) -> Result<Self, Self::Error>;
}

#[derive(Debug, Clone)]
pub enum ParseError {
	/// Parser reached an unexpected end-of-file.
	Eof,
	Expected(&'static str),
	InvalidChar,
}

impl fmt::Display for ParseError {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ParseError::Eof => fmt.write_str("not end-of-file"),
			ParseError::Expected(msg) => write!(fmt, "expected {}", msg),
			ParseError::InvalidChar => fmt.write_str("valid character literal"),
		}
	}
}

impl From<&'static str> for ParseError {
	fn from(msg: &'static str) -> Self {
		ParseError::Expected(msg)
	}
}


/**
This parser makes a deliberate decision to deviate from the usual iterator.
 
This is mostly a result of wanting easier error-handling; I didn't want to keep
storing the unexpected token in the error node, which I had to do since the
lexer has already moved on and forgotten about it by the time I call `.next()`.

Then it came to me: Why keep moving the token around when I could just... 
leave it there? That way you still have access to `logos`'s `Lexer`
information, including the source string.

This "not consuming the token immediately" approach would likely fare well with
token enums with payloads. Rever's tokens don't have payloads.
*/
#[derive(Clone)]
pub struct Parser<'src> {
	lexer: TokenStream<'src>,
	curr: Option<Token>,
	line: usize,
	last_nl: usize,
}

impl<'src> Parser<'src> {
	pub fn new(mut lexer: TokenStream<'src>) -> Self {
		let curr = lexer.next();
		Parser { lexer, curr, line: 1, last_nl: 0 }
	}
	
	pub fn slice(&self) -> &str {
		self.lexer.slice()
	}
	
	pub fn span(&self) -> Span {
		self.lexer.span()
	}
	
	pub fn remainder(&self) -> &str {
		self.lexer.remainder()
	}
	
	pub fn peek(&self) -> Option<&Token> {
		self.curr.as_ref()
	}
	
	pub fn line(&self) -> usize {
		self.line
	}
	
	pub fn column(&self) -> usize {
		self.span().end - self.last_nl
	}
	
	pub fn next(&mut self) -> Option<Token> {
		let prev = self.curr;
		self.curr = self.lexer.next();
		
		// adjust location state
		if self.curr == Some(Token::Newline) {
			self.line += 1;
			self.last_nl = self.span().end;
		}
		
		prev
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
	
	pub fn debug(&self) {
		eprintln!("{},{}: {:?}", self.line(), self.column(), self.slice());
	}
}
