use super::*;

#[derive(Debug, Clone)]
pub enum Statement {
	Skip,
	
	//Not(LValue),
	//Neg(LValue),
	
	RotLeft(LValue, Expr),
	RotRight(LValue, Expr),
	
	Xor(LValue, Expr),
	Add(LValue, Expr),
	Sub(LValue, Expr),
	
	Swap(LValue, LValue),
	//CSwap(Factor, LValue, LValue),
	
	Do(String, Vec<Expr>),
	Undo(String, Vec<Expr>),
	
	Var(String, Type, Expr, Vec<Statement>, Expr),
	If(Expr, Vec<Statement>, Vec<Statement>, Expr),
	From(Expr, Vec<Statement>, Vec<Statement>, Expr),
	//Match(String, Vec<_, Vec<Statement>>),
}

impl Statement {
	pub fn invert(self) -> Self {
		use self::Statement::*;
		match self {
			Skip => self,
			RotLeft(l, v) => RotRight(l, v),
			RotRight(l, v) => RotLeft(l, v),
			Add(l, v) => Sub(l, v),
			Sub(l, v) => Add(l, v),
			Xor(..) => self,
			Swap(..) => self,
			
			Do(p, args) => Undo(p, args),
			Undo(p, args) => Do(p, args),
			
			Var(name, typ, init, scope, dest) =>
				Var(name, typ, dest, scope, init),
			If(test, block, else_block, assert) =>
				If(assert, block, else_block, test),
			From(assert, do_block, loop_block, test) =>
				From(test, do_block, loop_block, assert),
		}
	}
}

impl From<crate::ast::Statement> for Statement {
	fn from(v: crate::ast::Statement) -> Self {
		use crate::ast::Statement as Stmt;
		match v {
			Stmt::Skip => Statement::Skip,
			Stmt::RotLeft(lval, expr) => Statement::RotLeft(lval.into(), expr.into()),
			Stmt::RotRight(lval, expr) => Statement::RotRight(lval.into(), expr.into()),
			Stmt::Add(lval, expr) => Statement::Add(lval.into(), expr.into()),
			Stmt::Sub(lval, expr) => Statement::Sub(lval.into(), expr.into()),
			Stmt::Xor(lval, expr) => Statement::Xor(lval.into(), expr.into()),
			Stmt::Swap(l, r) => Statement::Swap(l.into(), r.into()),
			
			_ => unimplemented!()
			/*
			Stmt::Do(p, args) => Statement::Do(p, args.into_iter().map(Expr::from).collect()),
			Stmt::Undo(p, args) => Statement::Undo(p, args.into_iter().map(Expr::from).collect()),
			Stmt::Var(n, t, s, b, e) =>
				Statement::Var(n, t.into(), s.into(), b.into_iter().map(Expr::from).collect(), e.into()),
			Stmt::If(e, b, eb, a) =>
				Statement::If(
					e.into(),
					b.into_iter().map(Expr::from).collect(),
					eb.into_iter().map(Expr::from).collect(),
					a.into()
				),
			Stmt::From(a, d, l, e) =>
				Statement::From(
					a.into(),
					d.into_iter().map(Expr::from).collect(),
					l.into_iter().map(Expr::from).collect(),
					e.into()
				),
			*/
		}
	}
}

