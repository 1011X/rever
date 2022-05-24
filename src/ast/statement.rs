use super::*;

#[derive(Debug, Clone)]
pub enum Stmt {
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
	
	Var(String, Type, Expr, Vec<Self>, Expr),
	If(Expr, Vec<Self>, Vec<Self>, Expr),
	From(Expr, Vec<Self>, Vec<Self>, Expr),
	//FromVar(String, Expr, Vec<Stmt>, Vec<Stmt>, Expr),
	//Match(String, Vec<_, Vec<Stmt>>),
	//For(String, Expr, Vec<Stmt>),
}

impl Stmt {
	pub fn invert(self) -> Self {
		match self {
			Stmt::Skip     => self,
			Stmt::Xor(..)  => self,
			Stmt::Swap(..) => self,
			
			Stmt::RotLeft(l, v) => Stmt::RotRight(l, v),
			Stmt::RotRight(l, v) => Stmt::RotLeft(l, v),
			
			Stmt::Add(l, v) => Stmt::Sub(l, v),
			Stmt::Sub(l, v) => Stmt::Add(l, v),
			
			Stmt::Do(p, args) => Stmt::Undo(p, args),
			Stmt::Undo(p, args) => Stmt::Do(p, args),
			
			// recursively invert blocks
			Stmt::Var(name, ty, init, scope, dest) =>
				Stmt::Var(
					name, ty,
					dest,
					scope.into_iter().map(|s| s.invert()).collect(),
					init
				),
			Stmt::If(test, main_block, else_block, assert) =>
				Stmt::If(
					assert,
					main_block.into_iter().map(|s| s.invert()).collect(),
					else_block.into_iter().map(|s| s.invert()).collect(),
					test
				),
			Stmt::From(assert, block, loop_block, test) =>
				Stmt::From(
					test,
					block.into_iter().map(|s| s.invert()).collect(),
					loop_block.into_iter().map(|s| s.invert()).collect(),
					assert
				),
		}
	}
}

impl Parser<'_> {
	pub fn parse_stmt(&mut self) -> ParseResult<Stmt> {
		let stmt = match *self.peek().ok_or("a statement")? {
			// skip
			// TODO use this keyword as a prefix to comment out statements?
			Token::Skip => {
				self.next();
				// TODO make this optional
				//self.parse_stmt()?;
				Stmt::Skip
			}
			
			/* do-call and undo-call syntax accept three forms:
			   + `do something`
			   + `do something: some, args` (1 arg min) TODO
			   + `do something(
			          multiline,
			          args
			      )` (0 arg min) TODO
			   also has special syntax like:
			   + do something: var new_var, drop used_var
			*/
			kw @ Token::Do | kw @ Token::Undo => {
				self.next();
				
				let name = match self.peek() {
					Some(Token::VarIdent) => self.slice().to_string(),
					_ => Err(match kw {
						Token::Do => "procedure name after `do`",
						Token::Undo => "procedure name after `undo`",
						_ => unreachable!()
					})?
				};
				self.next();
				
				// TODO check for parentheses. if so, go into multiline mode
				let mut args = Vec::new();
				
				if self.peek() == Some(&Token::Newline) {
					// do nothing on final newline
				}
				/*else if self.peek() == Some(&Token::Colon) {
					self.next();
					// TODO check for newline in case expression was forgotten
					let expr = self.parse_expr()?;
					args.push(expr);
					
					loop {
						match self.peek() {
							Some(Token::Newline) | None =>
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
				}*/
				else if self.peek() == Some(&Token::LBrace) {
					self.next();
					
					loop {
						match self.peek() {
							Some(Token::RBrace) => break,
							Some(_) => {
								args.push(self.parse_expr()?);
								
								match self.peek() {
									Some(Token::Comma | Token::Newline) => {
										self.next();
									}
									Some(Token::RBrace) => {}
									_ => Err("`,`, `}`, or newline")?,
								}
							}
							None => Err("`}` or expression")?,
						}
					}
					
					self.next();
				} else {
					return Err("`{`, or newline")?;
				}
				
				match kw {
					Token::Do   => Stmt::Do(name, args),
					Token::Undo => Stmt::Undo(name, args),
					_ => unreachable!()
				}
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
				
				Stmt::From(assert, main_block, back_block, test)
			}
			
			// var-drop
			Token::Var => {
				self.next();
				
				// get name
				let name = match self.peek() {
					Some(Token::VarIdent) => self.slice().to_string(),
					_ => Err("name in variable declaration")?,
				};
				self.next();
				
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
				let drop_name = match self.peek() {
					Some(Token::VarIdent) => self.slice().to_string(),
					_ => Err("name after `drop`")?,
				};
				self.next();
				
				if name != drop_name {
					Err("same variable name as before")?;
				}
				
				// get optional deinit value
				let drop = match self.expect(Token::Assign) {
					Some(_) => self.parse_expr()?,
					None => init.clone(),
				};
				
				Stmt::Var(name, typ, init, block, drop)
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
				
				// saw `else` instead of `fi`
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
						// check if immediately followed by an `if` token.
						// allows "embedding" of chained `if` statements.
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
				
				Stmt::If(cond, main_block, else_block, assert)
			}
			
			Token::VarIdent => {
				let lval = self.parse_lval()?;
				
				match self.peek().ok_or("modifying operator")? {
					Token::Assign => {
						self.next();
						let expr = self.parse_expr()?;
					    Stmt::Xor(lval, expr)
					}
					Token::AddAssign => {
						self.next();
						let expr = self.parse_expr()?;
					    Stmt::Add(lval, expr)
					}
					Token::SubAssign => {
						self.next();
						let expr = self.parse_expr()?;
					    Stmt::Sub(lval, expr)
					}
					
					Token::RolAssign => {
						self.next();
						let expr = self.parse_expr()?;
					    Stmt::RotLeft(lval, expr)
					}
					Token::RorAssign => {
						self.next();
						let expr = self.parse_expr()?;
					    Stmt::RotRight(lval, expr)
					}
					
					Token::Swap => {
						self.next();
						let rhs = self.parse_lval()?;
					    Stmt::Swap(lval, rhs)
					}
					
					_ => Err("`:=`, `+=`, `-=`, or `<>`")?,
				}
			}
			
			// TODO: handle newline here for empty statement
			token => {
				//eprintln!("Got {:?}: {}", token, self.slice());
				Err("a valid statement")?
			}
		};
				
		// mandatory newline after statement
		self.expect(Token::Newline)
			.ok_or("newline after statement")?;
		
		// eat all extra newlines
		self.skip_newlines();
		
		Ok(stmt)
	}
}

