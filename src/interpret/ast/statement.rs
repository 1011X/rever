use crate::tokenize::Token;
use super::*;

#[derive(Debug, Clone)]
pub enum Statement {
	Skip,
	
	Var(String, Option<Type>, Expr, Vec<Statement>, Expr),
	//Let(String, Option<Type>, Expr),
	If(Expr, Vec<Statement>, Vec<Statement>, Expr),
	From(Expr, Vec<Statement>, Vec<Statement>, Expr),
	
	//Match(String, Vec<_, Vec<Statement>>),
	//FromVar(String, Expr, Vec<Statement>, Vec<Statement>, Expr),
	
	Do(String, Vec<Expr>),
	Undo(String, Vec<Expr>),
	
	//Not(LValue),
	//Neg(LValue),
	
	//RotLeft(LValue, Expr),
	//RotRight(LValue, Expr),
	
	Xor(LValue, Expr),
	Add(LValue, Expr),
	Sub(LValue, Expr),
	
	Swap(LValue, LValue),
	//CSwap(Factor, LValue, LValue),
	
}

use self::Statement::*;
impl Statement {
	pub fn invert(self) -> Self {
		match self {
			Var(name, typ, init, scope, dest) =>
				Var(name, typ, dest, scope, init),
			
			//RotLeft(l, v) => RotRight(l, v),
			//RotRight(l, v) => RotLeft(l, v),
			Add(l, v) => Sub(l, v),
			Sub(l, v) => Add(l, v),
			
			Do(p, args) => Undo(p, args),
			Undo(p, args) => Do(p, args),
			
			If(test, block, else_block, assert) =>
				If(assert, block, else_block, test),
			From(assert, do_block, loop_block, test) =>
				From(test, do_block, loop_block, assert),
			
			_ => self
		}
	}
	
