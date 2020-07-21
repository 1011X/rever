use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
	Infer,
	Never,
	Unit,
	Bool,
	UInt, Int,
	Char, String,
	//Array(Box<Type>, usize),
	Array(usize),
	Fn(Vec<Type>, Box<Type>),
	Proc(Vec<(bool, Type)>),
	//Alternate(Vec<Type>),
	//Composite(Vec<Type>),
}

impl Parser {
	pub fn parse_type(&mut self) -> ParseResult<Type> {
		Ok(match self.peek().ok_or("a type")? {
			Token::Ident(_) => {
				let name = self.expect_ident().unwrap();
				match name.as_str() {
					"_"    => Type::Infer,
					"void" => Type::Never,
					"unit" => Type::Unit,
					"bool" => Type::Bool,
					"uint" => Type::UInt,
					"int"  => Type::Int,
					"str"  => Type::String,
					id     => todo!(),
				}
			}
			
			Token::Fn => {
				self.next();
				
				self.expect(&Token::LParen)
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
								_ => return Err("`,` or `)` in fn param list"),
							}
						}
						None => return Err("`,` or `)` in fn param list"),
					}
				}
				self.next();
				
				self.expect(&Token::Colon)
					.ok_or("`:` to specify `fn` return type")?;
				
				let ret = self.parse_type()?;
				
				Type::Fn(params, Box::new(ret))
			}
			
			Token::Proc => {
				self.next();
				
				self.expect(&Token::LParen)
					.ok_or("`(` for `proc` type")?;
				
				let mut params = Vec::new();
				loop {
					match self.peek() {
						Some(Token::RParen) => break,
						Some(_) => {
							let var = self.expect(&Token::Var).is_some();
							let t = self.parse_type()?;
							
							params.push((var, t));
							
							match self.peek() {
								Some(Token::Comma) => { self.next(); }
								Some(Token::RParen) => {}
								_ => return Err("`,` or `)` in fn param list"),
							}
						}
						None => return Err("`)` or `,` in proc param list"),
					}
				}
				self.next();
				
				Type::Proc(params)
			}
			
			_ => return Err("a valid type")
		})
	}
}

impl Default for Type {
	fn default() -> Self { Type::Infer }
}
