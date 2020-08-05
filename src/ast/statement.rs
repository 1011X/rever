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
				
				if self.expect(&Token::Newline).is_some() {
					// do nothing
				} else if self.expect(&Token::Colon).is_some() {
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
				} else if self.expect(&Token::LParen).is_some() {
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
				
				if self.expect(&Token::Newline).is_some() {
					// do nothing
				} else if self.expect(&Token::Colon).is_some() {
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
				} else if self.expect(&Token::LParen).is_some() {
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
				
				match self.peek() {
					Some(Token::Newline) => { self.next(); }
					Some(_) => Err("newline after `from` assertion")?,
					None => Err(ParseError::Eof)?,
				}
				
				// eat empty lines
				while self.expect(&Token::Newline).is_some() {}
				
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
				
				self.expect(&Token::Newline)
					.ok_or("newline after `until` expression")?;
				
				while self.expect(&Token::Newline).is_some() {}
				
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
				let typ = match self.expect(&Token::Colon) {
					Some(_) => self.parse_type()?,
					None => Type::Infer,
				};
				
				// check for assignment op
				self.expect(&Token::Assign)
					.ok_or("`:=` in variable declaration")?;
				
				// get initialization expression
				let init = self.parse_expr()?;
				
				self.expect(&Token::Newline)
					.ok_or("newline after variable declaration")?;
				
				// eat empty lines
				while self.expect(&Token::Newline).is_some() {}
				
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
				let drop_name = self.expect(&Token::Ident(name.clone()))
					.ok_or("same variable name as before")?;
				
				// get optional deinit value
				let drop = match self.expect(&Token::Assign) {
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
				
				match self.peek() {
					Some(Token::Newline) => { self.next(); }
					Some(_) => Err("newline after `if` predicate")?,
					None => Err(ParseError::Eof)?,
				}
				
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
				if self.expect(&Token::Else).is_some() {
					if self.expect(&Token::Newline).is_some() {
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
				let fi = self.expect(&Token::Fi)
					.ok_or("`fi` to finish `if` statement")?;
				
				// parse `fi` assertion, if any
				let assert = match self.peek() {
					Some(Token::Newline) => cond.clone(),
					Some(_) => self.parse_expr()?,
					None => Err("a newline or expression after `fi`")?,
				};
				
				Statement::If(cond, main_block, else_block, assert)
			}
			
			Token::Ident(_) => {
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
		self.expect(&Token::Newline)
			.ok_or("newline after statement")?;
		
		// eat all extra newlines
		while self.expect(&Token::Newline).is_some() {}
		
		Ok(stmt)
	}
}

impl Statement {
	pub fn eval(&self, t: &mut Scope, m: &Module) -> EvalResult {
		use self::Statement::*;
		
		match self {
			Skip => {}
			
			Let(id, _, init, block, dest) => {
				let init = init.eval(t)?;
				t.push((id.clone(), init));
				
				for stmt in block {
					stmt.eval(t, m)?;
				}
				
				let (final_id, final_val) = t.pop().unwrap();
				
				assert_eq!(*id, final_id);
				assert_eq!(final_val, dest.eval(t)?,
					"variable {:?} had unexpected value", id);
			}
			
			Xor(lval, expr) => match (lval.eval(t)?, expr.eval(t)?) {
				(Value::Int(l), Value::Int(r)) => {
					let pos = t.iter()
						.rposition(|(name, _)| *name == lval.id)
						.ok_or("variable name not found")?;
					t[pos].1 = Value::Int(l ^ r);
				}
				_ => return Err("tried to do something illegal")
			}
			
			Add(lval, expr) => {
				let expr = expr.eval(t)?;
				match (lval.get_mut_ref(t)?, expr) {
					(Value::Int(l), Value::Int(r)) => {
						/*let pos = t.iter()
							.rposition(|(name, _)| *name == lval.id)
							.expect("variable name not found");*/
						*l = l.wrapping_add(r);
						//t[pos].1 = Value::Int(l.wrapping_add(r));
					}
					_ => return Err("tried to do something illegal")
				}
			}
			Sub(lval, expr) => match (lval.eval(t)?, expr.eval(t)?) {
				(Value::Int(l), Value::Int(r)) => {
					let pos = t.iter()
						.rposition(|(name, _)| *name == lval.id)
						.expect("variable name not found");
					t[pos].1 = Value::Int(l.wrapping_sub(r));
				}
				_ => return Err("tried to do something illegal")
			}
			
			RotLeft(lval, expr) => match (lval.eval(t)?, expr.eval(t)?) {
				(Value::Int(l), Value::Int(r)) => {
					let pos = t.iter()
						.rposition(|(name, _)| *name == lval.id)
						.expect("variable name not found");
					t[pos].1 = Value::Int(l.rotate_left(r as u32));
				}
				_ => return Err("tried to do something illegal")
			}
			RotRight(lval, expr) => match (lval.eval(t)?, expr.eval(t)?) {
				(Value::Int(l), Value::Int(r)) => {
					let pos = t.iter()
						.rposition(|(name, _)| *name == lval.id)
						.expect("variable name not found");
					t[pos].1 = Value::Int(l.rotate_right(r as u32));
				}
				_ => return Err("tried to do something illegal")
			}
			
			Swap(left, right) => {
				let left_idx = t.iter()
					.rposition(|(name, _)| *name == left.id)
					.expect("variable name not found");
				let right_idx = t.iter()
					.rposition(|(name, _)| *name == right.id)
					.expect("variable name not found");
				
				// ensure types are the same
				assert_eq!(
					t[left_idx].1.get_type(),
					t[right_idx].1.get_type(),
					"tried to swap variables with different types"
				);
				
				// get names of values being swapped for later
				let left_name = t[left_idx].clone();
				let right_name = t[right_idx].clone();
				
				t.swap(left_idx, right_idx);
				
				// rectify names
				t[left_idx] = left_name;
				t[right_idx] = right_name;
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
							pr(vals.into_boxed_slice());
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
							pr(vals.into_boxed_slice());
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
