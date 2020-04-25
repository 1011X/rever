use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
	Unit,
	Bool,
	UInt, Int,
    Char, String,
	//Array(Box<Type>, usize),
	Fn(Vec<Type>, Box<Type>),
	//Proc(Vec<(bool, Type)>),
	//Alternate(Vec<Type>),
	//Composite(Vec<Type>),
}

impl Parse for Type {
	fn parse(tokens: &mut Tokens) -> ParseResult<Self> {
		match tokens.peek() {
			Some(Token::Ident(t)) if t == "unit" => {
				tokens.next();
				Ok(Type::Unit)
			}
			Some(Token::Ident(t)) if t == "bool" => {
				tokens.next();
				Ok(Type::Bool)
			}
			Some(Token::Ident(t)) if t == "uint" => {
				tokens.next();
				Ok(Type::UInt)
			}
			Some(Token::Ident(t)) if t == "int" => {
				tokens.next();
				Ok(Type::Int)
			}
			Some(Token::Ident(t)) if t == "str" => {
				tokens.next();
				Ok(Type::String)
			}
			Some(Token::Fn) => {
				tokens.next();
				
				if tokens.next() != Some(Token::LParen) {
					return Err("`(` for `fn` type");
				}
				
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
				
				if tokens.next() != Some(Token::Colon) {
					return Err("`:` to specify `fn` return type");
				}
				
				let ret = Type::parse(tokens)?;
				
				Ok(Type::Fn(params, Box::new(ret)))
			}
			
			_ => Err("a valid type")
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenize::tokenize;
    
    #[test]
    fn test() {
    	unimplemented!();
	}
}
