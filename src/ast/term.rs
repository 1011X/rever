use super::*;

#[derive(Debug, Clone)]
pub enum Term {
	Lit(Literal),
	LVal(LValue),
}

impl Parser {
	pub fn parse_term(&mut self) -> ParseResult<Term> {
		let mut clone = self.clone();
		
		if clone.parse_lit().is_ok() {
			let (lit, span) = self.parse_lit()?;
			Ok((Term::Lit(lit), span))
		} else {
			let (lval, span) = self.parse_lval()?;
			Ok((Term::LVal(lval), span))
		}
	}
}

impl Term {
	pub fn get_type(&self) -> Option<Type> {
		match self {
			Term::Lit(lit) => lit.get_type(),
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
