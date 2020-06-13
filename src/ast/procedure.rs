use super::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir { Fore, Back }

#[derive(Debug, Clone)]
pub struct Param {
	pub name: String,
	pub mutable: bool,
	pub typ: Type,
}

#[derive(Debug, Clone)]
pub struct Procedure {
	/// Name of the procedure.
	pub name: String,
	/// List of parameters for the procedure.
	pub params: Vec<Param>,
	/// Sequence of statements that define the procedure.
	pub code: Vec<Statement>,
}

impl Parse for Procedure {
	fn parse(tokens: &mut Tokens) -> ParseResult<Self> {
		// keyword `proc`
		tokens.expect(&Token::Proc)
			.ok_or("`proc`")?;
		
		// get procedure name
		let proc_name = tokens.expect_ident()
			.ok_or("procedure name")?;
		
		// parse parameter list
		let mut params = Vec::new();
		
		// starting '('
		if tokens.peek() == Some(&Token::LParen) {
			tokens.next();
			
			loop {
				// TODO add case for newline for multiline param declaration?
				match tokens.peek() {
					// ending ')'
					Some(Token::RParen) => {
						tokens.next();
						break;
					}
					
					// parse as parameter
					Some(_) => {
						// check mutability
						let mutable = tokens.expect(&Token::Var).is_some();
						
						// get parameter name
						let param_name = tokens.expect_ident()
							.ok_or("parameter name in procedure declaration")?;
						
						// expect ':'
						tokens.expect(&Token::Colon)
							.ok_or("`:` after parameter name")?;
						
						// get type
						let typ = Type::parse(tokens)?;
						
						for Param { name, .. } in &params {
							if *name == param_name {
								eprintln!(
									"Some parameter names in `proc {}` overlap: {:?}",
									proc_name, name
								);
								return Err("parameter name to be unique");
							}
						}
						
						// push to list of parameters
						params.push(Param { mutable, name: param_name, typ });
						
						match tokens.next() {
							Some(Token::RParen) => break,
							Some(Token::Comma) => {}
							_ => return Err("`,` or `)` in parameter list")
						}
					}
					
					None => return Err("`,` or `)` in parameter list"),
				}
			}
		}
		
		// check for newline
		tokens.expect(&Token::Newline)
			.ok_or("newline after parameter list")?;
		
		// code block section
		let mut code = Vec::new();
		loop {
			match tokens.peek() {
				// ending 'end'
				Some(Token::End) => {
					tokens.next();
					break;
				}
				
				None => return Err("a statement or `end`"),
				
				// statement
				_ => code.push(Statement::parse(tokens)?),
			}
		}
		
		// the optional `proc` in `end proc`
		if tokens.peek() == Some(&Token::Proc) {
			tokens.next();
			
			// the optional name of procedure after `end proc`
			if tokens.peek() == Some(&Token::Ident(proc_name.clone())) {
				tokens.next();
			}
		}
		
		// the likely newline afterwards
		if tokens.peek() == Some(&Token::Newline) { tokens.next(); }
		
		Ok(Procedure { name: proc_name, params, code })
	}
}
