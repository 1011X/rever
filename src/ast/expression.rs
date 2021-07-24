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

use std::fmt;
use std::error;

use super::*;

#[derive(Debug, Clone)]
pub enum BinOp {
	// precedence 4
	Exp,
	// precedence 5
	Mul, Div, Mod, And,
	// precedence 6
	Add, Sub, Or, //Xor,
	// precedence 7
	Eq, Ne, Lt, Gt, Le, Ge,
}

#[derive(Debug, Clone)]
pub enum Expr {
	// precedence 1
	Lit(Literal),
	LVal(LValue),
	Cast(Box<Self>, Type),
	
	// precedence 3
	Neg(Box<Self>),
	Not(Box<Self>),
	
	// binary op, precendeces 4-7
	BinOp(Box<Self>, BinOp, Box<Self>),
}

#[derive(Debug, Clone)]
pub enum ExprErr {
	// Inner parenthesized expression did not have terminating closing parenthesis.
	UnclosedSubexpr,
	
	// Expected l-value or literal.
	NotAtom(LValErr),
	
	// Malformed type expression in cast.
	BadCast(TypeErr),
}

impl fmt::Display for ExprErr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::UnclosedSubexpr =>
				f.write_str("`)` after subexpression"),
			
			Self::NotAtom(_) =>
				f.write_str("literal or l-value"),
			Self::BadCast(_) =>
				f.write_str("valid type expression in cast"),
		}
	}
}

impl error::Error for ExprErr {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		match self {
			Self::NotAtom(e) => Some(e),
			Self::BadCast(e) => Some(e),
			_ => None,
		}
	}
}