impl Statement {
	pub fn eval(&self, t: &mut Scope, m: &Module) -> EvalResult {
		use self::Statement::*;
		
		match self {
			Skip => {}
			
			Var(id, _, init, block, dest) => {
				let init = init.eval(t)?;
				t.push((id.clone(), init));
				
				for stmt in block {
					stmt.eval(t, m)?;
				}
				
				let (final_id, final_val) = t.pop().unwrap();
				assert_eq!(*id, final_id);
				assert_eq!(final_val, dest.eval(t)?);
			}
			
			Xor(lval, expr) => match (lval.eval(t), expr.eval(t)?) {
				(Value::Int(l), Value::Int(r)) => {
					let pos = t.iter()
						.rposition(|var| var.0 == lval.id)
						.ok_or("variable name not found")?;
					t[pos].1 = Value::Int(l ^ r);
				}
				_ => return Err("tried to do something illegal")
			}
			
			Add(lval, expr) => match (lval.eval(t), expr.eval(t)?) {
				(Value::Int(l), Value::Int(r)) => {
					let pos = t.iter()
						.rposition(|var| var.0 == lval.id)
						.expect("variable name not found");
					t[pos].1 = Value::Int(l.wrapping_add(r));
				}
				_ => return Err("tried to do something illegal")
			}
			Sub(lval, expr) => match (lval.eval(t), expr.eval(t)?) {
				(Value::Int(l), Value::Int(r)) => {
					let pos = t.iter()
						.rposition(|var| var.0 == lval.id)
						.expect("variable name not found");
					t[pos].1 = Value::Int(l.wrapping_sub(r));
				}
				_ => return Err("tried to do something illegal")
			}
			
			RotLeft(lval, expr) => match (lval.eval(t), expr.eval(t)?) {
				(Value::Int(l), Value::Int(r)) => {
					let pos = t.iter()
						.rposition(|var| var.0 == lval.id)
						.expect("variable name not found");
					t[pos].1 = Value::Int(l.rotate_left(r as u32));
				}
				_ => return Err("tried to do something illegal")
			}
			RotRight(lval, expr) => match (lval.eval(t), expr.eval(t)?) {
				(Value::Int(l), Value::Int(r)) => {
					let pos = t.iter()
						.rposition(|var| var.0 == lval.id)
						.expect("variable name not found");
					t[pos].1 = Value::Int(l.rotate_right(r as u32));
				}
				_ => return Err("tried to do something illegal")
			}
			
			Swap(left, right) => {
				let left_idx = t.iter()
					.rposition(|var| var.0 == left.id)
					.expect("variable name not found");
				let right_idx = t.iter()
					.rposition(|var| var.0 == right.id)
					.expect("variable name not found");
				
				// ensure types are the same
				assert_eq!(
					t[left_idx].1.get_type(),
					t[right_idx].1.get_type(),
					"tried to swap variables with different types"
				);
				
				// get names of values being swapped for later
				let left_name = t[left_idx].0.clone();
				let right_name = t[right_idx].0.clone();
				
				t.swap(left_idx, right_idx);
				
				// rectify names
				t[left_idx].0 = left_name;
				t[right_idx].0 = right_name;
			}
			
			// TODO find a way to call procedures.
			/* Clearly we need more info here. Eventually we'll need to store
			the "path" of the current module with the procedure, but for now
			just having the items of the current module is good enough. So find
			a way to make that available. */
			Do(callee_name, args) => {
				let mut vals = Vec::new();
				for arg in args {
					vals.push(arg.eval(t)?);
				}
				for (name, item) in &m.0 {
					if let Item::Proc(pr) = item {
						if name == callee_name {
							pr.call(vals, m);
							break;
						}
					} else if let Item::InternProc(pr, _) = item {
						if name == callee_name {
							pr(vals.into_boxed_slice());
							break;
						}
					}
				}
			}
			Undo(callee_name, args) => {
				let mut vals = Vec::new();
				for arg in args {
					vals.push(arg.eval(t)?);
				}
				for (name, item) in &m.0 {
					if let Item::Proc(pr) = item {
						if name == callee_name {
							pr.call(vals, m);
							break;
						}
					} else if let Item::InternProc(_, pr) = item {
						if name == callee_name {
							pr(vals.into_boxed_slice());
							break;
						}
					}
				}
			}
			
			If(test, block, else_block, assert) => {
				match test.eval(t)? {
					Value::Bool(true) => {
						for stmt in block {
							stmt.eval(t, m)?;
						}
						assert_eq!(assert.eval(t)?, Value::Bool(true));
					}
					Value::Bool(false) => {
						for stmt in else_block {
							stmt.eval(t, m)?;
						}
						assert_eq!(assert.eval(t)?, Value::Bool(false));
					}
					_ => return Err("tried to do something illegal")
				}
			}
			
			From(assert, do_block, loop_block, test) => {
				assert_eq!(assert.eval(t)?, Value::Bool(true));
				loop {
					for stmt in do_block {
						stmt.eval(t, m)?;
					}
					
					match test.eval(t)? {
						Value::Bool(true) => break,
						Value::Bool(false) =>
							for stmt in loop_block {
								stmt.eval(t, m)?;
							}
						_ => panic!("tried to do something illegal")
					}
					
					assert_eq!(assert.eval(t)?, Value::Bool(false));
				}
			}
		}
		
		Ok(Value::Nil)
	}
}
