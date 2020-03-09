use crate::interpret::{Scope, Value};
use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
	Lit(Literal),
	LVal(LValue),
}

impl Term {
	pub fn parse(s: &mut Tokens) -> ParseResult<Self> {
	    if let Ok(lit) = Literal::parse(s) {
	        Ok(Term::Lit(lit))
        }
        else {
		    let lval = LValue::parse(s)?;
		    Ok(Term::LVal(lval))
	    }
	}
	
	pub fn eval(&self, t: &Scope) -> Value {
	    match self {
	        Term::Lit(lit) => lit.eval(),
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
