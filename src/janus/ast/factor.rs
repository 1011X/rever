use super::*;
use super::super::interpret::{Value, SymTab};

#[derive(Debug, PartialEq, Eq)]
pub enum Factor {
	Literal(Literal),
	LValue(LValue),
}

impl Factor {
	named!(pub parse<Self>, alt_complete!(
		map!(Literal::parse, Factor::Literal)
		| map!(LValue::parse, Factor::LValue)
	));
	
	pub fn eval(&self, symtab: &SymTab) -> Result<Value, String> {
		match *self {
			Factor::Literal(ref lit) => Ok(lit.to_value()),
			Factor::LValue(ref lval) => lval.eval(symtab),
		}
	}
}
