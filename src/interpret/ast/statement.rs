use crate::tokenize::Token;
use super::*;

#[derive(Debug, Clone)]
pub enum Statement {
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
	
	pub fn parse(mut tokens: &[Token]) -> ParseResult<Self> {
		match tokens.first() {
			// do/undo
			Some(stmt_tok @ Token::Do) | Some(stmt_tok @ Token::Undo) => {
				tokens = &tokens[1..];
				
				let name =
					if let Some(Token::Ident(n)) = tokens.first() {
						tokens = &tokens[1..];
						n.clone()
					} else {
						return Err(format!("expected procedure name"));
					};
				
				// TODO check for parentheses. if so, go into multiline mode
				
				// parse arg list
				let mut args = Vec::new();
				loop {
					match tokens.first() {
						None => break,
						Some(Token::Newline) => {
							tokens = &tokens[1..];
							break;
						}
						Some(_) => {
							let (arg, tx) = Expr::parse(tokens)?;
							tokens = tx;
							args.push(arg);
						}
					}
				}
				
				match stmt_tok {
					Token::Do   => Ok((Statement::Do(name, args), tokens)),
					Token::Undo => Ok((Statement::Undo(name, args), tokens)),
					_ => unreachable!()
				}
			}
			
			// from-until
			Some(Token::From) => {
				tokens = &tokens[1..];
				
				// parse loop assertion
				let (assert, mut tokens) = Expr::parse(tokens)?;
				
				// ensure there's a newline afterwards
				if tokens.first() != Some(&Token::Newline) {
					return Err(format!("expected newline after from expression {:?}", tokens));
				}
				tokens = &tokens[1..];
				
				// parse the main loop block
				let mut main_block = Vec::new();
				while tokens.first() != Some(&Token::Until) {
					let (stmt, t) = Statement::parse(tokens)?;
					main_block.push(stmt);
					tokens = t;
				}
				tokens = &tokens[1..];
				
				// parse the `until` test expression
				let (test, mut tokens) = Expr::parse(tokens)?;
				
				// ensure there's a newline afterwards
				if tokens.first() != Some(&Token::Newline) {
					return Err(format!("expected newline after until expression"));
				}
				tokens = &tokens[1..];
				
				// parse reverse loop block
				let mut back_block = Vec::new();
				while tokens.first() != Some(&Token::End) {
					let (stmt, t) = Statement::parse(tokens)?;
					back_block.push(stmt);
					tokens = t;
				}
				tokens = &tokens[1..];
				
				// TODO check for optional `from` keyword; i.e `end from`
				
				// consume newline afterwards, if any
				if tokens.first() == Some(&Token::Newline) {
					tokens = &tokens[1..];
				}
				
				Ok((From(assert, main_block, back_block, test), tokens))
			}
			
			// var-drop
			Some(Token::Var) => {
				tokens = &tokens[1..];
				
				// get name
				let name = match tokens.first() {
					Some(Token::Ident(name)) => name.clone(),
					Some(_) => return Err(format!("expected name @ var init")),
					None => return Err(format!("eof @ var name init")),
				};
				tokens = &tokens[1..];
				
				// get optional type
				let mut typ = None;
				if let Some(Token::Colon) = tokens.first() {
					tokens = &tokens[1..];
					let (t, tx) = Type::parse(tokens)?;
					typ = Some(t);
					tokens = tx;
				}
				
				// check for assignment op
				match tokens.first() {
					Some(Token::Assign) => {tokens = &tokens[1..]}
					_ => return Err(format!("expected assignment op @ var init"))
				}
				
				// get initialization expression
				let (init, tx) = Expr::parse(tokens)?;
				tokens = tx;
				
				// get newline
				match tokens.first() {
					Some(Token::Newline) => {tokens = &tokens[1..]}
					_ => return Err(format!("expected newline after var init"))
				}
				
				// get list of statements for which this variable is valid
				let mut block = Vec::new();
				while tokens.first() != Some(&Token::Drop) {
					let (stmt, tx) = Statement::parse(tokens)?;
					block.push(stmt);
					tokens = tx;
				}
				tokens = &tokens[1..];
				
				// get deinit name
				match tokens.first() {
					Some(Token::Ident(n)) if *n == name =>
						tokens = &tokens[1..],
					Some(Token::Ident(e)) =>
						return Err(format!("expected {:?}, got {:?}", name, e)),
					Some(_) =>
						return Err(format!("expected name @ drop")),
					None =>
						return Err(format!("eof @ var drop")),
				}
				
				// check for assignment op
				match tokens.first() {
					Some(Token::Assign) => {tokens = &tokens[1..]}
					_ => return Err(format!("expected assignment op @ drop"))
				}
				
				// get deinit expression
				let (drop, tx) = Expr::parse(tokens)?;
				tokens = tx;
				
				// get newline, if any
				if tokens.first() == Some(&Token::Newline) {
					 tokens = &tokens[1..];
				}
				
				Ok((Var(name, typ, init, block, drop), tokens))
			}
			
			// if-else
			Some(Token::If) => {
				tokens = &tokens[1..];
				
				// parse if condition
				let (cond, t) = Expr::parse(tokens)?;
				tokens = t;
				
				// ensure there's a newline afterwards
				if tokens.first() != Some(&Token::Newline) {
					return Err(format!("expected newline after if expression {:?}", tokens));
				}
				tokens = &tokens[1..];
				
				// parse the main block
				let mut main_block = Vec::new();
				
				while tokens.first() != Some(&Token::Else) && tokens.first() != Some(&Token::Fi) {
					let (stmt, t) = Statement::parse(tokens)?;
					main_block.push(stmt);
					tokens = t;
				}
					
				// TODO: allow if-statements to end with `end`, such that if
				// they do, the assertion becomes the same as the condition.
				
				// parse else section
				let mut else_block = Vec::new();
				
				if tokens.first() == Some(&Token::Else) {
					tokens = &tokens[1..];
					
					// TODO: have at least 1 statement after `else`.
					// TODO: remove newline requirement below?
					
					// ensure there's a newline afterwards
					if tokens.first() != Some(&Token::Newline) {
						return Err(format!("expected newline after else {:?}", tokens));
					}
					tokens = &tokens[1..];
					
					// parse the else block
					while tokens.first() != Some(&Token::Fi) {
						let (stmt, t) = Statement::parse(tokens)?;
						else_block.push(stmt);
						tokens = t;
					}
				}
				tokens = &tokens[1..];
				
				// parse the `fi` assertion
				let (assert, mut tokens) = Expr::parse(tokens)?;
				
				// TODO check for optional `if` keyword; i.e `end if`
				
				// consume newline afterwards, if any
				if tokens.first() == Some(&Token::Newline) {
					tokens = &tokens[1..];
				}
				
				Ok((If(cond, main_block, else_block, assert), tokens))
			}
			
			Some(token) =>
				if let Ok((lval, tokens)) = LValue::parse(tokens) {
					match tokens.first() {
						Some(Token::Assign) | Some(Token::Add) | Some(Token::Sub) => {
							unimplemented!()
						}
						
						Some(Token::Swap) => {
						    let (rval, t) = LValue::parse(tokens)?;
						    Ok((Swap(lval, rval), t))
						}
						
						Some(_) => {
						    unimplemented!()
						}
						
						None => Err(format!("eof @ lval statement op")),
					}
				} else {
					Err(format!("unrecognized statement: {:?}; {:?}", token, &tokens[1..]))
				}
			
			None =>
				Err(format!("eof @ statement")),
		}
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
