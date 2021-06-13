use std::num::ParseIntError;

use super::*;

#[derive(Debug, Clone)]
pub enum Literal {
	Nil,
	Bool(bool),
	//Int(i64),
	Num(u32),
	Char(char),
	String(String),
	Array(Vec<Expr>),
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
	pub fn parse_lit(&mut self) -> ParseResult<Literal> {
		let lit = match self.peek() {
			Some(Token::ConIdent) => match self.slice() {
				"Nil" => Literal::Nil,
				"True" => Literal::Bool(true),
				"False" => Literal::Bool(false),
				_ => Err("`nil`, `True`, or `False`")?
				//_ => return Err(LitErr::UnrecognizedIdent)
			}
			
			Some(Token::Number) => {
				//Literal::Num(u32::from_str_radix(self.slice(), 10)?),
				match u32::from_str_radix(self.slice(), 10) {
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
						_ => return Err(ParseError::InvalidChar),
					}
					Some(c) if ! "\'\n\t\r\0".contains(c) => c,
					_ => return Err(ParseError::InvalidChar),
				};
				
				match chars.next() {
					Some('\'') => {}
					Some(c) => return Err(ParseError::InvalidChar),
					None => return Err(ParseError::Eof),
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
							
							Some(c) =>
								return Err(ParseError::InvalidChar),
								//return Err(LitErr::InvalidEscChar),
							None =>
								return Err(ParseError::Eof),
						}),
						Some(c) => string.push(c),
						None => return Err(ParseError::Eof),
					}
				}
				
				string.shrink_to_fit();
				Literal::String(string)
			}
			
			// array literal
			Some(Token::LBracket) => {
				let mut elements = Vec::new();
				self.next();
				
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
					.ok_or("`=` after arguments in closure")?;
				
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

impl Eval for Literal {
	fn eval(&self, t: &StackFrame) -> EvalResult<Value> {
		Ok(match self {
			Literal::Nil       => Value::Nil,
			Literal::Bool(b)   => Value::Bool(*b),
			//Literal::Int(n)    => Value::Int(*n),
			Literal::Num(n)   => Value::Uint(*n as u64),
			Literal::Char(c)   => Value::Char(*c),
			Literal::String(s) => Value::String(s.clone()),
			
			Literal::Array(arr) => Value::Array({
				let mut vec = Vec::with_capacity(arr.len());
				for expr in arr.iter() {
					vec.push(expr.eval(t)?);
				}
				vec.into_boxed_slice()
			}),
			//Literal::Fn(args, ret) => todo!(),
		})
	}
}

impl Literal {
	pub fn get_type(&self) -> Option<Type> {
		match self {
			Literal::Nil       => Some(Type::Unit),
			Literal::Bool(_)   => Some(Type::Bool),
			//Literal::Int(_)    => Some(Type::Int),
			Literal::Num(_)   => Some(Type::UInt),
			Literal::Char(_)   => Some(Type::Char),
			Literal::String(_) => Some(Type::String),
			Literal::Array(_)  => None,
			//Literal::Fn(..)    => None,
		}
	}
}
