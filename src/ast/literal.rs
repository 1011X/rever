use super::*;

#[derive(Debug, Clone)]
pub enum Literal {
	Nil,
	Bool(bool),
	Int(i64),
	UInt(u64),
	Char(char),
	String(String),
	Array(Vec<(Expr, Span)>),
	Fn(Vec<String>, Box<(Expr, Span)>),
}

impl Parser {
	pub fn parse_lit(&mut self) -> ParseResult<Literal> {
		Ok(match self.peek() {
			Some(Token::Ident(x)) if x == "nil" => {
				let (_, span) = self.next().unwrap();
				(Literal::Nil, span)
			}
			
			Some(Token::Ident(x)) if x == "true" => {
				let (_, span) = self.next().unwrap();
				(Literal::Bool(true), span)
			}
			
			Some(Token::Ident(x)) if x == "false" => {
				let (_, span) = self.next().unwrap();
				(Literal::Bool(false), span)
			}
			
			Some(Token::Number(num)) =>
				match i64::from_str_radix(num, 10) {
					Ok(n) => {
						let (_, span) = self.next().unwrap();
						(Literal::Int(n), span)
					}
					Err(_) => return Err("a smaller number"),
				}
			
			Some(&Token::Char(c)) => {
				let (_, span) = self.next().unwrap();
				(Literal::Char(c), span)
			}
			
			Some(Token::String(_)) => {
				let (s, span) = self.next().unwrap();
				if let Token::String(s) = s {
					(Literal::String(s), span)
				} else {
					unreachable!()
				}
			}
			
			Some(Token::LBracket) => {
				let (_, start) = self.next().unwrap();
				
				let mut elements = Vec::new();
				
				loop {
					match self.peek() {
						Some(Token::RBracket) =>
							break,
						Some(_) => {
							elements.push(self.parse_expr()?);
							
							match self.peek() {
								Some(Token::Comma) => { self.next(); }
								Some(Token::RBracket) => {}
								_ => return Err("`,` or `]` after element in array literal"),
							}
						}
						None =>
							return Err("`,` or `]` after element in array literal"),
					}
				}
				let (_, end) = self.next().unwrap();
				
				(Literal::Array(elements), start.merge(&end))
			}
			
			Some(Token::Fn) => {
				let (_, start) = self.next().unwrap();
				
				self.expect(&Token::LParen)
					.ok_or("`(` at start of closure")?;
				
				let mut args = Vec::new();
				loop {
					match self.peek() {
						Some(Token::RParen) => break,
						Some(Token::Ident(_)) => {
							let id = self.expect_ident().unwrap();
							args.push(id);
							
							match self.peek() {
								Some(Token::Comma) => { self.next(); }
								Some(Token::RParen) => {}
								_ => return Err("`,` or `)` after argument name in closure"),
							}
						}
						_ => return Err("`,` or `)` after argument name in closure")
					}
				}
				self.next();
				
				self.expect(&Token::Colon)
					.ok_or("`:` after arguments in closure")?;
				
				let expr = self.parse_expr()?;
				let span = start.merge(&expr.1);
				
				(Literal::Fn(args, Box::new(expr)), span)
			}
			
			_ => return Err("valid literal value")
		})
	}
}

impl Literal {
	pub fn get_type(&self) -> Option<Type> {
		match self {
			Literal::Nil       => Some(Type::Unit),
			Literal::Bool(_)   => Some(Type::Bool),
			Literal::Int(_)    => Some(Type::Int),
			Literal::UInt(_)   => Some(Type::UInt),
			Literal::Char(_)   => Some(Type::Char),
			Literal::String(_) => Some(Type::String),
			Literal::Array(_)  => None,
			Literal::Fn(..)    => None,
		}
	}
}
