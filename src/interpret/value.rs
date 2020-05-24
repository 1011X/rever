use crate::hir::{Literal, Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
	Nil,
    Bool(bool),
    //Byte(u8),
    Int(i64),
    Uint(u64),
    //Char(char),
    String(String),
    //Proc(String),
}

impl Value {
    pub fn get_type(&self) -> Type {
        match self {
        	Value::Nil => Type::Unit,
            Value::Bool(_) => Type::Bool,
            Value::Int(_) => Type::Int,
            Value::Uint(_) => Type::UInt,
            //Value::Signed(_) => Type::I32,
            //Value::Char(_) => Type::Char,
            Value::String(_) => Type::String,
        }
    }
}


impl From<Literal> for Value {
    fn from(l: Literal) -> Self {
        match l {
            Literal::Nil => Value::Nil,
            Literal::Bool(b) => Value::Bool(b),
            Literal::Int(n) => Value::Int(n),
            //Literal::Signed(n) => Value::Signed(n),
            Literal::String(s) => Value::String(s.clone()),
        }
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self { Value::Nil }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self { Value::Bool(b) }
}

impl From<i64> for Value {
    fn from(n: i64) -> Self { Value::Int(n) }
}

impl From<String> for Value {
	fn from(s: String) -> Self { Value::String(s) }
}