	pub fn parse(tokens: &mut Tokens) -> ParseResult<Self> {
		let res = match tokens.peek() {
			// skip
			Some(Token::Skip) => {
				tokens.next();
				Ok(Skip)
			}
			
			// do
			Some(Token::Do) => {
				tokens.next();
				
				let name = match tokens.next() {
					Some(Token::Ident(name)) => name,
					_ => return Err("procedure name")
				};
				
				// TODO check for parentheses. if so, go into multiline mode
				
				// parse arg list
				let mut args = Vec::new();
				
				loop {
					match tokens.peek() {
						None | Some(Token::Newline) => break,
						Some(_) => {
							let arg = Expr::parse(tokens)?;
							args.push(arg);
						}
					}
				}
				
				Ok(Statement::Do(name, args))
			}
			
			// undo
			// And no, you can't merge `do` and `undo` by using `tok @ pattern`
			// and matching later because it gives a "cannot borrow `*tokens`
			// as mutable more than once at a time" error.
			Some(Token::Undo) => {
				tokens.next();
				
				let name = match tokens.next() {
					Some(Token::Ident(name)) => name,
					_ => return Err("procedure name")
				};
				
				// TODO check for parentheses. if so, go into multiline mode
				
				// parse arg list
				let mut args = Vec::new();
				
				loop {
					match tokens.peek() {
						None | Some(Token::Newline) => break,
						Some(_) => {
							let arg = Expr::parse(tokens)?;
							args.push(arg);
						}
					}
				}
				
				Ok(Statement::Undo(name, args))
			}
			
			// from-until
			Some(Token::From) => {
				tokens.next();
				
				// parse loop assertion
				let assert = Expr::parse(tokens)?;
				
				// ensure there's a newline afterwards
				if tokens.next() != Some(Token::Newline) {
					return Err("newline after from expression");
				}
				
				// parse the main loop block
				let mut main_block = Vec::new();
				loop {
					match tokens.peek() {
						Some(Token::Until) => {
							tokens.next();
							break;
						}
						Some(_) => {
							let stmt = Statement::parse(tokens)?;
							main_block.push(stmt);
						}
						None => return Err("a statement or `until`")
					}
				}
				
				// parse the `until` test expression
				let test = Expr::parse(tokens)?;
				
				// ensure there's a newline afterwards
				if tokens.next() != Some(Token::Newline) {
					return Err("newline after until expression");
				}
				
				// parse reverse loop block
				let mut back_block = Vec::new();
				loop {
					match tokens.peek() {
						Some(Token::End) => {
							tokens.next();
							break;
						}
						Some(_) => {
							let stmt = Statement::parse(tokens)?;
							back_block.push(stmt);
						}
						None => return Err("a statement or `end`")
					}
				}
				
				// TODO check for optional `from` keyword; i.e `end from`
				
				Ok(From(assert, main_block, back_block, test))
			}
			
			// var-drop
			Some(Token::Var) => {
				tokens.next();
				
				// get name
				let name = match tokens.next() {
					Some(Token::Ident(name)) => name,
					_ => return Err("variable name")
				};
				
				// get optional type
				let mut typ = None;
				
				if tokens.peek() == Some(&Token::Colon) {
					tokens.next();
					let t = Type::parse(tokens)?;
					typ = Some(t);
				}
				
				// check for assignment op
				if tokens.next() != Some(Token::Assign) {
					return Err("`:=`");
				}
				
				// get initialization expression
				let init = Expr::parse(tokens)?;
				
				// get newline
				if tokens.next() != Some(Token::Newline) {
					return Err("newline after variable declaration");
				}
				
				// get list of statements for which this variable is valid
				let mut block = Vec::new();
				loop {
					match tokens.peek() {
						Some(Token::Drop) => {
							tokens.next();
							break;
						}
						Some(_) => {
							let stmt = Statement::parse(tokens)?;
							block.push(stmt);
						}
						None => return Err("a statement or `drop`")
					}
				}
				
				// get deinit name
				match tokens.next() {
					Some(Token::Ident(n)) if *n == name => {}
					_ => return Err("same variable name as before")
				}
				
				// check for assignment op
				if tokens.next() != Some(Token::Assign) {
					return Err("`:=`");
				}
				
				// get deinit expression
				let drop = Expr::parse(tokens)?;
				
				Ok(Var(name, typ, init, block, drop))
			}
			
			// if-else
			Some(Token::If) => {
				tokens.next();
				
				// parse if condition
				let cond = Expr::parse(tokens)?;
				
				// ensure there's a newline afterwards
				if tokens.next() != Some(Token::Newline) {
					return Err("newline after if predicate");
				}
				
				// parse the main block
				let mut main_block = Vec::new();
				
				// if `else` or `fi` is found, end block.
				// TODO: allow if-statements to end with `end`, such that if
				// they do, the assertion becomes the same as the condition.
				loop {
					match tokens.peek() {
						Some(Token::Else) |
						Some(Token::Fi) => break,
						
						Some(_) => {
							let stmt = Statement::parse(tokens)?;
							main_block.push(stmt);
						}
						None => return Err("eof @ if main block")
					}
				}
				
				// parse else section
				let mut else_block = Vec::new();
				
				// saw `else`
				if tokens.peek() == Some(&Token::Else) {
					tokens.next();
					
					// TODO: have at least 1 statement after `else`.
					// TODO: remove newline requirement below?
					
					// ensure there's a newline afterwards
					if tokens.next() != Some(Token::Newline) {
						return Err("newline after else");
					}
					
					// parse else block
					loop {
						match tokens.peek() {
							Some(Token::Fi) => {
								tokens.next();
								break;
							}
							Some(_) => {
								let stmt = Statement::parse(tokens)?;
								else_block.push(stmt);
							}
							None => return Err("a statement or `fi`")
						}
					}
				}
				
				// consume `fi`
				tokens.next();
				
				// parse the `fi` assertion
				let assert = Expr::parse(tokens)?;
				
				// TODO check for optional `if` keyword; i.e `end if`
				
				Ok(If(cond, main_block, else_block, assert))
			}
			
			Some(_) =>
				if let Ok(lval) = LValue::parse(tokens) {
					match tokens.peek() {
						Some(Token::Assign) => {
							tokens.next();
							
						    let expr = Expr::parse(tokens)?;
						    Ok(Xor(lval, expr))
						}
						Some(Token::AddAssign) => {
							tokens.next();
							
						    let expr = Expr::parse(tokens)?;
						    Ok(Add(lval, expr))
						}
						Some(Token::SubAssign) => {
							tokens.next();
							
						    let expr = Expr::parse(tokens)?;
						    Ok(Sub(lval, expr))
						}
						
						Some(Token::Swap) => {
							tokens.next();
							
						    let rval = LValue::parse(tokens)?;
						    Ok(Swap(lval, rval))
						}
						
						Some(_) => Err("`:=`, `+=`, `-=`, or `<>`"),
						None => Err("modifying operator"),
					}
				} else {
					Err("a valid statement")
				}
			
			None => Err("a statement"),
		};
				
		// consume newline afterwards, if any
		if tokens.peek() == Some(&Token::Newline) {
			tokens.next();
		}
		
		res
	}
	
