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
	
	Var(String, Option<Type>, Expr, Vec<Statement>, Expr),
	If(Expr, Vec<Statement>, Vec<Statement>, Expr),
	From(Expr, Vec<Statement>, Vec<Statement>, Expr),
	//Match(String, Vec<_, Vec<Statement>>),
	//FromVar(String, Expr, Vec<Statement>, Vec<Statement>, Expr),
	//For(String, Expr, Vec<Statement>),
}

impl Parse for Statement {
	fn parse(tokens: &mut Tokens) -> ParseResult<Self> {
		let res = match tokens.peek() {
			// skip
			// TODO use this keyword as a prefix to comment out statements?
			Some(Token::Skip) => {
				tokens.next();
				Ok(Statement::Skip)
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
			Some(Token::Do) => {
				tokens.next();
				
				let name = match tokens.next() {
					Some(Token::Ident(name)) => name,
					_ => return Err("procedure name after `do`")
				};
				
				// TODO check for parentheses. if so, go into multiline mode
				let mut args = Vec::new();
				
				if tokens.peek() == Some(&Token::Newline) {
					// do nothing
				} else if tokens.peek() == Some(&Token::Colon) {
					tokens.next();
					// TODO check for newline, in case expression is missing
					args.push(Expr::parse(tokens)?);
					loop {
						match tokens.peek() {
							None | Some(Token::Newline) => break,
							Some(Token::Comma) => {
								tokens.next();
								// TODO check for "substatements" first.
								// E.g. `var file` or `drop buf` in args.
								args.push(Expr::parse(tokens)?);
							}
							_ => return Err("`,` or newline"),
						}
					}
				} else if tokens.peek() == Some(&Token::LParen) {
					tokens.next();
					unimplemented!();
				} else {
					return Err("`:`, or newline");
				}
				
				Ok(Statement::Do(name, args))
			}
			
			// undo
			// accepts same forms as `do`.
			// And no, you can't merge the `do` and `undo` branches by using
			// `tok @ pattern` and matching at the end because it gives a
			// "cannot borrow `*tokens` as mutable more than once at a time"
			// error (as of rustc v1.42.0).
			Some(Token::Undo) => {
				tokens.next();
				
				let name = match tokens.next() {
					Some(Token::Ident(name)) => name,
					_ => return Err("procedure name after `undo`")
				};
				
				// TODO check for parentheses. if so, go into multiline mode
				let mut args = Vec::new();
				
				if tokens.peek() == Some(&Token::Newline) {
					// do nothing
				} else if tokens.peek() == Some(&Token::Colon) {
					tokens.next();
					// TODO check for newline, in case expression is missing
					args.push(Expr::parse(tokens)?);
					loop {
						match tokens.peek() {
							None | Some(Token::Newline) => break,
							Some(Token::Comma) => {
								tokens.next();
								args.push(Expr::parse(tokens)?);
							}
							_ => return Err("`,` or newline"),
						}
					}
				} else if tokens.peek() == Some(&Token::LParen) {
					tokens.next();
					unimplemented!();
				} else {
					return Err("`:`, or newline");
				}
				
				Ok(Statement::Undo(name, args))
			}
			
			// from-until
			Some(Token::From) => {
				tokens.next();
				
				// parse loop assertion
				let assert = Expr::parse(tokens)?;
				
				// ensure there's a newline afterwards
				tokens.expect(&Token::Newline)
					.ok_or("newline after from expression")?;
				
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
				tokens.expect(&Token::Newline)
					.ok_or("newline after until expression")?;
				
				// parse reverse loop block
				let mut back_block = Vec::new();
				loop {
					match tokens.peek() {
						Some(Token::Loop) => {
							tokens.next();
							break;
						}
						Some(_) => back_block.push(Statement::parse(tokens)?),
						None => return Err("a statement or `loop`"),
					}
				}
				
				if main_block.is_empty() && back_block.is_empty() {
					return Err("a non-empty do-block or back-block in from-loop");
				}
				
				Ok(Statement::From(assert, main_block, back_block, test))
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
				tokens.expect(&Token::Assign)
					.ok_or("`:=` after variable name")?;
				
				// get initialization expression
				let init = Expr::parse(tokens)?;
				
				// get newline
				tokens.expect(&Token::Newline)
					.ok_or("newline after variable declaration")?;
				
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
				tokens.expect(&Token::Ident(name.to_string()))
					.ok_or("same variable name as before")?;
				
				// check for assignment op
				let drop =
					if tokens.peek() == Some(&Token::Assign) {
						tokens.next();
						Expr::parse(tokens)?
					} else {
						init.clone()
					};
				
				Ok(Statement::Var(name, typ, init, block, drop))
			}
			
			// if-else
			Some(Token::If) => {
				tokens.next();
				
				// parse if condition
				let cond = Expr::parse(tokens)?;
				
				// expect newline
				tokens.expect(&Token::Newline)
					.ok_or("newline after `if` predicate")?;
				
				// parse the main block
				let mut main_block = Vec::new();
				loop {
					match tokens.peek() {
						// if `else` or `fi` is found, end block.
						Some(Token::Else) |
						Some(Token::Fi) => break,
						
						Some(_) => {
							let stmt = Statement::parse(tokens)?;
							main_block.push(stmt);
						}
						None => return Err("a statement, `else`, or `fi`")
					}
				}
				
				// parse else section
				let mut else_block = Vec::new();
				
				// saw `else`
				if tokens.peek() == Some(&Token::Else) {
					tokens.next();
					
					// check if newline to parse a block
					if tokens.peek() == Some(&Token::Newline) {
						tokens.next();
						// parse else block. MUST have at least 1 statement.
						loop {
							match tokens.peek() {
								// TODO is minimum statement requirement a good idea?
								Some(Token::Fi) if else_block.is_empty() =>
									return Err("else-block to have at least 1 statement"),
								Some(Token::Fi) =>
									break,
								Some(_) =>
									else_block.push(Statement::parse(tokens)?),
								None =>
									return Err("a statement or `fi`"),
							}
						}
					} else if tokens.peek() == Some(&Token::If) {
						// check if it's a single `if` statement. this allows
						// "embedding" of chained `if` statements.
						let stmt = Statement::parse(tokens)?;
						else_block.push(stmt);
					} else {
						return Err("chaining `if` or a newline");
					}
				}
				
				// expect ending `fi`
				tokens.expect(&Token::Fi)
					.ok_or("`fi` to finish `if` statement")?;
				
				// parse the `fi` assertion if any, else use initial condition
				let assert = match tokens.peek() {
					None | Some(Token::Newline) => cond.clone(),
					_ => Expr::parse(tokens)?,
				};
				
				Ok(Statement::If(cond, main_block, else_block, assert))
			}
			
			Some(_) =>
				if let Ok(lval) = LValue::parse(tokens) {
					match tokens.peek() {
						Some(Token::Assign) => {
							tokens.next();
							
						    let expr = Expr::parse(tokens)?;
						    Ok(Statement::Xor(lval, expr))
						}
						Some(Token::AddAssign) => {
							tokens.next();
							
						    let expr = Expr::parse(tokens)?;
						    Ok(Statement::Add(lval, expr))
						}
						Some(Token::SubAssign) => {
							tokens.next();
							
						    let expr = Expr::parse(tokens)?;
						    Ok(Statement::Sub(lval, expr))
						}
						
						Some(Token::Rol) => {
							tokens.next();
							
						    let expr = Expr::parse(tokens)?;
						    Ok(Statement::RotLeft(lval, expr))
						}
						Some(Token::Ror) => {
							tokens.next();
							
						    let expr = Expr::parse(tokens)?;
						    Ok(Statement::RotRight(lval, expr))
						}
						
						Some(Token::Swap) => {
							tokens.next();
							
						    let rval = LValue::parse(tokens)?;
						    Ok(Statement::Swap(lval, rval))
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
		tokens.expect(&Token::Newline)
			.ok_or("newline after statement")?;
		
		res
	}
}
