#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
	Nil,
	Bool(bool),
	//Byte(u8),
	Int(i64),
	Uint(u64),
	Char(char),
	String(String),
	Array(Box<[Value]>),
	//Proc(String),
}

use crate::ast::Type;
impl Value {
	pub fn get_type(&self) -> Type {
		match self {
			Value::Nil       => Type::Unit,
			Value::Bool(_)   => Type::Bool,
			Value::Int(_)    => Type::Int,
			Value::Uint(_)   => Type::UInt,
			//Value::Signed(_) => Type::I32,
			Value::Char(_)   => Type::Char,
			Value::String(_) => Type::String,
			
			Value::Array(_)  => todo!()
		}
	}
}

impl fmt::Display for Value {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Value::Nil => fmt.write_str("nil"),
			
			Value::Bool(b) => b.fmt(fmt),
			Value::Int(i)  => i.fmt(fmt),
			Value::Uint(u) => u.fmt(fmt),
			
			Value::Char(c)   => write!(fmt, "{:?}", c),
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
	fn from(_: ()) -> Self { Value::Nil }
}

impl From<bool> for Value {
	fn from(b: bool) -> Self { Value::Bool(b) }
}

impl From<char> for Value {
	fn from(c: char) -> Self { Value::Char(c) }
}

impl From<i64> for Value {
	fn from(n: i64) -> Self { Value::Int(n) }
}

impl From<String> for Value {
	fn from(s: String) -> Self { Value::String(s) }
}
