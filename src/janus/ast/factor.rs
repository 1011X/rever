use super::*;
use super::super::interpret::{self, SymTab, Value};

#[derive(Debug)]
pub enum Factor {
	Literal(Literal),
	LValue(LValue),
}

impl Factor {
	named!(pub parse<Factor>, alt_complete!(
		map!(Literal::parse, Factor::Literal)
		| map!(LValue::parse, Factor::LValue)
	));
	
	fn eval(&self, symtab: &SymTab) -> interpret::Result {
		match *self {
			Factor::Literal(ref lit) => Ok(lit.to_value()),
			Factor::LValue(ref lval) => lval.eval(symtab),
		}
	}
}
