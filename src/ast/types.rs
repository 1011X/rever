use std::error;

use super::*;

/// A type annotation on the AST.
///
/// This is used wherever a type annotation can occur in the syntax,
/// such as a variable declaration, procedure data, or function
/// parameters.
///
/// In the type checking phase, a separate enum is used that does not
/// have an `Infer` case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
	Infer,
	Nil,
	Bool,
	U32,
	String,
	Fn(Vec<Self>, Box<Self>),
	Decl(String, Vec<Self>),
}

impl Default for Type {
	fn default() -> Self { Type::Infer }
}


#[derive(Debug, Clone)]
pub struct TypeErr;

impl fmt::Display for TypeErr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("type error")
	}
}

impl error::Error for TypeErr {}


impl Parser<'_> {
	pub fn parse_type(&mut self) -> ParseResult<Type> {
		let typ = match self.peek().ok_or("a type")? {
			Token::Underscore => Type::Infer,
			
			// named types with optional generics
			Token::ConIdent => {
				let name = self.slice().to_string();
				self.next();
				
				let mut type_params = Vec::new();
				
				if let Some(Token::LBracket) = self.peek() {
					self.next();
					loop {
						match self.peek() {
							Some(Token::RBracket) => break,
							Some(_) => {
								type_params.push(self.parse_type()?);

								match self.peek() {
									Some(Token::Comma) => { self.next(); }
									Some(Token::RBracket) => {}
									_ => Err("`,` or `]` in generic param list")?,
								}
							}
							None => Err("`,` or `]` in generic param list")?,
						}
					}
					self.next();
				}
				
				match name.as_str() {
					"U32" => Type::U32,
					"Str" => Type::String,
					_ => Type::Decl(name, type_params),
				}
			}
			/*
			// tuples
			Token::LParen => {
				self.next();

				let mut types = Vec::new();
				loop {
					match self.peek() {
						Some(Token::RParen) => break,
						Some(_) => {
							types.push(self.parse_type()?);

							match self.peek() {
								Some(Token::Comma) => { self.next(); }
								Some(Token::RParen) => {}
								_ => Err("`,` or `)` in tuple list")?,
							}
						}
						None => Err("`,` or `)` in tuple list")?,
					}
				}
				self.next();

				Type::Tuple(types)
			}
			
			// arrays
			Token::LBracket => {
				self.next();

				let size = match self.parse_literal()? {
					Literal::Num(n) => n as usize,
					_ => Err("size of array")?,
				};

				self.expect(Token::Semicolon)
					.ok_or("`;` after array size")?;

				let inner_type = self.parse_type()?;

				self.expect(Token::RBracket)
					.ok_or("`]` after inner array type")?;

				Type::Array(Box::new(inner_type), Some(size))
			}
			*/
			Token::Fn => {
				self.next();
				
				self.expect(Token::LParen)
					.ok_or("`(` for `fn` type")?;
				
				let mut params = Vec::new();
				loop {
					match self.peek() {
						Some(Token::RParen) => break,
						Some(_) => {
							params.push(self.parse_type()?);
							
							match self.peek() {
								Some(Token::Comma) => { self.next(); }
								Some(Token::RParen) => {}
								_ => Err("`,` or `)` in fn param list")?,
							}
						}
						None => Err("`,` or `)` in fn param list")?,
					}
				}
				self.next();
				
				self.expect(Token::Colon)
					.ok_or("`:` to specify `fn` return type")?;
				
				let ret = self.parse_type()?;
				
				Type::Fn(params, Box::new(ret))
			}
			/*
			Token::Proc => {
				self.next();
				
				self.expect(Token::LParen)
					.ok_or("`(` for `proc` type")?;
				
				let mut params = Vec::new();
				loop {
					match self.peek() {
						Some(Token::RParen) => break,
						Some(_) => {
							let var = self.expect(Token::Var).is_some();
							let t = self.parse_type()?;
							
							params.push((var, t));
							
							match self.peek() {
								Some(Token::Comma) => { self.next(); }
								Some(Token::RParen) => {}
								_ => Err("`,` or `)` in fn param list")?,
							}
						}
						None => Err("`)` or `,` in proc param list")?,
					}
				}
				self.next();
				
				Type::Proc(params)
			}
			*/
			_ => Err("a valid type")?
		};
		
		Ok(typ)
	}
}
