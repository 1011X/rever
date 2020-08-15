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
	
	Let(String, Type, Expr, Vec<Statement>, Expr),
	If(Expr, Vec<Statement>, Vec<Statement>, Expr),
	From(Expr, Vec<Statement>, Vec<Statement>, Expr),
	//FromLet(String, Expr, Vec<Statement>, Vec<Statement>, Expr),
	//Match(String, Vec<_, Vec<Statement>>),
	//For(String, Expr, Vec<Statement>),
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
			
			Let(n, t, init, s, dest) =>
				Let(n, t, dest, s, init),
			If(test, b, eb, assert) =>
				If(assert, b, eb, test),
			From(assert, b, lb, test) =>
				From(test, b, lb, assert),
		}
	}
}

impl Parser<'_> {
	pub fn parse_stmt(&mut self) -> ParseResult<Statement> {
		let stmt = match self.peek().ok_or("a statement")? {
			// skip
			// TODO use this keyword as a prefix to comment out statements?
			Token::Skip => {
				self.next();
				Statement::Skip
			}
			
			/* do-call syntax accepts three forms:
			   + `do something`
			   + `do something: some, args` (1 arg min) TODO
			   + `do something(
			          multiline,
			          args
			      )` (0 arg min) TODO
			   also has special syntax like:
			   + do something: var new_var, drop used_var
			*/
			Token::Do => {
				self.next();
				
				let name = self.expect_ident()
					.ok_or("procedure name after `do`")?;
				
				// TODO check for parentheses. if so, go into multiline mode
				let mut args = Vec::new();
				
				if self.expect(Token::Newline).is_some() {
					// do nothing
				} else if self.expect(Token::Colon).is_some() {
					// TODO check for newline, in case expression is missing
					let expr = self.parse_expr()?;
					args.push(expr);
					
					loop {
						match self.peek() {
							Some(Token::Newline)
							| None =>
								break,
							Some(Token::Comma) => {
								self.next();
								// TODO check for "substatements" first.
								// E.g. `var file` or `drop buf` in args.
								args.push(self.parse_expr()?);
							}
							_ => Err("`,` or newline")?,
						}
					}
				} else if self.expect(Token::LParen).is_some() {
					unimplemented!();
				} else {
					Err("`:`, or newline")?;
				};
				
				Statement::Do(name, args)
			}
			
			// undo
			// accepts same forms as `do`.
			// And no, you can't merge the `do` and `undo` branches by using
			// `tok @ pattern` and matching at the end because it gives a
			// "cannot borrow `*tokens` as mutable more than once at a time"
			// error (as of rustc v1.42.0).
			Token::Undo => {
				self.next();
				
				let name = self.expect_ident()
					.ok_or("procedure name after `undo`")?;
				
				// TODO check for parentheses. if so, go into multiline mode
				let mut args = Vec::new();
				
				if self.expect(Token::Newline).is_some() {
					// do nothing
				} else if self.expect(Token::Colon).is_some() {
					// TODO check for newline, in case expression is missing
					let expr = self.parse_expr()?;
					args.push(expr);
					
					loop {
						match self.peek() {
							Some(Token::Newline)
							| None => 
								break,
							Some(Token::Comma) => {
								self.next();
								args.push(self.parse_expr()?);
							}
							_ => Err("`,` or newline")?,
						}
					}
				} else if self.expect(Token::LParen).is_some() {
					unimplemented!();
				} else {
					Err("`:`, or newline")?;
				};
				
				Statement::Undo(name, args)
			}
			
			// from-until
			Token::From => {
				self.next();
				
				// parse loop assertion
				let assert = self.parse_expr()?;
				
				self.expect(Token::Newline)
					.ok_or("newline after `from` assertion")?;
				
				// eat empty lines
				self.skip_newlines();
				
				// parse the main loop block
				let mut main_block = Vec::new();
				loop {
					match self.peek() {
						Some(Token::Until) => break,
						Some(_) => main_block.push(self.parse_stmt()?),
						None => Err("a statement or `until`")?,
					}
				}
				self.next();
				
				// parse the `until` test expression
				let test = self.parse_expr()?;
				
				self.expect(Token::Newline)
					.ok_or("newline after `until` expression")?;
				
				self.skip_newlines();
				
				// parse reverse loop block
				let mut back_block = Vec::new();
				loop {
					match self.peek() {
						Some(Token::Loop) => break,
						Some(_) => back_block.push(self.parse_stmt()?),
						None => Err("a statement or `loop`")?,
					}
				}
				self.next();
				
				Statement::From(assert, main_block, back_block, test)
			}
			
			// var-drop
			Token::Var => {
				self.next();
				
				// get name
				let name = self.expect_ident()
					.ok_or("name in variable declaration")?;
				
				// get optional type
				let typ = match self.expect(Token::Colon) {
					Some(_) => self.parse_type()?,
					None => Type::Infer,
				};
				
				// check for assignment op
				self.expect(Token::Assign)
					.ok_or("`:=` in variable declaration")?;
				
				// get initialization expression
				let init = self.parse_expr()?;
				
				self.expect(Token::Newline)
					.ok_or("newline after variable declaration")?;
				
				// eat empty lines
				self.skip_newlines();
				
				// get list of statements for which this variable is valid
				let mut block = Vec::new();
				loop {
					match self.peek() {
						Some(Token::Drop) => break,
						Some(_) => block.push(self.parse_stmt()?),
						None => Err("a statement or `drop`")?,
					}
				}
				self.next();
				
				// assert name
				let drop_name = self.expect_ident()
					.ok_or("name after `drop`")?;
				
				if drop_name != name {
					Err("same variable name as before")?;
				}
				
				// get optional deinit value
				let drop = match self.expect(Token::Assign) {
					Some(_) => self.parse_expr()?,
					None => init.clone(),
				};
				
				Statement::Let(name, typ, init, block, drop)
			}
			
			// if-else
			Token::If => {
				self.next();
				
				// parse if condition
				let cond = self.parse_expr()?;
				
				self.expect(Token::Newline)
					.ok_or("newline after `if` predicate")?;
				
				// parse the main block
				let mut main_block = Vec::new();
				loop {
					match self.peek() {
						Some(Token::Else)
						| Some(Token::Fi) => break,
						Some(_) => main_block.push(self.parse_stmt()?),
						None => Err("a statement, `else`, or `fi`")?,
					}
				}
				//self.next();
				
				// parse else section
				let mut else_block = Vec::new();
				
				// saw `else`
				if self.expect(Token::Else).is_some() {
					if self.expect(Token::Newline).is_some() {
						// parse a block
						loop {
							match self.peek() {
								Some(Token::Fi) => break,
								Some(_) => else_block.push(self.parse_stmt()?),
								None => Err("a statement or `fi`")?,
							}
						}
					} else if self.peek() == Some(&Token::If) {
						// check if it's a single `if` statement. this allows
						// "embedding" of chained `if` statements.
						else_block.push(self.parse_stmt()?);
					} else {
						Err("chaining `if` or a newline")?;
					}
				}
				
				// expect ending `fi`
				let fi = self.expect(Token::Fi)
					.ok_or("`fi` to finish `if` statement")?;
				
				// parse `fi` assertion, if any
				let assert = match self.peek() {
					Some(Token::Newline) => cond.clone(),
					Some(_) => self.parse_expr()?,
					None => Err("a newline or expression after `fi`")?,
				};
				
				Statement::If(cond, main_block, else_block, assert)
			}
			
			Token::Ident => {
				let lval = self.parse_lval()?;
				
				match self.peek().ok_or("modifying operator")? {
					Token::Assign => {
						self.next();
						let expr = self.parse_expr()?;
					    Statement::Xor(lval, expr)
					}
					Token::AddAssign => {
						self.next();
						let expr = self.parse_expr()?;
					    Statement::Add(lval, expr)
					}
					Token::SubAssign => {
						self.next();
						let expr = self.parse_expr()?;
					    Statement::Sub(lval, expr)
					}
					
					Token::Rol => {
						self.next();
						let expr = self.parse_expr()?;
					    Statement::RotLeft(lval, expr)
					}
					Token::Ror => {
						self.next();
						let expr = self.parse_expr()?;
					    Statement::RotRight(lval, expr)
					}
					
					Token::Swap => {
						self.next();
						let rhs = self.parse_lval()?;
					    Statement::Swap(lval, rhs)
					}
					
					_ => Err("`:=`, `+=`, `-=`, or `<>`")?,
				}
			}
			
			// TODO: handle newline here for empty statement
			_ => Err("a valid statement")?,
		};
				
		// mandatory newline after statement
		self.expect(Token::Newline)
			.ok_or("newline after statement")?;
		
		// eat all extra newlines
		self.skip_newlines();
		
		Ok(stmt)
	}
}

