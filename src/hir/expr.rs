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
  + No short-circuiting; like Pascal.
  + Short-circuiting can be achieved using `and` and `or`.

TODO:
+ Add precedences 2, 
*/

use super::*;

#[derive(Debug, Clone)]
pub enum BinOp {
	Exp,
	// precedence 5
	Mul, Div, Mod, And,
	// precedence 6
	Add, Sub, Or, //Xor,
	// precedence 7
	Eq, Ne, Lt, Gt, Le, Ge,
}

impl From<ast::BinOp> for BinOp {
	fn from(v: ast::BinOp) -> Self {
		match v {
			ast::BinOp::Exp => BinOp::Exp,
			ast::BinOp::Mul => BinOp::Mul,
			ast::BinOp::Div => BinOp::Div,
			ast::BinOp::Mod => BinOp::Mod,
			ast::BinOp::And => BinOp::And,
			ast::BinOp::Add => BinOp::Add,
			ast::BinOp::Sub => BinOp::Sub,
			ast::BinOp::Or  => BinOp::Or,
			ast::BinOp::Eq  => BinOp::Eq,
			ast::BinOp::Ne  => BinOp::Ne,
			ast::BinOp::Lt  => BinOp::Lt,
			ast::BinOp::Gt  => BinOp::Gt,
			ast::BinOp::Le  => BinOp::Le,
			ast::BinOp::Ge  => BinOp::Ge,
		}
	}
}

#[derive(Debug, Clone)]
pub enum Expr {
	Term(Term),
	Cast(Box<Expr>, Type),
	
	Not(Box<Expr>),
	
	/*
	Exp(Box<Expr>, Box<Expr>),
	Product(Vec<Expr>),
	Sum(Vec<Expr>),
	Compare(Box<Expr>, Box<Expr>),
	All(Vec<Expr>),
	Any(Vec<Expr>),
	*/
	BinOp(Box<Expr>, BinOp, Box<Expr>),
	
	If(Box<Expr>, Box<Expr>, Box<Expr>),
	Let(String, Type, Box<Expr>, Box<Expr>),
}

impl Expr {
	pub fn get_type(&self) -> Type {
		match self {
			Expr::Term(term) => term.get_type(),
			_ => todo!()
		}
	}
}

impl From<ast::Expr> for Expr {
	fn from(v: ast::Expr) -> Self {
		match v {
			ast::Expr::Term(term) => Expr::Term(term.into()),
			ast::Expr::Cast(expr, kind) => Expr::Cast(
				Box::new(expr.0.into()),
				kind.0.into()
			),
			
			ast::Expr::Not(e) => Expr::Not(Box::new((*e).0.into())),
			
			ast::Expr::BinOp(left, op, right) => Expr::BinOp(
				Box::new(left.0.into()),
				op.into(),
				Box::new(right.0.into()),
			),
			
			_ => todo!()
		}
	}
}

// rel  -> expr {(=|≠|<|>|≤|≥|in) expr}
// expr -> term {(+|-|or) term}
// term -> exp {(*|/|mod|and) exp}
// exp  -> atom {^ atom}
// atom -> ( expr )
//      -> expr 'as' type
//      -> factor
impl Eval for Expr {
	fn eval(&self, t: &Scope) -> EvalResult {
		match self {
			Expr::Term(term) => Ok(term.eval(t)?),
			
			Expr::Cast(e, typ) => match (typ, e.eval(t)?) {
				(Type::Unit, _) => Ok(Value::Nil),
				(Type::Int, Value::Uint(u))  => Ok(Value::Int(u as i64)),
				(Type::UInt, Value::Bool(b)) => Ok(Value::Uint(b as u64)),
				(Type::UInt, Value::Int(i))  => Ok(Value::Uint(i as u64)),
				_ => unimplemented!()
			}
			
			Expr::Not(e) => match e.eval(t)? {
				Value::Bool(b) => Ok(Value::Bool(!b)),
				Value::Uint(n) => Ok(Value::Uint(!n)),
				Value::Int(n) => Ok(Value::Int(!n)),
				_ => Err("tried NOTting non-boolean or non-integer value")
			}
			
			/*
			Expr::Exp(base, exp) => {
				let base = base.eval(t)?;
				let exp = exp.eval(t)?;
				
				match (base, exp) {
					(Value::Int(base), Value::Uint(exp)) =>
						Ok(Value::Int(base.pow(exp as u32))),
					(Value::Uint(base), Value::Uint(exp)) =>
						Ok(Value::Uint(base.pow(exp as u32))),
					_ => Err("tried to get power of non-integer values")
				}
			}
			
			Expr::Product(terms) => {
				// there *should* be at least one term in the vector
				let first = terms.remove(0);
				
				for term in terms {
					
				}
			}
			*/
			
			Expr::BinOp(left, op, right) => {
				let left = left.eval(t)?;
				let right = right.eval(t)?;
				
				match (op, left, right) {
					// 5
					(BinOp::Mul, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l * r)),
					(BinOp::Mul, _, _) =>
						Err("tried multiplying non-integer values"),
					(BinOp::Div, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l / r)),
					(BinOp::Div, _, _) =>
						Err("tried dividing non-integer values"),
					(BinOp::Mod, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from((l % r + r) % r)),
					(BinOp::Mod, _, _) =>
						Err("tried getting remainder of non-integer values"),
					(BinOp::And, Value::Bool(l), Value::Bool(r)) =>
						Ok(Value::from(l && r)),
					(BinOp::And, _, _) =>
						Err("tried ANDing non-boolean values"),
					
					// 6
					(BinOp::Add, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l + r)),
					(BinOp::Add, _, _) =>
						Err("tried adding non-integer values"),
					(BinOp::Sub, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l - r)),
					(BinOp::Sub, _, _) =>
						Err("tried subtracting non-integer values"),
					(BinOp::Or, Value::Bool(l), Value::Bool(r)) =>
						Ok(Value::from(l || r)),
					(BinOp::Or, _, _) =>
						Err("tried ORing non-boolean values"),
					/*
					(BinOp::Xor, Value::Bool(l), Value::Bool(r)) =>
						Ok(Value::from(l ^ r)),
					(BinOp::Xor, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l ^ r)),
					(BinOp::Xor, _, _) =>
						Err("tried XORing non-boolean or non-integer values"),
					*/
					
					// 7
					(BinOp::Eq, l, r) =>
						Ok(Value::from(l == r)),
					(BinOp::Ne, l, r) =>
						Ok(Value::from(l != r)),
					(BinOp::Lt, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l < r)),
					(BinOp::Lt, _, _) =>
						Err("tried comparing non-integer values"),
					(BinOp::Gt, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l > r)),
					(BinOp::Gt, _, _) =>
						Err("tried comparing non-integer values"),
					(BinOp::Le, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l <= r)),
					(BinOp::Le, _, _) =>
						Err("tried comparing non-integer values"),
					(BinOp::Ge, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l >= r)),
					(BinOp::Ge, _, _) =>
						Err("tried comparing non-integer values"),
					
					_ => todo!()
				}
			}
			
			Expr::If(test, expr, else_expr) => {
				if test.eval(t)? == Value::Bool(true) {
					expr.eval(t)
				} else {
					else_expr.eval(t)
				}
			}
			
			Expr::Let(name, _, val, scope) => {
				let val = val.eval(t)?;
				let mut t_copy = t.clone();
				t_copy.push((name.clone(), val));
				scope.eval(&t_copy)
			}
		}
	}
}

impl From<Term> for Expr {
	fn from(f: Term) -> Self { Expr::Term(f) }
}