impl Parser<'_> {
	// rel  -> expr {(=|≠|<|>|≤|≥|in) expr}
	// expr -> term {(+|-|or) term}
	// term -> exp {(*|/|mod|and) exp}
	// exp  -> atom {^ atom}
	// atom -> ( expr )
	//      -> expr 'as' type
	//      -> factor
	pub fn parse_expr(&mut self) -> ParseResult<Expr> {
		// <term>
		let first = self.parse_expr_add()?;
		let mut exprs: Vec<(BinOp, Expr)> = Vec::new();
		
		// { ('=' | '!=' | '<' | '>' | '<=' | '>=') <expr> }
		loop {
			let op = match self.peek() {
				Some(Token::Eq)  => BinOp::Eq,
				Some(Token::Neq) => BinOp::Ne,
				Some(Token::Lt)  => BinOp::Lt,
				Some(Token::Gt)  => BinOp::Gt,
				Some(Token::Lte) => BinOp::Le,
				Some(Token::Gte) => BinOp::Ge,
			    _ => break
			};
			self.next();
			
			let expr = self.parse_expr_add()?;
			exprs.push((op, expr));
		}
		
		if exprs.is_empty() {
			return Ok(first);
		}
		
		let expr = exprs.into_iter()
			.fold(first, |acc, (op, base)| {
				Expr::BinOp(Box::new(acc), op, Box::new(base))
			});
		
		Ok(expr)
	}
	
	pub fn parse_expr_add(&mut self) -> ParseResult<Expr> {
		// <term>
		let first = self.parse_expr_mul()?;
		let mut terms: Vec<(BinOp, Expr)> = Vec::new();
		
		// { ('+' | '-' | 'or' | ':') <term> }
		loop {
			let op = match self.peek() {
				Some(Token::Plus)  => BinOp::Add,
				Some(Token::Minus) => BinOp::Sub,
				Some(Token::Or)    => BinOp::Or,
			    _ => break
			};
			self.next();
			
			let term = self.parse_expr_mul()?;
			terms.push((op, term));
		}
		
		if terms.is_empty() {
			return Ok(first);
		}
		
		let expr = terms.into_iter()
			.fold(first, |acc, (op, base)| {
				Expr::BinOp(Box::new(acc), op, Box::new(base))
			});
		
		Ok(expr)
	}
	
	pub fn parse_expr_mul(&mut self) -> ParseResult<Expr> {
		// <fact>
		let first = self.parse_expr_exp()?;
		let mut facts: Vec<(BinOp, Expr)> = Vec::new();
		
		// { ('*' | '/' | '%' | 'and') <fact> }
		loop {
			let op = match self.peek() {
				Some(Token::Star)    => BinOp::Mul,
				Some(Token::FSlash)  => BinOp::Div,
				Some(Token::Percent) => BinOp::Mod,
				Some(Token::And)     => BinOp::And,
				_ => break
			};
			self.next();
			
			let fact = self.parse_expr_exp()?;
			facts.push((op, fact));
		}
		
		if facts.is_empty() {
			return Ok(first);
		}
		
		let expr = facts.into_iter()
			.fold(first, |acc, (op, base)| {
				Expr::BinOp(Box::new(acc), op, Box::new(base))
			});
		
		Ok(expr)
	}
	
	pub fn parse_expr_exp(&mut self) -> ParseResult<Expr> {
		// <exp>
		let first = self.parse_expr_atom()?;
		let mut exps = Vec::new();
		
		// { ('^') <exp> }
		loop {
			let op = match self.peek() {
				Some(Token::Caret) => {}
				_ => break
			};
			self.next();
			
			let exp = self.parse_expr_atom()?;
			exps.push(exp);
		}
		
		if exps.is_empty() {
			return Ok(first);
		}
		
		let last = exps.pop().unwrap();
		let res = exps.into_iter().rfold(last, |acc, base| {
			Expr::BinOp(Box::new(base), BinOp::Exp, Box::new(acc))
		});
		
		Ok(Expr::BinOp(Box::new(first), BinOp::Exp, Box::new(res)))
	}
	
	pub fn parse_expr_atom(&mut self) -> ParseResult<Expr> {
		// check if there's an open parenthesis
		let mut expr = match self.peek() {
			Some(Token::LParen) => {
				self.next();
				
				let expr = self.parse_expr()?;
				
				// make sure there's a closing parenthesis
				self.expect(Token::RParen)
					.ok_or("`)` after subexpression")?;
					//.ok_or(ExprErr::UnclosedSubexpr)?;
				
				expr
			}
			
			Some(Token::VarIdent) => {
				Expr::LVal(self.parse_lval()?)
				//Expr::LVal(self.parse_lval().ok_or(ExprErr::NotAtom)?)
			}
			
			Some(_) => {
				Expr::Lit(self.parse_lit()?)
			}
			
			None => Err("`(`, l-value, or literal")?,
		};
		
		// check for `as` casting
		while self.peek() == Some(&Token::As) {
			self.next();
			
			let ty = self.parse_type()?;
			//.ok_or(ExprErr::BadCast)?
			
			expr = Expr::Cast(Box::new(expr), ty);
		}
		
		Ok(expr)
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
	fn eval(&self, t: &StackFrame) -> EvalResult<Value> {
		match self {
			Expr::Lit(lit) => lit.eval(t),
			Expr::LVal(lval) => lval.eval(t),
			
			Expr::Cast(e, typ) => match (typ, e.eval(t)?) {
				(Type::Unit, _) => Ok(Value::Nil),
				(Type::Int, Value::Uint(u))  => Ok(Value::Int(u as i64)),
				(Type::UInt, Value::Bool(b)) => Ok(Value::Uint(b as u64)),
				(Type::UInt, Value::Int(i))  => Ok(Value::Uint(i as u64)),
				(Type::Char, Value::Int(i)) => Ok(Value::Char(i as u8 as char)),
				(Type::Char, Value::Uint(i)) => Ok(Value::Char(i as u8 as char)),
				(Type::String, Value::Char(c)) => Ok(Value::String(c.to_string())),
				(typ, value) => panic!("tried casting {} to {:?}", value, typ),
			}
			
			Expr::Not(e) => match e.eval(t)? {
				Value::Bool(b) => Ok(Value::Bool(!b)),
				Value::Uint(n) => Ok(Value::Uint(!n)),
				Value::Int(n) => Ok(Value::Int(!n)),
				val => Err(EvalError::TypeMismatch {
					expected: Type::Bool,
					got: val.get_type(),
				})
			}
			
			Expr::Neg(e) => match e.eval(t)? {
				Value::Int(n) => Ok(Value::Int(n.wrapping_neg())),
				val => Err(EvalError::TypeMismatch {
					expected: Type::Int,
					got: val.get_type(),
				})
			}
			
			Expr::BinOp(left, op, right) => {
				let left = left.eval(t)?;
				let right = right.eval(t)?;
				
				match (op, left, right) {
					// 4
					(BinOp::Exp, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l.pow(r as u32))),
					
					// 5
					(BinOp::Mul, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l * r)),
					(BinOp::Div, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l / r)),
					(BinOp::Mod, Value::Uint(l), Value::Uint(r)) =>
						Ok(Value::from((l % r + r) % r)),
					(BinOp::And, Value::Bool(l), Value::Bool(r)) =>
						Ok(Value::from(l && r)),
					
					// 6
					(BinOp::Add, Value::Uint(l), Value::Uint(r)) =>
						Ok(Value::from(l + r)),
					(BinOp::Sub, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l - r)),
					(BinOp::Or, Value::Bool(l), Value::Bool(r)) =>
						Ok(Value::from(l || r)),
					/*
					(BinOp::Xor, Value::Bool(l), Value::Bool(r)) =>
						Ok(Value::from(l ^ r)),
					(BinOp::Xor, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l ^ r)),
					*/
					
					// 7
					(BinOp::Eq, l, r) =>
						Ok(Value::from(l == r)),
					(BinOp::Ne, l, r) =>
						Ok(Value::from(l != r)),
					(BinOp::Lt, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l < r)),
					(BinOp::Gt, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l > r)),
					(BinOp::Le, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l <= r)),
					(BinOp::Ge, Value::Int(l), Value::Int(r)) =>
						Ok(Value::from(l >= r)),
					
					
					(BinOp::Lt, Value::Uint(l), Value::Uint(r)) =>
						Ok(Value::from(l < r)),
					
					(op, left, right) =>
						panic!(
							"tried to do {:?} with types {:?} and {:?}: {:#?}",
							op,
							left.get_type(),
							right.get_type(),
							self
						),
				}
			}
		}
	}
}
