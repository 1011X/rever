use std::error;

use super::*;

/// A type annotation.
///
/// This is used wherever a type annotation occurs in the syntax, such as a
/// variable declaration, procedure data, or function parameters.
//
// In the type checking phase, a separate enum is used that does not have an
// `Infer` case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
	/// placeholder for when a type isn't given or known at a point in the ast
	Infer,
	
	Nil,
	
	/// 32-bit unsigned integer, default number type
	U32,
	
	/// resizeable utf-8 string type
	String,
	
	/// a type covering the range of unsigned numbers from 0 to N-1, where N is
	/// given by a numeric literal. when paired with function type syntax, can
	/// be used to specify an array of a certain size.
	/// 
	/// for example, the type `0256` would cover 0-255, equivalent to a `u8` in
	/// rust. there's also `0` (the empty type), `01` (the unit type), `02` (the
	/// boolean type), etc.
	Index(u32),
	
	Stack(Box<Self>),
	
	/// a type that takes a value of the first type and returns a value of the
	/// second type.
	/// 
	/// for example, `u32 -> str` takes a number of type `u32` and returns a
	/// string. you can also do `03 -> u32` for an array of three `u32`s.
	Fn(Box<Self>, Box<Self>),
	
	//Decl(String, Vec<Self>),
}

impl Default for Type {
	fn default() -> Self { Type::Infer }
}

/// a generic type error.
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
		Ok(match self.peek().ok_or("a type")? {
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
					_ => todo!()
				}
			}
			
			Token::Number => {
				todo!()
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
		})
	}
}
