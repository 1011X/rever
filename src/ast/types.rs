use std::error;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
	Infer,
	Never,
	Unit,
	Bool,
	UInt, Int,
	Char, String,
	//Array(Box<Self>, usize),
	Array(usize),
	Fn(Vec<Self>, Box<Self>),
	Proc(Vec<(bool, Self)>),
	//Alternate(Vec<Self>),
	//Composite(Vec<Self>),
}

#[derive(Debug, Clone)]
pub struct TypeErr;

impl fmt::Display for TypeErr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("type error")
	}
}

impl error::Error for TypeErr {
}

impl Parser<'_> {
	pub fn parse_type(&mut self) -> ParseResult<Type> {
		let typ = match self.peek().ok_or("a type")? {
			Token::Underscore => Type::Infer,
			
			Token::ConIdent => {
				let t = match self.slice() {
					"Void" => Type::Never,
					"Unit" => Type::Unit,
					"Bool" => Type::Bool,
					"Num"  => Type::UInt,
					"Char" => Type::Char,
					"Str"  => Type::String,
					id     => todo!("custom types not yet supported: {:?}", id)
				};
				self.next();
				
				t
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
			
			_ => Err("a valid type")?
		};
		
		Ok(typ)
	}
}

impl Default for Type {
	fn default() -> Self { Type::Infer }
}
