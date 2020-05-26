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
	// precedence 4
	Exp,
	// precedence 5
	Mul, Div, Mod, And,
	// precedence 6
	Add, Sub, Or, Xor,
	// precedence 7
	Eq, Ne, Lt, Gt, Le, Ge,
}

#[derive(Debug, Clone)]
pub enum Expr {
	// precedence 1
	Term(Term),
	Group(Box<Expr>),
	Cast(Box<Expr>, Type),
	
	// precedence 3
	Not(Box<Expr>),
	
	// binary op, precendeces 4-7
	BinOp(BinOp, Box<Expr>, Box<Expr>),
	
	// secret precendece 8
	If(Box<Expr>, Box<Expr>, Box<Expr>),
	
	// secret precedence 9
	Let(String, Option<Type>, Box<Expr>, Box<Expr>),
}

impl From<crate::ast::Expr> for Expr {
	fn from(v: crate::ast::Expr) -> Self {
		unimplemented!()
	}
}

// rel  -> expr {(=|≠|<|>|≤|≥|in) expr}
// expr -> term {(+|-|or) term}
// term -> exp {(*|/|mod|and) exp}
// exp  -> atom {^ atom}
// atom -> ( expr )
//      -> expr 'as' type
//      -> factor
impl Expr {
	pub fn eval(&self, t: &Scope) -> EvalResult {
		match self {
			// 1
			Expr::Term(term) => Ok(term.eval(t)),
			Expr::Group(e) => Ok(e.eval(t)?),
			Expr::Cast(e, typ) => match (typ, e.eval(t)?) {
				(Type::Unit, _) => Ok(Value::Nil),
				(Type::Int, Value::Uint(v)) => Ok(Value::Int(v as i64)),
				(Type::UInt, Value::Int(v)) => Ok(Value::Uint(v as u64)),
				_ => unimplemented!()
			}
			
			// 3
			Expr::Not(e) => match e.eval(t)? {
				Value::Bool(true) => Ok(Value::Bool(false)),
				Value::Bool(false) => Ok(Value::Bool(true)),
				_ => Err("tried NOTting non-boolean value")
			}
			
			// 4 - 7
			Expr::BinOp(op, left, right) => {
				let left = left.eval(t)?;
				let right = right.eval(t)?;
				
				match (op, left, right) {
					// 4
					(BinOp::Exp, Value::Int(b), Value::Int(e)) =>
						Ok(Value::from(b.pow(e as u32))),
					(BinOp::Exp, _, _) =>
						Err("tried to get power of non-integer values"),
					
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
					(BinOp::Xor, Value::Bool(l), Value::Bool(r)) =>
						Ok(Value::from(l ^ r)),
					(BinOp::Xor, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l ^ r)),
					(BinOp::Xor, _, _) =>
						Err("tried XORing non-boolean or non-integer values"),
					
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
	
	pub fn get_type(&self) -> Option<Type> {
		unimplemented!()
	}
}

impl From<Term> for Expr {
	fn from(f: Term) -> Self { Expr::Term(f) }
}
