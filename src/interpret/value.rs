use std::fmt;

use crate::interpret::{EvalError, EvalResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
	Nil,
	Bool(bool),
	//Byte(u8),
	U32(u32),
	String(String),
	Array(Box<[Value]>),
	//Proc(Path),
}

use crate::ast::Type;
impl Value {
	pub fn get_type(&self) -> Type {
		match self {
			Value::Nil       => Type::Nil,
			Value::Bool(_)   => Type::U32,
			Value::U32(_)    => Type::U32,
			Value::String(s) => Type::String,
			
			Value::Array(_)  => todo!()
		}
	}
	
	pub fn swap(&mut self, val: &mut Value) -> EvalResult<()> {
		// check that types are the same.
		if self.get_type() != val.get_type() {
			return Err(EvalError::TypeMismatch {
				expected: self.get_type(),
				got: val.get_type(),
			});
		}
		
		std::mem::swap(self, val);
		Ok(())
	}
	
	pub fn xor(&mut self, val: &Value) -> EvalResult<()> {
		match (self, val) {
			(Value::Nil, Value::Nil) => {}
			
			(Value::Bool(a), Value::Bool(b)) => *a ^= b,
			
			(Value::U32(a), Value::U32(b)) => *a ^= b,
			
			_ => todo!()
		}
		Ok(())
	}
}

impl fmt::Display for Value {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Value::Nil => fmt.write_str("nil"),
			
			Value::Bool(b) => b.fmt(fmt),
			Value::U32(i) => i.fmt(fmt), /*{
				// TODO modify this to show bijective numerals
				let mut bij_repr = Vec::new();
				let mut carry = false;
				
				for digit in i.to_string().chars().rev() {
					match digit {
						'2'..='9' if carry => {
							let digit = digit.to_digit(10).unwrap() - 1;
							bij_repr.push(char::from_digit(digit, 10).unwrap());
							carry = false;
						}
						'1' if carry => {
							carry = false;
						}
						'0' if carry =>
							bij_repr.push('9'),
						
						// no carry
						'1'..='9' =>
							bij_repr.push(digit),
						'0' => {
							bij_repr.push('A');
							carry = true;
						}
						_ => unreachable!(),
					}
				}
				
				if carry {
					bij_repr.push('1');
				}
				bij_repr.push('0');
				bij_repr.reverse();
				
				let bij_repr: String = bij_repr.into_iter().collect();
				
				match i32::from_str_radix(&dec_repr, 10) {
					Ok(n) => Literal::Num(n),
					Err(_) => Err("a smaller number")?,
				}
			}*/
			
			//Value::Char(c)   => write!(fmt, "{:?}", c),
			Value::String(s) => write!(fmt, "{:?}", s),
			
			Value::Array(array) => {
				fmt.write_str("[")?;
				for value in array.iter() {
					write!(fmt, "{}, ", value)?;
				}
				fmt.write_str("]")
			}
		}
	}
}

impl From<()> for Value {
	#[inline]
	fn from(_: ()) -> Self { Value::Nil }
}

impl From<bool> for Value {
	#[inline]
	fn from(b: bool) -> Self { Value::Bool(b) }
}

impl From<char> for Value {
	#[inline]
	fn from(c: char) -> Self { Value::U32(c as u32) }
}

impl From<u32> for Value {
	#[inline]
	fn from(n: u32) -> Self { Value::U32(n) }
}

impl From<String> for Value {
	#[inline]
	fn from(s: String) -> Self { Value::String(s) }
}
