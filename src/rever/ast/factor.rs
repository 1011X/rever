use super::*;
use super::super::compile::SymbolTable;
use rel;

#[derive(Debug)]
pub enum Factor {
	Lit(Literal),
	LVal(LValue),
}

impl Factor {
	named!(pub parse<Self>, ws!(alt_complete!(
		map!(Literal::parse, Factor::Lit)
		| map!(LValue::parse, Factor::LVal)
	)));
	
	pub fn compile(&self, st: &mut SymbolTable) -> Vec<rel::Op> {
		use rel::Op;
		vec![Op::Nop]
	}
}