impl Stmt {
	pub fn eval(&self, t: &mut StackFrame, m: &Module) -> EvalResult<()> {
		match self {
			Stmt::Skip => {}
			
			Stmt::Var(id, _, init, block, dest) => {
				let init = init.eval(t)?;
				t.push(id.clone(), init);
				
				for stmt in block {
					if let Err(e) = stmt.eval(t, m) {
						eprintln!("{:?}", stmt);
						panic!("var {}: {:?}", id, e);
					}
				}
				
				let (final_id, final_val) = t.pop().unwrap();
				
				assert_eq!(*id, final_id);
				assert_eq!(final_val, dest.eval(t)?,
					"variable {:?} had unexpected value", id);
			}
			
			Stmt::Xor(lval, expr) => {
				let expr = expr.eval(t)?;
				match (t.get_mut(&lval)?, &expr) {
					(Value::U32(l), Value::U32(r)) =>
						*l ^= *r,
					_ => panic!("tried to do something illegal")
				}
			}
			
			Stmt::Add(lval, expr) => {
				let expr = expr.eval(t)?;
				match (t.get_mut(&lval)?, &expr) {
					(Value::U32(l), Value::U32(r)) =>
						*l = l.wrapping_add(*r),
					//(Value::Uint(l), Value::Uint(r)) =>
					//	*l = l.wrapping_add(*r),
					(Value::String(l), Value::String(r)) =>
						*l += r,
					(l, r) => panic!(
						"tried to increment a {:?} with a {:?}",
						l, r
					)
				}
			}
			
			Stmt::Sub(lval, expr) => {
				let expr = expr.eval(t)?;
				match (t.get_mut(&lval)?, &expr) {
					(Value::U32(l), Value::U32(r)) =>
						*l = l.wrapping_sub(*r),
					//(Value::Uint(l), Value::Uint(r)) =>
					//	*l = l.wrapping_sub(*r),
					(Value::String(l), Value::String(r)) => {
						assert!(
							l.ends_with(r),
							"string {:?} does not end with {:?}",
							l, r
						);
						*l = l.strip_suffix(r).unwrap().to_string();
					}
					(l, r) => panic!(
						"tried to decrement a {:?} with a {:?}",
						l.get_type(), r.get_type()
					)
				}
			}
			
			Stmt::RotLeft(lval, expr) => {
				let expr = expr.eval(t)?;
				match (t.get_mut(&lval)?, &expr) {
					(Value::U32(l), Value::U32(r)) =>
						*l = l.rotate_left(*r as u32),
					//(Value::Uint(l), Value::Uint(r)) =>
					//	*l = l.rotate_left(*r as u32),
					_ => panic!("tried to do something illegal")
				}
			}
			
			Stmt::RotRight(lval, expr) => {
				let expr = expr.eval(t)?;
				match (t.get_mut(&lval)?, &expr) {
					(Value::U32(l), Value::U32(r)) =>
						*l = l.rotate_right(*r as u32),
					//(Value::Uint(l), Value::Uint(r)) =>
					//	*l = l.rotate_right(*r as u32),
					_ => panic!("tried to do something illegal")
				}
			}
			
			// sighhhhhhhhhhhhhhhhh
			Stmt::Swap(left, right) => {
				t.swap(&left.id, &right.id)?
				/*
				// ensure types are the same
				assert_eq!(
					t.values[left_idx].1.get_type(),
					t.values[right_idx].1.get_type(),
					"tried to swap variables with different types"
				);
				
				todo!("swapping is not currently supported");
				
				// get names of values being swapped for later
				let left_name = t.vars[left_idx].clone();
				let right_name = t.vars[right_idx].clone();
				
				t.vars.swap(left_idx, right_idx);
				
				// rectify names
				t.vars[left_idx] = left_name;
				t.vars[right_idx] = right_name;
				*/
			}
			
			/* Clearly we need more info here. Eventually we'll need to store
			the "path" of the current module with the procedure, but for now
			just having the items of the current module is good enough. So find
			a way to make that available. */
			kw @ Stmt::Do(callee_name, args)
			| kw @ Stmt::Undo(callee_name, args) => {
				let mut vals = Vec::new();
				for arg in args {
					vals.push(arg.eval(t)?);
				}
				
				// search items in current module for a matching procedure
				let mut p = None;
				for item in &m.items {
					match item {
						Item::Proc(pr) if pr.name == *callee_name => {
							p = Some(pr);
							break;
						}
						_ => {}
					}
				}
				
				// if procedure name found, call it. otherwise panic.
				let results = match (kw, p) {
					(Stmt::Do(..), Some(pr)) => pr.call(vals, m)?,
					(Stmt::Undo(..), Some(pr)) => pr.uncall(vals, m)?,
					_ => panic!("could not (un)call procedure {}: not found", callee_name),
				};
				
				// update variables to new values
				for (arg_expr, result) in args.iter().zip(results.into_iter()) {
					// check for arguments that were just an l-value, then
					// update those.
					let var_value = match arg_expr {
						Expr::LVal(lval) => t.get_mut(&lval)?,
						_ => continue,
					};
					
					// usually we'd do type checking here, but Procedure::call
					// already handles that.
					*var_value = result;
				}
			}
			
			Stmt::If(test, block, else_block, assert) => {
				match test.eval(t)? {
					Value::Bool(true) => {
						for stmt in block {
							if let Err(e) = stmt.eval(t, m) {
								eprintln!("{:?}", stmt);
								panic!("{:?}", e);
							}
						}
						assert_eq!(assert.eval(t)?, Value::Bool(true));
					}
					Value::Bool(false) => {
						for stmt in else_block {
							if let Err(e) = stmt.eval(t, m) {
								eprintln!("{:?}", stmt);
								panic!("{:?}", e);
							}
						}
						assert_eq!(assert.eval(t)?, Value::Bool(false));
					}
					_ => panic!("tried to do something illegal")
				}
			}
			
			Stmt::From(assert, do_block, loop_block, test) => {
				assert_eq!(assert.eval(t)?, Value::Bool(true));
				loop {
					for stmt in do_block {
						stmt.eval(t, m)?;
					}
					
					match test.eval(t)? {
						Value::Bool(true) => break,
						Value::Bool(false) =>
							for stmt in loop_block {
								if let Err(e) = stmt.eval(t, m) {
									eprintln!("{:?}", stmt);
									panic!("{:?}", e);
								}
							}
						_ => panic!("tried to do something illegal")
					}
					
					//eprintln!("{:?}", t);
					assert_eq!(assert.eval(t)?, Value::Bool(false));
				}
			}
		}
		
		Ok(())
	}
}
