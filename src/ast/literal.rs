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
			
			_ => return Err("valid literal value")
		})
	}
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
