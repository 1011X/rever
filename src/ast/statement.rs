use super::*;

#[derive(Debug, Clone)]
pub enum Statement {
	Skip,
	
	//Not(LValue),
	//Neg(LValue),
	
	RotLeft((LValue, Span), (Expr, Span)),
	RotRight((LValue, Span), (Expr, Span)),
	
	Xor((LValue, Span), (Expr, Span)),
	Add((LValue, Span), (Expr, Span)),
	Sub((LValue, Span), (Expr, Span)),
	
	Swap((LValue, Span), (LValue, Span)),
	//CSwap(Factor, LValue, LValue),
	
	Do(String, Vec<(Expr, Span)>),
	Undo(String, Vec<(Expr, Span)>),
	
	Let(
		String,
		Option<(Type, Span)>,
		(Expr, Span),
		Vec<(Statement, Span)>,
		Option<(Expr, Span)>
	),
	If(
		(Expr, Span),
		Vec<(Statement, Span)>,
		Vec<(Statement, Span)>,
		Option<(Expr, Span)>
	),
	From(
		(Expr, Span),
		Vec<(Statement, Span)>,
		Vec<(Statement, Span)>,
		(Expr, Span)
	),
	//FromLet(String, Expr, Vec<Statement>, Vec<Statement>, Expr),
	//Match(String, Vec<_, Vec<Statement>>),
	//For(String, Expr, Vec<Statement>),
}

impl Parser {
	pub fn parse_stmt(&mut self) -> ParseResult<Statement> {
		let stmt = match self.peek().ok_or("a statement")? {
			// skip
			// TODO use this keyword as a prefix to comment out statements?
			Token::Skip => {
				let (_, span) = self.next().unwrap();
				(Statement::Skip, span)
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
				let (_, start) = self.next().unwrap();
				
				let (name, end) = self.expect_ident_span()
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
							_ => return Err("`,` or newline"),
						}
					}
				} else if self.expect(&Token::LParen).is_some() {
					unimplemented!();
				} else {
					return Err("`:`, or newline");
				};
				
				let end = args.last().map(|(_, span)| *span).unwrap_or(end);
				
