use crate::ast::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Factor {
	Lit(Literal),
	LVal(LValue),
}

impl Factor {
	pub fn parse(s: &str) -> ParseResult<Self> {
	    if let Ok((lit, s)) = Literal::parse(s) {
	        return Ok((Factor::Lit(lit), s))
        }
        
        let (lval, s) = LValue::parse(s)?;
        Ok((Factor::LVal(lval), s))
	}
	
	pub fn eval(&self, t: &ScopeTable) -> Value {
	    match self {
	        Factor::Lit(lit) => lit.eval(t),
	        Factor::LVal(lval) => lval.eval(t),
	    }
	}
}
