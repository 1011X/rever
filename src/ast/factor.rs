use crate::ast::*;

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
	
	pub fn eval(&self, t: &VarTable) -> Value {
	    match self {
	        Factor::Lit(l) => l.eval(),
	        Factor::LVal(l) => l.eval(),
	    }
	}
}
