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
}
