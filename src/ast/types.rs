use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
	Infer,
	Never,
	Unit,
	Bool,
	UInt, Int,
	Char, String,
	//Array(Box<(Type, Span)>, usize),
	Array(usize),
	Fn(Vec<(Type, Span)>, Box<(Type, Span)>),
	Proc(Vec<(bool, (Type, Span))>),
	//Alternate(Vec<Type>),
	//Composite(Vec<Type>),
}

impl Parser {
	pub fn parse_type(&mut self) -> ParseResult<Type> {
		Ok(match self.peek().ok_or("a type")? {
			Token::Ident(_) => {
				let (name, span) = self.expect_ident_span().unwrap();
				match name.as_str() {
					"_"    => (Type::Infer, span),
					"void" => (Type::Never, span),
					"unit" => (Type::Unit, span),
					"bool" => (Type::Bool, span),
					"uint" => (Type::UInt, span),
					"int"  => (Type::Int, span),
					"str"  => (Type::String, span),
					id     => todo!(),
				}
			}
			
			Token::Fn => {
				let (_, start) = self.next().unwrap();
				
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
				let span = start.merge(&ret.1);
				
				(Type::Fn(params, Box::new(ret)), span)
			}
			
			Token::Proc => {
				let (_, start) = self.next().unwrap();
				
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
				let (_, end) = self.next().unwrap();
				
				(Type::Proc(params), start.merge(&end))
			}
			
			_ => return Err("a valid type")
		})
	}
}

impl Default for Type {
	fn default() -> Self { Type::Infer }
}
