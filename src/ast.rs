/*! AST representation of Rever. */

use std::fmt;

use logos::{Logos, Span};

use crate::token::{Token, TokenStream};
use crate::interpret::{
	EvalError, EvalResult,
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
	/// Parser reached an unexpected token
	Expected(&'static str),
}

impl fmt::Display for ParseError {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ParseError::Expected(msg) => write!(fmt, "expected {}", msg),
		}
	}
}

impl From<&'static str> for ParseError {
	fn from(msg: &'static str) -> Self {
		ParseError::Expected(msg)
	}
}


/// parser state information.
#[derive(Clone)]
pub struct Parser<'src> {
	/// iterator for getting tokens
	lexer: TokenStream<'src>,
	/// stores current token for peeking purposes
	curr: Option<Token>,
	/// the current line number in source
	line: usize,
	/// byte position of last newline character in source
	last_nl: usize,
}

impl<'src> Parser<'src> {
	pub fn new(src: &'src str) -> Self {
		let mut lexer = Token::lexer(src);
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


/// converts a bijective numeral string into an ordinary decimal number string.
///
/// this function assumes i's characters only match `[1-9Aa]*`
pub fn bij_to_dec(i: &str) -> ParseResult<std::borrow::Cow<'_, str>> {
	// empty string is zero
	if i.is_empty() {
		return Ok(Cow::Borrowed("0"));
	}
	// no A's means no need to convert, reuse string as-is
	if i.find(['A', 'a']).is_none() {
		return Ok(Cow::Borrowed(i));
	}
	
	// make a vec of bools marking where there are a's
	let mut carries: Vec<_> = i.chars()
		.map(|digit| digit == 'A' || digit == 'a')
		.collect();
	
	let mut digits: Vec<_> = i.chars()
		.map(|d| if d == 'A' || d == 'a' { '0' } else { d })
		.collect();
	
	let mut result = String::with_capacity(i.len());
	// get rid of first carry immediately
	if carries[0] {
		result += "1";
	}
	// conveniently shifts all carries up for processing
	carries.remove(0);
	
	// continue as long as there are carries
	while carries.iter().any(|c| *c) {
		for i in 0..carries.len() {
			match (digits[i], carries[i]) {
				(_, false) => {}
				(c @ '0'..='8', true) => {
					digits[i] = char::from_digit(c.to_digit(10).unwrap() + 1, 10)
						.unwrap();
				}
				_ => todo!()
			}
		}
		if carries[0] {
			result += "";
		}
		carries.remove(0);
	}
	
	Ok(Cow::Owned(result))
}