	/*
	pub fn eval(&self, t: &mut Scope) {
		match self {
			Var(id, _, init, block, dest) => {
				t.push((id.clone(), init.eval(t)));
				for stmt in block {
					stmt.eval(t);
				}
				let (final_id, final_val) = t.pop().unwrap();
				assert_eq!(*id, final_id);
				assert_eq!(final_val, dest.eval(t));
			}
			
			RotLeft(lval, fact) => match (lval.eval(t), fact.eval(t)) {
				(Value::Unsigned(l), Value::Unsigned(r)) =>
					*t.get_mut(&lval.id).unwrap() = Value::Unsigned(l.rotate_left(r as u32)),
				_ => panic!("tried to do something illegal"),
			}
			RotRight(lval, fact) => match (lval.eval(t), fact.eval(t)) {
				(Value::Unsigned(l), Value::Unsigned(r)) =>
					*t.get_mut(&lval.id).unwrap() = Value::Unsigned(l.rotate_right(r as u32)),
				_ => panic!("tried to do something illegal"),
			}
			
			Xor(lval, fact) => match (lval.eval(t), fact.eval(t)) {
				(Value::Unsigned(l), Value::Unsigned(r)) => {
					let pos = t.iter()
						.rposition(|var| var.0 == lval.id)
						.expect("variable name not found");
					t[pos].1 = Value::Unsigned(l ^ r);
				}
				_ => panic!("tried to do something illegal"),
			}
			
			Add(lval, fact) => match (lval.eval(t), fact.eval(t)) {
				(Value::Unsigned(l), Value::Unsigned(r)) => {
					let pos = t.iter()
						.rposition(|var| var.0 == lval.id)
						.expect("variable name not found");
					t[pos].1 = Value::Unsigned(l.wrapping_add(r));
				}
				_ => panic!("tried to do something illegal"),
			}
			Sub(lval, fact) => match (lval.eval(t), fact.eval(t)) {
				(Value::Unsigned(l), Value::Unsigned(r)) => {
					let pos = t.iter()
						.rposition(|var| var.0 == lval.id)
						.expect("variable name not found");
					t[pos].1 = Value::Unsigned(l.wrapping_sub(r));
				}
				_ => panic!("tried to do something illegal"),
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
			
			// TODO find a way to call procedures. maybe by adding a `call`
			// method to `Value`.
			Do(name, args) => {
				let vals: Vec<Value> = args.iter()
					.map(|arg| arg.eval(t))
					.collect();
				t.iter()
					.rfind(|var| var.0 == name.id)
					.unwrap()
					.1
					.call(vals, t);
			}
			Undo(name, args) => {
				let vals: Vec<Value> = args.iter()
					.map(|arg| arg.eval(t))
					.collect();
				t.iter()
					.rfind(|var| var.0 == name.id)
					.unwrap()
					.1
					.uncall(vals, t);
			}
			
			If(test, block, else_block, assert) => {
				match test.eval(t) {
					Value::Bool(true) => {
						for stmt in block {
							stmt.eval(t);
						}
						assert_eq!(assert.eval(t), Value::Bool(true));
					}
					Value::Bool(false) => {
						for stmt in else_block {
							stmt.eval(t);
						}
						assert_eq!(assert.eval(t), Value::Bool(false));
					}
					_ => panic!("tried to do something illegal")
				}
			}
			From(assert, do_block, loop_block, test) => {
				assert_eq!(assert.eval(t), Value::Bool(true));
				loop {
					for stmt in do_block {
						stmt.eval(t);
					}
					
					match test.eval(t) {
						Value::Bool(true) => break,
						Value::Bool(false) =>
							for stmt in loop_block {
								stmt.eval(t);
							}
						_ => panic!("tried to do something illegal")
					}
					
					assert_eq!(assert.eval(t), Value::Bool(false));
				}
			}
			
			_ => unreachable!()
		}
	}
	*/
}
