use super::*;

#[derive(Debug, Clone)]
pub enum Literal {
	Nil,
	Bool(bool),
	Int(i64),
	UInt(u64),
	//Char(char),
	String(String),
	Fn(Vec<String>, Box<Expr>)
}

impl Parse for Literal {
	fn parse(tokens: &mut Tokens) -> ParseResult<Self> {
		Ok(match tokens.peek() {
			Some(Token::Ident(x)) if x == "nil" => {
				tokens.next();
				Literal::Nil
			}
			
			Some(Token::Ident(x)) if x == "true" => {
				tokens.next();
				Literal::Bool(true)
			}
			
			Some(Token::Ident(x)) if x == "false" => {
				tokens.next();
				Literal::Bool(false)
			}
			
			Some(Token::Number(num)) => {
				/*if num.starts_with('+') || num.starts_with('-') {
					let (n, tx) = Literal::snum(&num)?;
					Ok((Literal::SNum(n), sx))
				else {*/
				match i64::from_str_radix(num, 10) {
					Ok(n)  => {
						tokens.next();
						Literal::Int(n)
					}
					Err(_) => return Err("a smaller number"),
				}
				//}
			}
			
			Some(Token::String(st)) => {
				let s = st.clone();
				tokens.next();
				Literal::String(s)
			}
			
			Some(Token::Fn) => {
				tokens.next();
				tokens.expect(&Token::LParen)
					.ok_or("`(` at start of function literal")?;
				
				let mut args = Vec::new();
				loop {
					match tokens.next() {
						Some(Token::RParen) => break,
						Some(Token::Ident(id)) => {
							args.push(id);
							
							if let Some(Token::Comma) = tokens.peek() {
								tokens.next();
							}
						}
						_ => return Err("`,` or `)` after argument name in function literal")
					}
				}
				
				tokens.expect(&Token::Colon)
					.ok_or("`:` after arguments in function literal")?;
				
				let expr = Expr::parse(tokens)?;
				
				Literal::Fn(args, Box::new(expr))
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
			Literal::String(_) => Some(Type::String),
			Literal::Fn(..)    => None,
		}
	}
}
