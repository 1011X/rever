use std::num::ParseIntError;

use super::*;

#[derive(Debug, Clone)]
pub enum Literal {
	Nil,
	//Int(i64),
	Num(i32),
	Char(char),
	String(String),
	Array(Vec<Expr>),
	Variant(String),
	//Fn(Vec<String>, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum LitErr {
	/// Reached end-of-file (EOF) while reading file.
	Eof,
	
	/// Given identifier is not a valid literal value.
	UnrecognizedIdent,
	
	/// Given number is too large to contain.
	Num(ParseIntError),
	
	/// Unknown escape character.
	UnknownEscChar,
	
	/// Prohibitted character literal.
	BannedCharLit,
	
	/// No final end quote for character literal found.
	EndQuote,
}

impl fmt::Display for LitErr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Eof => f.write_str("end of file"),
			Self::UnrecognizedIdent => f.write_str("unrecognized primitive"),
			Self::Num(pie) => pie.fmt(f),
			Self::UnknownEscChar => f.write_str("unknown escape character"),
			_ => todo!()
		}
	}
}

impl From<ParseIntError> for LitErr {
	fn from(pie: ParseIntError) -> Self { Self::Num(pie) }
}

impl Parser<'_> {
	pub fn parse_literal(&mut self) -> ParseResult<Literal> {
		let lit = match self.peek() {
			Some(Token::ConIdent) => Literal::Variant(self.slice().to_string()),
			
			Some(Token::Number) => {
				let mut dec_repr = Vec::with_capacity(self.slice().len());
				let mut carry = false;
				
				for digit in self.slice()[1..].chars().rev() {
					match digit {
						'1'..='8' if carry => {
							let digit = digit.to_digit(10).unwrap() + 1;
							dec_repr.push(char::from_digit(digit, 10).unwrap());
							carry = false;
						}
						'9' if carry =>
							dec_repr.push('0'),
						'A' | 'a' if carry =>
							dec_repr.push('1'),
						
						'1'..='9' =>
							dec_repr.push(digit),
						'A' | 'a' => {
							dec_repr.push('0');
							carry = true;
						}
						_ => unreachable!(),
					}
				}
				
				// this covers the empty string case
				dec_repr.push(if carry { '1' } else { '0' });
				dec_repr.reverse();
				
				let dec_repr: String = dec_repr.into_iter().collect();
				
				match i32::from_str_radix(&dec_repr, 10) {
					Ok(n) => Literal::Num(n),
					Err(_) => Err("a smaller number")?,
				}
			}
			
			Some(Token::Char) => {
				let mut chars = self.slice().chars();
				chars.next();
				
				let c = match chars.next() {
					Some('\\') => match chars.next() {
						Some('\\') => '\\',
						Some('\'') => '\'',
						Some('n') => '\n',
						Some('t') => '\t',
						Some('r') => '\r',
						Some('0') => '\0',
						_ => Err("a valid escape character")?,
					}
					Some(c) if !"\'\n\t\r\0".contains(c) => c,
					_ => Err("an accepted character in literal")?,
				};
				
				match chars.next() {
					Some('\'') => {}
					_ => Err("a terminated character literal")?,
				}
				
				Literal::Char(c)
			}
			
			Some(Token::String) => {
				let mut chars = self.slice().chars();
				let mut string = String::new();
				
				let dual = match chars.next().unwrap() {
					'"' => '"',
					'“' => '”',
					'»' => '«',
					'«' => '»',
					_ => unreachable!()
				};
				
				loop {
					match chars.next() {
						Some(c) if c == dual => break,
						Some('\\') => string.push(match chars.next() {
							Some('\\') => '\\',
							Some('"')  => '"',
							Some('”')  => '”',
							Some('»')  => '»',
							Some('«')  => '«',
							
							Some('n')  => '\n',
							Some('t')  => '\t',
							Some('r')  => '\r',
							Some('0')  => '\0',
							
							_ => Err("a valid escape character")?,
						}),
						Some(c) => string.push(c),
						None => Err("a terminated string")?,
					}
				}
				
				string.shrink_to_fit();
				Literal::String(string)
			}
			
			// array literal
			Some(Token::LBracket) => {
				self.next();
				
				let mut elements = Vec::new();
				loop {
					match self.peek() {
						// found ']'
						Some(Token::RBracket) => break,
						// element in array
						Some(_) => {
							elements.push(self.parse_expr()?); // XXX
							
							match self.peek() {
								Some(Token::Comma) => { self.next(); }
								Some(Token::RBracket) => {}
								_ => Err("`,` or `]` after element in array")?,
							}
						}
						None => Err("`,` or `]` after element in array")?,
					}
				}
				
				Literal::Array(elements)
			}
			
			// function/closure literal
			/*
			Some(Token::Fn) => {
				self.next();
				self.expect(Token::LParen)
					.ok_or("`(` at start of closure")?;
				
				let mut args = Vec::new();
				loop {
					match self.peek() {
						Some(Token::RParen) => break,
						Some(Token::VarIdent) => {
							let id = self.slice();
							args.push(id.to_string());
							self.next();
							
							match self.peek() {
								Some(Token::Comma) => { self.next(); }
								Some(Token::RParen) => {}
								_ => Err("`,` or `)` after argument name in closure")?,
							}
						}
						_ => Err("`,` or `)` after argument name in closure")?,
					}
				}
				self.next();
				
				// TODO `:` with return type should be optional here
				
				self.expect(Token::Equal)
					.ok_or("`=` after closure arguments")?;
					//.ok_or(LitErr::
				
				let expr = self.parse_expr()?;
				
				Literal::Fn(args, Box::new(expr))
			}
			*/
			
			_ => Err("valid literal value")?
		};
		
		self.next();
		Ok(lit)
	}
}

use crate::interpret::StackFrame;
impl Literal {
	pub fn eval(&self, ctx: &StackFrame) -> EvalResult<Value> {
		Ok(match self {
			Literal::Nil       => Value::Nil,
			//Literal::Int(n)    => Value::U32(*n),
			Literal::Num(n)    => Value::U32(*n as u32),
			Literal::Char(c)   => Value::U32(*c as u32),
			Literal::String(s) => Value::String(s.clone()),
			
			Literal::Array(arr) => Value::Array({
				let mut vec = Vec::with_capacity(arr.len());
				for expr in arr.iter() {
					vec.push(expr.eval(ctx)?);
				}
				vec.into_boxed_slice()
			}),
			
			Literal::Variant(_) => unimplemented!(),
			//Literal::Fn(args, ret) => todo!(),
		})
	}
}

impl Literal {
	pub fn get_type(&self) -> Option<Type> {
		match self {
			//Literal::Int(_)    => Some(Type::Int),
			Literal::Nil       => None,
			Literal::Num(_)    => Some(Type::U32),
			Literal::Char(_)   => Some(Type::U32),
			Literal::String(_) => Some(Type::String),
			Literal::Array(v)  => None,
			Literal::Variant(_) => None,
			//Literal::Fn(..)    => None,
		}
	}
}
