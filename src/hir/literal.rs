use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
	Nil,
	Bool(bool),
	Int(i64),
	//Signed(u64),
	//Char(char),
	String(String),
}

impl Literal {
	pub fn eval(&self) -> Value {
		match self {
			Literal::Nil       => Value::Nil,
			Literal::Bool(b)   => Value::Bool(*b),
			Literal::Int(n)    => Value::Int(*n),
			Literal::String(s) => Value::String(s.clone()),
		}
	}
	
	pub fn get_type(&self) -> Type {
		match self {
			Literal::Nil       => Type::Unit,
			Literal::Bool(_)   => Type::Bool,
			Literal::Int(_)    => Type::Int,
			Literal::String(_) => Type::String,
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
