use super::*;

#[derive(Debug, Clone)]
pub enum Term {
	Lit(Literal),
	LVal(LValue),
}

impl Term {
	pub fn get_type(&self) -> Option<Type> {
		match self {
			Term::Lit(lit) => lit.get_type(),
			Term::LVal(_)  => None,
		}
	}
}

impl Parser<'_> {
	pub fn parse_term(&mut self) -> ParseResult<Term> {
		let mut clone = self.clone();
		
		if clone.parse_lit().is_ok() {
			Ok(self.parse_lit()?.into())
		} else {
			Ok(self.parse_lval()?.into())
		}
	}
}

impl Eval for Term {
	fn eval(&self, t: &StackFrame) -> EvalResult<Value> {
	    match self {
	        Term::Lit(lit)   => lit.eval(t),
	        Term::LVal(lval) => lval.eval(t),
	    }
	}
}

impl From<Literal> for Term {
	fn from(lit: Literal) -> Self { Term::Lit(lit) }
}

impl From<LValue> for Term {
	fn from(lval: LValue) -> Self { Term::LVal(lval) }
}
