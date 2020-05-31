//use super::*;

pub use crate::ast::Type;

/*
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
	Unit,
	Bool,
	UInt, Int,
    Char, String,
	//Array(Box<Type>, usize),
	//Fn(Vec<Type>, Box<Type>),
	Proc(Vec<(bool, Type)>),
	//Alternate(Vec<Type>),
	//Composite(Vec<Type>),
}

impl From<ast::Type> for Type {
	fn from(v: ast::Type) -> Self {
		match v {
			ast::Type::Unit => Type::Unit,
			ast::Type::Bool => Type::Bool,
			ast::Type::UInt => Type::UInt,
			ast::Type::Int => Type::Int,
			ast::Type::Char => Type::Char,
			ast::Type::String => Type::String,
			ast::Type::Proc(v) => unimplemented!(),
			ast::Type::Fn(..) => unimplemented!(),
		}
	}
}
*/
