use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
	Unit,
	Bool,
	UInt, Int,
    Char, String,
	//Array(Box<Type>, usize),
	Fn(Vec<Type>, Box<Type>),
	Proc(Vec<(bool, Type)>),
	//Alternate(Vec<Type>),
	//Composite(Vec<Type>),
}

impl From<ast::Type> for Type {
	fn from(v: ast::Type) -> Self {
		unimplemented!()
	}
}
