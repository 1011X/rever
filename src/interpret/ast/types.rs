use crate::tokenize::Token;
use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
	Bool,
	Uint, Int,
    Char, String,
	//Pointer(Box<Type>),
	//Array(Box<Type>, usize),
	//Fn(Vec<Type>, Box<Type>),
	Proc(Vec<(bool, Type)>),
	//Composite(String),
}

impl Type {
	pub fn parse(tokens: &[Token]) -> ParseResult<Self> {
		match tokens.first() {
			Some(Token::Ident(t)) if t == "bool" =>
				Ok((Type::Bool, &tokens[1..])),
			
			Some(Token::Ident(t)) if t == "uint" =>
				Ok((Type::Uint, &tokens[1..])),
			
			_f =>
		        Err(format!("unknown type: {:?}", _f))
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
