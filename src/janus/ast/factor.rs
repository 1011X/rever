use super::*;
use super::super::interpret::{Value, SymTab};
use super::super::compile::State;
use rel;

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
	
	pub fn compile(&self, state: &mut State, code: &mut Vec<rel::Op>) -> rel::Reg {
		match *self {
			Factor::LValue(ref lval) => lval.compile(state, code),
			Factor::Literal(ref lit) => unimplemented!(),
		}
	}
	
	pub fn eval(&self, symtab: &SymTab) -> Result<Value, String> {
		match *self {
			Factor::Literal(ref lit) => Ok(lit.to_value()),
			Factor::LValue(ref lval) => lval.eval(symtab),
		}
	}
}