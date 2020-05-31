use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
	Nil,
	Bool(bool),
	Int(i64),
	UInt(u64),
	//Char(char),
	String(String),
}

impl Literal {
	pub fn eval(&self) -> Value {
		match self {
			Literal::Nil       => Value::Nil,
			Literal::Bool(b)   => Value::Bool(*b),
			Literal::Int(n)    => Value::Int(*n),
			Literal::UInt(n)   => Value::Uint(*n),
			Literal::String(s) => Value::String(s.clone()),
		}
	}
	
	pub fn get_type(&self) -> Type {
		match self {
			Literal::Nil       => Type::Unit,
			Literal::Bool(_)   => Type::Bool,
			Literal::Int(_)    => Type::Int,
			Literal::UInt(_)   => Type::UInt,
			Literal::String(_) => Type::String,
		}
	}
}

impl From<ast::Literal> for Literal {
	fn from(v: ast::Literal) -> Self {
		match v {
			ast::Literal::Nil => Literal::Nil,
			ast::Literal::Bool(b) => Literal::Bool(b),
			ast::Literal::Int(i) => Literal::Int(i),
			ast::Literal::UInt(u) => Literal::UInt(u),
			ast::Literal::String(s) => Literal::String(s),
			_ => todo!()
		}
	}
}
