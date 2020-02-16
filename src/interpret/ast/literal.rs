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
	pub fn eval(&self) -> Value {
		match self {
			//Literal::Nil         => Value::Nil,
			Literal::Bool(b)     => Value::Bool(*b),
			Literal::Unsigned(n) => Value::Unsigned(*n),
			Literal::String(s)   => Value::String(s.clone()),
		}
	}
	
	pub fn parse(tokens: &[Token]) -> ParseResult<Self> {
		match tokens.first() {
			//Some(Token::Ident(x)) if x == "nil" =>
				//Ok((Literal::Nil, &tokens[1..])),
			
			Some(Token::Ident(x)) if x == "true" =>
				Ok((Literal::Bool(true), &tokens[1..])),
			
			Some(Token::Ident(x)) if x == "false" =>
				Ok((Literal::Bool(false), &tokens[1..])),
			
			Some(Token::Number(num)) => {
				/*if num.starts_with('+') || num.starts_with('-') {
					let (n, tx) = Literal::snum(&num)?;
					Ok((Literal::SNum(n), sx))
				else {*/
				match u64::from_str_radix(num, 10) {
					Ok(n)  => Ok((Literal::Unsigned(n), &tokens[1..])),
					Err(_) => Err(format!("number too big")),
				}
				//}
			}
			
			Some(Token::String(st)) =>
				Ok((Literal::String(st.clone()), &tokens[1..])),
			
			_ => Err(format!("invalid literal value"))
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
