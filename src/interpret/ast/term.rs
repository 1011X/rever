use crate::tokenize::Token;
use crate::interpret::{ScopeTable, Value};
use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
	Lit(Literal),
	LVal(LValue),
}

impl Term {
	pub fn parse(s: &[Token]) -> ParseResult<Self> {
	    if let Ok((lit, s)) = Literal::parse(s) {
	        Ok((Term::Lit(lit), s))
        }
        else {
		    let (lval, s) = LValue::parse(s)?;
		    Ok((Term::LVal(lval), s))
	    }
	}
	
	pub fn eval(&self, t: &ScopeTable) -> Value {
	    match self {
	        Term::Lit(lit) => lit.eval(),
	        Term::LVal(lval) => lval.eval(t),
	    }
	}
}

impl From<Literal> for Term {
    fn from(lit: Literal) -> Self {
        Term::Lit(lit)
    }
}

impl From<LValue> for Term {
    fn from(lval: LValue) -> Self {
        Term::LVal(lval)
    }
}
