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
use super::{ParseResult, Term, Tokens};
use crate::interpret::{Value, Scope};
use crate::interpret::EvalResult;

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
	pub fn parse(tokens: &mut Tokens) -> ParseResult<Self> {
	    enum Op { Eq, Neq, Lt, Gt, Lte, Gte, /* In */}
		
		// <term>
		let first = Expr::parse_expr(tokens)?;
		let mut exprs: Vec<(Op, Expr)> = Vec::new();
		
		// { ('=' | '!=' | '<' | '>' | '<=' | '>=') <expr> }
		loop {
			let op = match tokens.peek() {
				Some(Token::Eq)  => Op::Eq,
				Some(Token::Neq) => Op::Neq,
				Some(Token::Lt)  => Op::Lt,
				Some(Token::Gt)  => Op::Gt,
				Some(Token::Lte) => Op::Lte,
				Some(Token::Gte) => Op::Gte,
				//Some(Token::In)  => Op::In,
			    _ => break
			};
			tokens.next();
		    
		    let expr = Expr::parse_expr(tokens)?;
		    exprs.push((op, expr));
		}
		
		if exprs.is_empty() {
		    return Ok(first);
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
		
		Ok(res)
	}
	
	pub fn parse_expr(tokens: &mut Tokens) -> ParseResult<Self> {
	    enum Op { Add, Sub, Or }
		
		// <term>
		let first = Expr::parse_term(tokens)?;
		let mut terms: Vec<(Op, Expr)> = Vec::new();
		
		// { ('+' | '-' | 'or') <term> }
		loop {
			let op = match tokens.peek() {
				Some(Token::Plus)  => Op::Add,
				Some(Token::Minus) => Op::Sub,
				Some(Token::Or)    => Op::Or,
			    _ => break
			};
			tokens.next();
		    
		    let term = Expr::parse_term(tokens)?;
		    terms.push((op, term));
		}
		
		if terms.is_empty() {
		    return Ok(first);
		}
		
		let res = terms.into_iter()
		.fold(first, |acc, (op, base)| match op {
			Op::Add => Expr::Add(Box::new(acc), Box::new(base)),
			Op::Sub => Expr::Sub(Box::new(acc), Box::new(base)),
			Op::Or  => Expr::Or(Box::new(acc), Box::new(base)),
		});
		
		Ok(res)
	}
	
	fn parse_term(tokens: &mut Tokens) -> ParseResult<Self> {
	    enum Op { Mul, Div, Mod, And }
		
		// <fact>
		let first = Expr::parse_exp(tokens)?;
		let mut facts: Vec<(Op, Expr)> = Vec::new();
		
		// { ('*' | '/' | 'mod' | 'and') <fact> }
		loop {
			let op = match tokens.peek() {
				Some(Token::Star)   => Op::Mul,
				Some(Token::FSlash) => Op::Div,
				Some(Token::Mod)    => Op::Mod,
				Some(Token::And)    => Op::And,
			    _ => break
			};
			tokens.next();
		    
		    let fact = Expr::parse_exp(tokens)?;
		    facts.push((op, fact));
		}
		
		if facts.is_empty() {
		    return Ok(first);
		}
		
		let res = facts.into_iter()
		.fold(first, |acc, (op, base)| match op {
			Op::Mul => Expr::Mul(Box::new(acc), Box::new(base)),
			Op::Div => Expr::Div(Box::new(acc), Box::new(base)),
			Op::Mod => Expr::Mod(Box::new(acc), Box::new(base)),
			Op::And => Expr::And(Box::new(acc), Box::new(base)),
		});
		
		Ok(res)
	}
	
	fn parse_exp(tokens: &mut Tokens) -> ParseResult<Self> {
		// <exp>
		let first = Expr::parse_base(tokens)?;
		let mut exps = Vec::new();
		
		// { ('^') <exp> }
		loop {
			let op = match tokens.peek() {
				Some(Token::Caret) => {}
			    _ => break
			};
			tokens.next();
		    
		    let exp = Expr::parse_base(tokens)?;
		    exps.push(exp);
		}
		
		if exps.is_empty() {
		    return Ok(first);
		}
		
		let last = exps.pop().unwrap();
		let res = exps.into_iter()
		.rfold(last, |acc, base|
		    Expr::Exp(Box::new(base), Box::new(acc))
	    );
		
		Ok(Expr::Exp(Box::new(first), Box::new(res)))
	}
	
	fn parse_base(tokens: &mut Tokens) -> ParseResult<Self> {
		// check if there's an open parenthesis
		Ok(if tokens.peek() == Some(&Token::LParen) {
			tokens.next();
			
			let expr = Expr::parse(tokens)?;
			
			// make sure there's a closing parenthesis
			if tokens.next() != Some(Token::RParen) {
				return Err("closing parenthesis in subexpression");
			}
			
			Expr::Group(Box::new(expr))
		} else {
			// otherwise, treat it as a Term.
			Expr::Term(Term::parse(tokens)?)
		})
	}
	
	pub fn eval(&self, t: &Scope) -> EvalResult {
		match self {
			// 1
			Expr::Term(term) => Ok(term.eval(t)),
			Expr::Group(e) => Ok(e.eval(t)?),
			
			// 3
			Expr::Not(e) => match e.eval(t)? {
				Value::Bool(true) => Ok(Value::Bool(false)),
				Value::Bool(false) => Ok(Value::Bool(true)),
				_ => Err("tried NOTting non-boolean expression")
			}
			
			// 4
			Expr::Exp(base, exp) => match (base.eval(t)?, exp.eval(t)?) {
				(Value::Int(b), Value::Int(e)) => Ok(Value::from(b.pow(e as u32))),
				_ => Err("tried to get power of non-integer values")
			}
			
			// 5
			Expr::Mul(l, r) => match (l.eval(t)?, r.eval(t)?) {
				(Value::Int(l), Value::Int(r)) => Ok(Value::from(l * r)),
				_ => Err("tried multiplying non-integer values")
			}
			Expr::Div(l, r) => match (l.eval(t)?, r.eval(t)?) {
				(Value::Int(l), Value::Int(r)) => Ok(Value::from(l / r)),
				_ => Err("tried dividing non-integer values")
			}
			Expr::Mod(l, r) => match (l.eval(t)?, r.eval(t)?) {
				(Value::Int(l), Value::Int(r)) => Ok(Value::from((l % r + r) % r)),
				_ => Err("tried getting remainder of non-integer values")
			}
			Expr::And(l, r) => match (l.eval(t)?, r.eval(t)?) {
				(Value::Bool(l), Value::Bool(r)) => Ok(Value::from(l && r)),
				_ => Err("tried ANDing non-boolean values")
			}
			
			// 6
			Expr::Add(l, r) => match (l.eval(t)?, r.eval(t)?) {
				(Value::Int(l), Value::Int(r)) => Ok(Value::from(l + r)),
				_ => Err("tried adding non-integer values")
			}
			Expr::Sub(l, r) => match (l.eval(t)?, r.eval(t)?) {
				(Value::Int(l), Value::Int(r)) => Ok(Value::from(l - r)),
				_ => Err("tried subtracting non-integer values")
			}
			Expr::Or(l, r) => match (l.eval(t)?, r.eval(t)?) {
				(Value::Bool(l), Value::Bool(r)) => Ok(Value::from(l || r)),
				_ => Err("tried ORing non-boolean expressions")
			}
			
			// 7
			Expr::Eq(l, r) => Ok(Value::from(l.eval(t)? == r.eval(t)?)),
			Expr::Neq(l, r) => Ok(Value::from(l.eval(t)? != r.eval(t)?)),
			
			Expr::Lt(l, r) => match (l.eval(t)?, r.eval(t)?) {
				(Value::Int(l), Value::Int(r)) => Ok(Value::from(l < r)),
				_ => Err("tried comparing non-integer values")
			}
			Expr::Lte(l, r) => match (l.eval(t)?, r.eval(t)?) {
				(Value::Int(l), Value::Int(r)) => Ok(Value::from(l <= r)),
				_ => Err("tried comparing non-integer values")
			}
			
			Expr::Gt(l, r) => match (l.eval(t)?, r.eval(t)?) {
				(Value::Int(l), Value::Int(r)) => Ok(Value::from(l > r)),
				_ => Err("tried comparing non-integer values")
			}
			Expr::Gte(l, r) => match (l.eval(t)?, r.eval(t)?) {
				(Value::Int(l), Value::Int(r)) => Ok(Value::from(l >= r)),
				_ => Err("tried comparing non-integer values")
			}
		}
	}
}

impl From<Term> for Expr {
	fn from(f: Term) -> Self { Expr::Term(f) }
}
