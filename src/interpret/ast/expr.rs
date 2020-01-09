/*!
Expressions in Rever have 5 levels of precendence. From strongest to weakest:
1. Parentheses
2. Function calls
3. Unary operators: not - (maybe: ! ~)
4. Exponential operators: ^ << >> shl shr rol ror
5. Multiplicative operators: * / mod as and
6. Additive operators: + - or xor
7. Relational operators: = != ≠ < > <= ≤ >= ≥ in

Ideas:
+ No AND expressions in if/when statements
  + Must have another if/when inside it
  + What about loops? how would that work?
+ Chained relations, a la Python?
*/

use crate::tokenize::Token;
use crate::interpret::{Scope, Value};
use super::*;

#[derive(Debug, Clone)]
pub enum Expr {
	Term(Term),
	Group(Box<Expr>),
	
	Not(Box<Expr>),
	
	Exp(Box<Expr>, Box<Expr>),
	
	Mul(Box<Expr>, Box<Expr>),
	Div(Box<Expr>, Box<Expr>),
	
	Add(Box<Expr>, Box<Expr>),
	Sub(Box<Expr>, Box<Expr>),
	
	Eq(Box<Expr>, Box<Expr>),
	Ne(Box<Expr>, Box<Expr>),
	//Le(Factor, Factor),
	Ge(Box<Expr>, Box<Expr>),
	Lt(Box<Expr>, Box<Expr>),
	//Gt(Box<Expr>, Box<Expr>),
	
	And(Box<Expr>, Box<Expr>),
	Or(Box<Expr>, Box<Expr>),
	//Xor(Factor, Factor),
}

impl Expr {
	pub fn eval(&self, t: &Scope) -> Value {
		match self {
			Expr::Term(term) => term.eval(t),
			Expr::Group(e) => e.eval(t),
			
			Expr::Exp(base, exp) => unimplemented!(),
			
			Expr::Mul(l, r) => match (l.eval(t), r.eval(t)) {
				(Value::Unsigned(l), Value::Unsigned(r)) => Value::from(l * r),
				_ => panic!("tried multiplying non-integer values")
			}
			Expr::Div(l, r) => match (l.eval(t), r.eval(t)) {
				(Value::Unsigned(l), Value::Unsigned(r)) => Value::from(l / r),
				_ => panic!("tried dividing non-integer values")
			}
			
			Expr::Add(l, r) => match (l.eval(t), r.eval(t)) {
				(Value::Unsigned(l), Value::Unsigned(r)) => Value::from(l + r),
				_ => panic!("tried multiplying non-integer values")
			}
			Expr::Sub(l, r) => match (l.eval(t), r.eval(t)) {
				(Value::Unsigned(l), Value::Unsigned(r)) => Value::from(l - r),
				_ => panic!("tried multiplying non-integer values")
			}
			
			Expr::Eq(l, r) => Value::from(l.eval(t) == r.eval(t)),
			Expr::Ne(l, r) => Value::from(l.eval(t) != r.eval(t)),
			
			Expr::Lt(l, r) => match (l.eval(t), r.eval(t)) {
				(Value::Unsigned(l), Value::Unsigned(r)) => Value::from(l < r),
				_ => panic!("tried comparing non-integer values")
			}
			Expr::Ge(l, r) => match (l.eval(t), r.eval(t)) {
				(Value::Unsigned(l), Value::Unsigned(r)) => Value::from(l >= r),
				_ => panic!("tried comparing non-integer values")
			}
			
			
			Expr::Not(e) => match e.eval(t) {
				Value::Bool(true) => Value::Bool(false),
				Value::Bool(false) => Value::Bool(true),
				_ => panic!("tried negating non-boolean expression")
			}
			Expr::And(l, r) => match (l.eval(t), r.eval(t)) {
				(Value::Bool(l), Value::Bool(r)) => Value::from(l && r),
				_ => panic!("tried ANDing non-boolean expressions")
			}
			Expr::Or(l, r) => match (l.eval(t), r.eval(t)) {
				(Value::Bool(l), Value::Bool(r)) => Value::from(l || r),
				_ => panic!("tried ORing non-boolean expressions")
			}
		}
	}
	
	// Note to future self: This is how the parser should be structured:
	// expr -> term {(+|-) term}
	// term -> fact {(*|/) fact}
	// fact -> exp {^ exp}
	// exp -> ( expr )
	//     -> factor
	pub fn parse(tokens: &[Token]) -> ParseResult<Self> {
	    Expr::parse_factor(tokens)
	}
	
	// "term" here is not to be confused with Term; Term is a variable or
	// literal, while term here means "stuff separated by + or -"
	fn parse_term(mut tokens: &[Token]) -> ParseResult<Self> {
		let mut facts: Vec<(Token, Expr)> = Vec::new();
		let (first, t) = Expr::parse_factor(tokens)?;
		tokens = t;
		
	    unimplemented!()
	}
	
	fn parse_factor(mut tokens: &[Token]) -> ParseResult<Self> {
		let mut exps = Vec::new();
		let (first, t) = Expr::parse_exp(tokens)?;
		tokens = t;
		
		while tokens.first() == Some(&Token::Caret) {
			let (exp, t) = Expr::parse_exp(&tokens[1..])?;
			tokens = t;
			exps.push(exp);
		}
		
		if exps.is_empty() {
		    return Ok((first, tokens));
		}
		
		let last = exps.pop().unwrap();
		let res = exps.into_iter()
			.rfold(last, |acc, base|
			    Expr::Exp(Box::new(base), Box::new(acc))
		    );
		
		Ok((Expr::Exp(Box::new(first), Box::new(res)), tokens))
	}
	
	fn parse_exp(tokens: &[Token]) -> ParseResult<Self> {
		// check if there's an open parenthesis
		if tokens.first() == Some(&Token::LParen) {
			let (expr, t) = Expr::parse(&tokens[1..])?;
			
			// make sure there's a closing parenthesis
			if t.first() != Some(&Token::RParen) {
				Err(format!("no closing parenthesis found"))
			}
			else {
				Ok((Expr::Group(Box::new(expr)), &t[1..]))
			}
		}
		else {
			// otherwise, treat it as a Term.
			let (term, t) = Term::parse(tokens)?;
			Ok((Expr::Term(term), t))
		}
	}
}

impl From<Term> for Expr {
	fn from(f: Term) -> Self {
		Expr::Term(f)
	}
}