				(Statement::Do(name, args), start.merge(&end))
			}
			
			// undo
			// accepts same forms as `do`.
			// And no, you can't merge the `do` and `undo` branches by using
			// `tok @ pattern` and matching at the end because it gives a
			// "cannot borrow `*tokens` as mutable more than once at a time"
			// error (as of rustc v1.42.0).
			Token::Undo => {
				let (_, start) = self.next().unwrap();
				
				let (name, end) = self.expect_ident_span()
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
							_ => return Err("`,` or newline"),
						}
					}
				} else if self.expect(&Token::LParen).is_some() {
					unimplemented!();
				} else {
					return Err("`:`, or newline");
				};
				
				let end = args.last().map(|(_, span)| *span).unwrap_or(end);
				
				(Statement::Undo(name, args), start.merge(&end))
			}
			
			// from-until
			Token::From => {
				let (_, start) = self.next().unwrap();
				
				// parse loop assertion
				let assert = self.parse_expr()?;
				
				self.expect(&Token::Newline)
					.ok_or("newline after `from` assertion")?;
				
				// parse the main loop block
				let mut main_block = Vec::new();
				loop {
					match self.peek() {
						Some(Token::Until) => break,
						Some(_) => main_block.push(self.parse_stmt()?),
						None => return Err("a statement or `until`"),
					}
				}
				self.next();
				
				// parse the `until` test expression
				let test = self.parse_expr()?;
				
				self.expect(&Token::Newline)
					.ok_or("newline after `until` expression")?;
				
				// parse reverse loop block
				let mut back_block = Vec::new();
				loop {
					match self.peek() {
						Some(Token::Loop) => break,
						Some(_) => back_block.push(self.parse_stmt()?),
						None => return Err("a statement or `loop`"),
					}
				};
				let (_, end) = self.next().unwrap();
				
				// TODO: remove?
				if main_block.is_empty() && back_block.is_empty() {
					return Err("a non-empty do-block or back-block in from-loop");
				}
				
				(Statement::From(assert, main_block, back_block, test), start.merge(&end))
			}
			
			// let-drop
			Token::Let => {
				let (_, start) = self.next().unwrap();
				
				// get name
				let name = self.expect_ident()
					.ok_or("name in variable declaration")?;
				
				// get optional type
				let typ = match self.expect(&Token::Colon) {
					Some(_) => Some(self.parse_type()?),
					None => None,
				};
				
				// check for assignment op
				self.expect(&Token::Assign)
					.ok_or("`:=` in variable declaration")?;
				
				// get initialization expression
				let init = self.parse_expr()?;
				
				self.expect(&Token::Newline)
					.ok_or("newline after variable declaration")?;
				
				// get list of statements for which this variable is valid
				let mut block = Vec::new();
				loop {
					match self.peek() {
						Some(Token::Drop) => break,
						Some(_) => block.push(self.parse_stmt()?),
						None => return Err("a statement or `drop`"),
					}
				}
				self.next();
				
				// assert name
				let drop_name = self.expect(&Token::Ident(name.clone()))
					.ok_or("same variable name as before")?;
				
				// get optional deinit value
				let drop = match self.expect(&Token::Assign) {
					Some(_) => Some(self.parse_expr()?),
					None => None,
				};
				
				let end = drop.as_ref().map(|(_, span)| span).unwrap_or(&drop_name.1);
				let span = start.merge(&end);
				
				(Statement::Let(name, typ, init, block, drop), span)
			}
			
			// if-else
			Token::If => {
				let (_, start) = self.next().unwrap();
				
				// parse if condition
				let cond = self.parse_expr()?;
				
				self.expect(&Token::Newline)
					.ok_or("newline after `if` predicate")?;
				
				// parse the main block
				let mut main_block = Vec::new();
				loop {
					match self.peek() {
						Some(Token::Else)
						| Some(Token::Fi) => break,
						Some(_) => main_block.push(self.parse_stmt()?),
						None => return Err("a statement, `else`, or `fi`"),
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
								None => return Err("a statement or `fi`"),
							}
						}
					} else if self.peek() == Some(&Token::If) {
						// check if it's a single `if` statement. this allows
						// "embedding" of chained `if` statements.
						else_block.push(self.parse_stmt()?);
					} else {
						return Err("chaining `if` or a newline");
					}
				}
				
				// expect ending `fi`
				let fi = self.expect(&Token::Fi)
					.ok_or("`fi` to finish `if` statement")?;
				
				// parse `fi` assertion, if any
				let assert = match self.peek() {
					Some(Token::Newline)
					| None  => None,
					Some(_) => Some(self.parse_expr()?),
				};
				
				let end = assert.as_ref().map(|expr| expr.1).unwrap_or(fi.1);
				let span = start.merge(&end);
				
				(Statement::If(cond, main_block, else_block, assert), span)
			}
			
			Token::Ident(_) => {
				let lval = self.parse_lval()?;
				let start = lval.1;
				
				match self.peek().ok_or("modifying operator")? {
					Token::Assign => {
						self.next();
						let expr = self.parse_expr()?;
						let span = start.merge(&expr.1);
					    (Statement::Xor(lval, expr), span)
					}
					Token::AddAssign => {
						self.next();
						let expr = self.parse_expr()?;
						let span = start.merge(&expr.1);
					    (Statement::Add(lval, expr), span)
					}
					Token::SubAssign => {
						self.next();
						let expr = self.parse_expr()?;
						let span = start.merge(&expr.1);
					    (Statement::Sub(lval, expr), span)
					}
					
					Token::Rol => {
						self.next();
						let expr = self.parse_expr()?;
						let span = start.merge(&expr.1);
					    (Statement::RotLeft(lval, expr), span)
					}
					Token::Ror => {
						self.next();
						let expr = self.parse_expr()?;
						let span = start.merge(&expr.1);
					    (Statement::RotRight(lval, expr), span)
					}
					
					Token::Swap => {
						self.next();
						let rhs = self.parse_lval()?;
						let span = start.merge(&rhs.1);
					    (Statement::Swap(lval, rhs), span)
					}
					
					_ => return Err("`:=`, `+=`, `-=`, `:<`, `:>`, or `<>`"),
				}
			}
			
			// TODO: handle newline here for empty statement
			_ => return Err("a valid statement"),
		};
				
		// mandatory newline after statement
		self.expect(&Token::Newline)
			.ok_or("newline after statement")?;
		
		// eat all extra newlines
		while self.expect(&Token::Newline).is_some() {}
		
		Ok(stmt)
	}
}
