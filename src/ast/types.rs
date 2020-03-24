use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
	Unit,
	Bool,
	Uint, Int,
    Char, String,
	//Pointer(Box<Type>),
	//Array(Box<Type>, usize),
	//Fn(Vec<Type>, Box<Type>),
	Proc(Vec<(bool, Type)>),
	//Composite(String),
}

impl Parse for Type {
	fn parse(tokens: &mut Tokens) -> ParseResult<Self> {
		match tokens.peek() {
			Some(Token::Ident(t)) if t == "bool" => {
				tokens.next();
				Ok(Type::Bool)
			}
			//Some(Token::Ident(t)) if t == "uint" => Ok(Type::Uint),
			Some(Token::Ident(t)) if t == "int" => {
				tokens.next();
				Ok(Type::Int)
			}
			Some(Token::Ident(t)) if t == "str" => {
				tokens.next();
				Ok(Type::String)
			}
			
			_ => Err("valid type")
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenize::tokenize;
    
    #[test]
    fn boolean() {
    	assert_eq!(
    		Type::parse(&tokenize("bool").unwrap()).unwrap(),
    		(Type::Bool, &[][..])
		);
	}
	
    #[test]
    fn int() {
    	assert_eq!(
    		Type::parse(&tokenize("uint").unwrap()).unwrap(),
    		(Type::Uint, &[][..])
		);
	}
}