impl Statement {
	pub fn eval(&self, t: &mut StackFrame, m: &Module) -> EvalResult<Value> {
		use self::Statement::*;
		
		match self {
			Skip => {}
			
			Let(id, _, init, block, dest) => {
				let init = init.eval(t)?;
				t.push(id.clone(), init);
				
				for stmt in block {
					stmt.eval(t, m)?;
				}
				
				let (final_id, final_val) = t.pop().unwrap();
				
				assert_eq!(*id, final_id);
				assert_eq!(final_val, dest.eval(t)?,
					"variable {:?} had unexpected value", id);
			}
			
			Xor(lval, expr) => {
				let expr = expr.eval(t)?;
				match (lval.get_mut_ref(t)?, expr) {
					(Value::Int(ref mut l), Value::Int(r)) =>
						*l ^= r,
					_ => panic!("tried to do something illegal")
				}
			}
			
			Add(lval, expr) => {
				let expr = expr.eval(t)?;
				match (lval.get_mut_ref(t)?, expr) {
					(Value::Int(ref mut l), Value::Int(r)) =>
						*l = l.wrapping_add(r),
					_ => panic!("tried to do something illegal")
				}
			}
			
			Sub(lval, expr) => {
				let expr = expr.eval(t)?;
				match (lval.get_mut_ref(t)?, expr) {
					(Value::Int(ref mut l), Value::Int(r)) =>
						*l = l.wrapping_sub(r),
					_ => panic!("tried to do something illegal")
				}
			}
			
			RotLeft(lval, expr) => {
				let expr = expr.eval(t)?;
				match (lval.get_mut_ref(t)?, expr) {
					(Value::Int(ref mut l), Value::Int(r)) =>
						*l = l.rotate_left(r as u32),
					_ => panic!("tried to do something illegal")
				}
			}
			
			RotRight(lval, expr) => {
				let expr = expr.eval(t)?;
				match (lval.get_mut_ref(t)?, expr) {
					(Value::Int(ref mut l), Value::Int(r)) =>
						*l = l.rotate_right(r as u32),
					_ => panic!("tried to do something illegal")
				}
			}
			
			// sighhhhhhhhhhhhhhhhh
			Swap(left, right) => {
				let StackFrame { args, vars } = t;
				
				let left_idx = vars.iter()
					.rposition(|(name, _)| *name == left.id);
				let right_idx = vars.iter()
					.rposition(|(name, _)| *name == right.id);
				
				match (left_idx, right_idx) {
					(Some(left_idx), Some(right_idx)) => {
						let left  = &mut vars[left_idx];
						let right = &mut vars[right_idx];
						let left_name  = left.0.clone();
						let right_name = right.0.clone();
						assert_eq!(
							left.1.get_type(),
							right.1.get_type(),
							"tried to swap variables with different types"
						);
						vars.swap(left_idx, right_idx);
					}
					(Some(left_idx), None) => {
						let left = &mut vars[left_idx].1;
						let right = &mut args[&right.id];
						assert_eq!(
							left.get_type(),
							right.get_type(),
							"tried to swap variables with different types"
						);
						std::mem::swap(left, right);
					}
					(None, Some(right_idx)) => {
						let left = &mut args[&left.id];
						let right = &mut vars[right_idx].1;
						assert_eq!(
							left.get_type(),
							right.get_type(),
							"tried to swap variables with different types"
						);
						std::mem::swap(left, right);
					}
					(None, None) => {
						let left = &mut args[&left.id];
						let right = &mut args[&right.id];
						assert_eq!(
							left.get_type(),
							right.get_type(),
							"tried to swap variables with different types"
						);
						std::mem::swap(left, right);
					}
				}
				/*
				// ensure types are the same
				assert_eq!(
					t.vars[left_idx].1.get_type(),
					t.vars[right_idx].1.get_type(),
					"tried to swap variables with different types"
				);
				
				// get names of values being swapped for later
				let left_name = t.vars[left_idx].clone();
				let right_name = t.vars[right_idx].clone();
				
				t.vars.swap(left_idx, right_idx);
				
				// rectify names
				t.vars[left_idx] = left_name;
				t.vars[right_idx] = right_name;
				*/
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
				for item in &m.items {
					match item {
						Item::Proc(pr)
						if pr.name == *callee_name => {
							pr.call(vals, m);
							break;
						}
						
						Item::InternProc(name, pr, _)
						if name == callee_name => {
							pr(&mut vals)?;
							break;
						}
						
						_ => {}
					}
				}
			}
			Undo(callee_name, args) => {
				let mut vals = Vec::new();
				for arg in args {
					vals.push(arg.eval(t)?);
				}
				for item in &m.items {
					match item {
						Item::Proc(pr)
						if pr.name == *callee_name => {
							pr.uncall(vals, m);
							break;
						}
						
						Item::InternProc(name, _, pr)
						if name == callee_name => {
							pr(&mut vals)?;
							break;
						}
						
						_ => {}
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
					_ => panic!("tried to do something illegal")
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
