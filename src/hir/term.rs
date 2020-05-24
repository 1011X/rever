use super::*;

#[derive(Debug, Clone)]
pub enum Term {
	Lit(Literal),
	LVal(LValue),
}

impl Term {
	pub fn eval(&self, t: &Scope) -> Value {
	    match self {
	        Term::Lit(lit) => lit.eval(),
	        Term::LVal(lval) => lval.eval(t),
	    }
	}
	
	pub fn get_type(&self) -> Option<Type> {
		match self {
			Term::Lit(lit) => Some(lit.get_type()),
			Term::LVal(_)  => None,
		}
	}
}

impl From<Literal> for Term {
    fn from(lit: Literal) -> Self { Term::Lit(lit) }
}

impl From<LValue> for Term {
    fn from(lval: LValue) -> Self { Term::LVal(lval) }
}
