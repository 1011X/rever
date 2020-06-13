use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
	Never,
	Unit,
	Bool,
	UInt, Int,
    Char, String,
	Array(Box<Type>, usize),
	Fn(Vec<Type>, Box<Type>),
	Proc(Vec<(bool, Type)>),
	//Alternate(Vec<Type>),
	//Composite(Vec<Type>),
}

impl Parse for Type {
	fn parse(tokens: &mut Tokens) -> ParseResult<Self> {
		match tokens.peek().ok_or("a type")? {
			Token::Ident(t) if t == "never" => {
				tokens.next();
				Ok(Type::Never)
			}
			Token::Ident(t) if t == "unit" => {
				tokens.next();
				Ok(Type::Unit)
			}
			Token::Ident(t) if t == "bool" => {
				tokens.next();
				Ok(Type::Bool)
			}
			Token::Ident(t) if t == "uint" => {
				tokens.next();
				Ok(Type::UInt)
			}
			Token::Ident(t) if t == "int" => {
				tokens.next();
				Ok(Type::Int)
			}
			Token::Ident(t) if t == "str" => {
				tokens.next();
				Ok(Type::String)
			}
			Token::Fn => {
				tokens.next();
				
				tokens.expect(&Token::LParen)
					.ok_or("`(` for `fn` type")?;
				
				let mut params = Vec::new();
				
				while tokens.peek() != Some(&Token::RParen) {
					params.push(Type::parse(tokens)?);
					
					match tokens.peek() {
						Some(Token::RParen) => {}
						Some(Token::Comma) => { tokens.next(); }
						_ => return Err("`)` or `,` in fn param list")
					}
				}
				tokens.next();
				
				tokens.expect(&Token::Colon)
					.ok_or("`:` to specify `fn` return type")?;
				
				let ret = Type::parse(tokens)?;
				
				Ok(Type::Fn(params, Box::new(ret)))
			}
			Token::Proc => {
				tokens.next();
				
				tokens.expect(&Token::LParen)
					.ok_or("`(` for `proc` type")?;
				
				let mut params = Vec::new();
				
				while tokens.peek() != Some(&Token::RParen) {
					let mut var = false;
					
					if let Some(Token::Var) = tokens.peek() {
						var = true;
						tokens.next();
					}
					
					params.push((var, Type::parse(tokens)?));
					
					match tokens.peek() {
						Some(Token::RParen) => {}
						Some(Token::Comma) => { tokens.next(); }
						_ => return Err("`)` or `,` in proc param list")
					}
				}
				tokens.next();
				
				Ok(Type::Proc(params))
			}
			
			_ => Err("a valid type")
		}
	}
}
