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
+ Chained relations, a la Python?
+ In `if` statements, conjunctions is `,` and disjunction is `;` (from Prolog).
  + No short-circuiting, like Pascal.
  + Short-circuiting can be achieved using `and` and `or`.

TODO:
+ Add precedences 2, 
*/

use crate::tokenize::Token;
use super::{ParseResult, Term};

#[derive(Debug, Clone)]
pub enum Expr {
	// precedence 1
	Term(Term),
	Group(Box<Expr>),
	
	// precedence 3
	Not(Box<Expr>),
	
	// precedence 4
	Exp(Box<Expr>, Box<Expr>),
	
	// precedence 5
	Mul(Box<Expr>, Box<Expr>),
	Div(Box<Expr>, Box<Expr>),
	Mod(Box<Expr>, Box<Expr>),
	And(Box<Expr>, Box<Expr>),
	
	// precedence 6
	Add(Box<Expr>, Box<Expr>),
	Sub(Box<Expr>, Box<Expr>),
	Or(Box<Expr>, Box<Expr>),
	
	// precedence 7
	Eq(Box<Expr>, Box<Expr>),
	Neq(Box<Expr>, Box<Expr>),
	Lt(Box<Expr>, Box<Expr>),
	Gt(Box<Expr>, Box<Expr>),
	Lte(Box<Expr>, Box<Expr>),
	Gte(Box<Expr>, Box<Expr>),
}

impl Expr {
	// Note to future self: This is how the parser should be structured:
	// bxpr -> expr {(=|≠|<|>|≤|≥|in) expr}
	// expr -> term {(+|-|or) term}
	// term -> exp {(*|/|mod|and) exp}
	// exp  -> prim {^ prim}
	// prim -> ( expr )
	//      -> factor
	pub fn parse(mut tokens: &[Token]) -> ParseResult<Self> {
	    enum Op { Eq, Neq, Lt, Gt, Lte, Gte, /* In */}
		
		// <term>
		let (first, t) = Expr::parse_expr(tokens)?;
		tokens = t;
	    
		let mut exprs: Vec<(Op, Expr)> = Vec::new();
		
		// { ('=' | '!=' | '<' | '>' | '<=' | '>=') <expr> }
		loop {
			let op = match tokens.first() {
				Some(Token::Eq)  => Op::Eq,
				Some(Token::Neq) => Op::Neq,
				Some(Token::Lt)  => Op::Lt,
				Some(Token::Gt)  => Op::Gt,
				Some(Token::Lte) => Op::Lte,
				Some(Token::Gte) => Op::Gte,
				//Some(Token::In)  => Op::In,
			    _ => break
			};
		    
		    let (expr, t) = Expr::parse_expr(&tokens[1..])?;
		    tokens = t;
		    exprs.push((op, expr));
		}
		
		if exprs.is_empty() {
		    return Ok((first, tokens));
		}
		
		let res = exprs.into_iter()
		.fold(first, |acc, (op, base)| match op {
			Op::Eq  => Expr::Eq(Box::new(acc), Box::new(base)),
			Op::Neq => Expr::Neq(Box::new(acc), Box::new(base)),
			Op::Lt  => Expr::Lt(Box::new(acc), Box::new(base)),
			Op::Gt  => Expr::Gt(Box::new(acc), Box::new(base)),
			Op::Lte => Expr::Lte(Box::new(acc), Box::new(base)),
			Op::Gte => Expr::Gte(Box::new(acc), Box::new(base)),
			//Op::In  => Expr::In(Box::new(acc), Box::new(base)),
		});
		
		Ok((res, tokens))
	}
	
	pub fn parse_expr(mut tokens: &[Token]) -> ParseResult<Self> {
	    enum Op { Add, Sub, Or }
		
		// <term>
		let (first, t) = Expr::parse_term(tokens)?;
		tokens = t;
	    
		let mut terms: Vec<(Op, Expr)> = Vec::new();
		
		// { ('+' | '-' | 'or') <term> }
		loop {
			let op = match tokens.first() {
				Some(Token::Add) => Op::Add,
				Some(Token::Sub) => Op::Sub,
				Some(Token::Or)  => Op::Or,
			    _ => break
			};
		    
		    let (term, t) = Expr::parse_term(&tokens[1..])?;
		    tokens = t;
		    terms.push((op, term));
		}
		
		if terms.is_empty() {
		    return Ok((first, tokens));
		}
		
		let res = terms.into_iter()
		.fold(first, |acc, (op, base)| match op {
			Op::Add => Expr::Add(Box::new(acc), Box::new(base)),
			Op::Sub => Expr::Sub(Box::new(acc), Box::new(base)),
			Op::Or  => Expr::Or(Box::new(acc), Box::new(base)),
		});
		
		Ok((res, tokens))
	}
	
	fn parse_term(mut tokens: &[Token]) -> ParseResult<Self> {
	    enum Op { Mul, Div, Mod, And }
		
		// <fact>
		let (first, t) = Expr::parse_exp(tokens)?;
		tokens = t;
	    
		let mut facts: Vec<(Op, Expr)> = Vec::new();
		
		// { ('*' | '/' | 'mod' | 'and') <fact> }
		loop {
			let op = match tokens.first() {
				Some(Token::Star)   => Op::Mul,
				Some(Token::FSlash) => Op::Div,
				Some(Token::Mod)    => Op::Mod,
				Some(Token::And)    => Op::And,
			    _ => break
			};
		    
		    let (fact, t) = Expr::parse_exp(&tokens[1..])?;
		    tokens = t;
		    facts.push((op, fact));
		}
		
		if facts.is_empty() {
		    return Ok((first, tokens));
		}
		
		let res = facts.into_iter()
		.fold(first, |acc, (op, base)| match op {
			Op::Mul => Expr::Mul(Box::new(acc), Box::new(base)),
			Op::Div => Expr::Div(Box::new(acc), Box::new(base)),
			Op::Mod => Expr::Mod(Box::new(acc), Box::new(base)),
			Op::And => Expr::And(Box::new(acc), Box::new(base)),
		});
		
		Ok((res, tokens))
	}
	
	// TODO rework this allow multiple operators in exponential phase.
	fn parse_exp(mut tokens: &[Token]) -> ParseResult<Self> {
		// <exp>
		let (first, t) = Expr::parse_base(tokens)?;
		tokens = t;
		
		let mut exps = Vec::new();
		
		// { ('^') <exp> }
		loop {
			let op = match tokens.first() {
				Some(Token::Caret) => {}
			    _ => break
			};
		    
		    let (exp, t) = Expr::parse_base(&tokens[1..])?;
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
	
	fn parse_base(tokens: &[Token]) -> ParseResult<Self> {
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
	
	/*
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
	*/
}

impl From<Term> for Expr {
	fn from(f: Term) -> Self {
		Expr::Term(f)
	}
}
