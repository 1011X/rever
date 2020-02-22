use crate::tokenize::Token;
use crate::interpret::Value;
use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
	//Nil,
	Bool(bool),
	Unsigned(u64),
	//Signed(u64),
	//Char(char),
	String(String),
}

impl Literal {
	pub fn parse(tokens: &mut Tokens) -> ParseResult<Self> {
		match tokens.next() {
			//Some(Token::Ident(x)) if x == "nil" =>
				//Ok(Literal::Nil),
			
			Some(Token::Ident(x)) if x == "true" =>
				Ok(Literal::Bool(true)),
			
			Some(Token::Ident(x)) if x == "false" =>
				Ok(Literal::Bool(false)),
			
			Some(Token::Number(num)) => {
				/*if num.starts_with('+') || num.starts_with('-') {
					let (n, tx) = Literal::snum(&num)?;
					Ok((Literal::SNum(n), sx))
				else {*/
				match u64::from_str_radix(&num, 10) {
					Ok(n)  => Ok(Literal::Unsigned(n)),
					Err(_) => Err("a smaller number"),
				}
				//}
			}
			
			Some(Token::String(st)) =>
				Ok(Literal::String(st)),
			
			_ => Err("valid literal value")
		}
	}
	
	pub fn eval(&self) -> Value {
		match self {
			//Literal::Nil         => Value::Nil,
			Literal::Bool(b)     => Value::Bool(*b),
			Literal::Unsigned(n) => Value::Unsigned(*n),
			Literal::String(s)   => Value::String(s.clone()),
		}
	}
}

// FIXME
#[cfg(test)]
mod tests {
	use crate::tokenize::tokenize;
	use super::*;
	#[test]
	fn boolean() {
		assert_eq!(
			Literal::parse(&[Token::Ident("true".to_string())]).unwrap(),
			(Literal::Bool(true), &[][..])
		);
		assert_eq!(
			Literal::parse(&[Token::Ident("false".to_string())]).unwrap(),
			(Literal::Bool(false), &[][..])
		);
	}
	#[test]
	fn int() {
		assert_eq!(
			Literal::parse(&[Token::Number("0".to_string())]).unwrap(),
			(Literal::Unsigned(0), &[][..])
		);
		//assert_eq!(Literal::parse("-1").unwrap(), (Literal::Num(-1), &[][..]));
		assert_eq!(
			Literal::parse(&[Token::Number("10".to_string())]).unwrap(),
			(Literal::Unsigned(10), &[][..])
		);
	}
	/*
	#[test]
	fn string() {
		assert_eq!(
			Literal::parse(&tokenize("\"abc\"").unwrap()).unwrap(),
			(Literal::String(format!("abc")), &[][..])
		);
		assert_eq!(
			Literal::parse(&tokenize("\"a\\\"b\\\\c\"").unwrap()).unwrap(),
			(Literal::String(format!("a\"b\\c")), &[][..])
		);
	}
	*/
}
