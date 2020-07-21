use super::*;

#[derive(Debug, Clone)]
pub enum Literal {
	Nil,
	Bool(bool),
	Int(i64),
	UInt(u64),
	Char(char),
	String(String),
	Array(Vec<Expr>),
	Fn(Vec<String>, Box<Expr>),
}

impl Parser {
	pub fn parse_lit(&mut self) -> ParseResult<Literal> {
		Ok(match self.peek() {
			Some(Token::Ident(x)) if x == "nil" => {
				self.next();
				Literal::Nil
			}
			
			Some(Token::Ident(x)) if x == "true" => {
				self.next();
				Literal::Bool(true)
			}
			
			Some(Token::Ident(x)) if x == "false" => {
				self.next();
				Literal::Bool(false)
			}
			
			Some(Token::Number(num)) =>
				match i64::from_str_radix(num, 10) {
					Ok(n) => {
						self.next();
						Literal::Int(n)
					}
					Err(_) => return Err("a smaller number"),
				}
			
			Some(&Token::Char(c)) => {
				self.next();
				Literal::Char(c)
			}
			
			Some(Token::String(_)) => {
				if let Token::String(s) = self.next().unwrap() {
					Literal::String(s)
				} else {
					unreachable!()
				}
			}
			
			Some(Token::LBracket) => {
				self.next();
				let mut elements = Vec::new();
				
				loop {
					match self.peek() {
						Some(Token::RBracket) => break,
						Some(_) => {
							elements.push(self.parse_expr()?);
							
							match self.peek() {
								Some(Token::Comma) => { self.next(); }
								Some(Token::RBracket) => {}
								_ => return Err("`,` or `]` after element in array"),
							}
						}
						None => Err("`,` or `]` after element in array")?,
					}
				}
				self.next();
				
				Literal::Array(elements)
			}
			
			Some(Token::Fn) => {
				self.next();
				
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
				
				Literal::Fn(args, Box::new(expr))
			}
			
			_ => return Err("valid literal value")
		})
	}
}

impl Eval for Literal {
	fn eval(&self, t: &Scope) -> EvalResult {
		Ok(match self {
			Literal::Nil       => Value::Nil,
			Literal::Bool(b)   => Value::Bool(*b),
			Literal::Int(n)    => Value::Int(*n),
			Literal::UInt(n)   => Value::Uint(*n),
			Literal::Char(c)   => Value::Char(*c),
			Literal::String(s) => Value::String(s.clone()),
			
			Literal::Array(arr) => Value::Array({
				let mut vec = Vec::with_capacity(arr.len());
				for expr in arr.iter() {
					vec.push(expr.eval(t)?);
				}
				vec.into_boxed_slice()
			}),
			Literal::Fn(args, ret) => todo!(),
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
