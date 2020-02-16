use super::ast::{Literal, Param, Statement, Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Bool(bool),
    Unsigned(u64),
    //Signed(u64),
    //Char(char),
    String(String),
    //Proc(Vec<Param>, Vec<Statement>),
}

impl Value {
    pub fn get_type(&self) -> Type {
        match self {
        	//Value::Nil => Type::Unit,
            Value::Bool(_) => Type::Bool,
            Value::Unsigned(_) => Type::Uint,
            //Value::Signed(_) => Type::I32,
            //Value::Char(_) => Type::Char,
            Value::String(_) => Type::String,
        }
    }
}


impl From<Literal> for Value {
    fn from(l: Literal) -> Self {
        match l {
            //Literal::Nil => Value::Nil,
            Literal::Bool(b) => Value::Bool(b),
            Literal::Unsigned(n) => Value::Unsigned(n),
            //Literal::Signed(n) => Value::Signed(n),
            Literal::String(s) => Value::String(s.clone()),
        }
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self { Value::Bool(b) }
}

impl From<u64> for Value {
    fn from(n: u64) -> Self { Value::Unsigned(n) }
}

impl From<String> for Value {
	fn from(s: String) -> Self { Value::String(s) }
}
