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
		if tokens.next() != Some(Token::Proc) {
			return Err("`proc`");
		}
		
		// get procedure name
		let proc_name = match tokens.next() {
			Some(Token::Ident(n)) => n,
			_ => return Err("procedure name")
		};
		
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
					
					None => return Err("`,` or `)`"),
					
					// parse as parameter
					_ => {
						// check mutability
						let mut mutable = false;
						
						if tokens.peek() == Some(&Token::Var) {
							mutable = true;
							tokens.next();
						}
						
						// get parameter name
						let param_name = match tokens.next() {
							Some(Token::Ident(n)) => n,
							_ => return Err("a parameter name")
						};
						
						// ':'
						if tokens.next() != Some(Token::Colon) {
							return Err("`:` after parameter name");
						}
						
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
							_ => return Err("`,` or `)`")
						}
					}
				}
			}
		}
		
		// check for newline
		if tokens.next() != Some(Token::Newline) {
			return Err("newline after parameter list");
		}
		
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

impl Procedure {
	// TODO perhaps the arguments should be stored in a HashMap, the local vars
	// in a vector, and then turn the vector into a hashmap and compare keys at
	// the end to verify everything is there.
	fn call_base(&self, dir: Dir, args: Vec<Value>, m: &Module) -> Vec<Value> {
		// verify number of arguments and their types
		assert_eq!(args.len(), self.params.len());
		for (arg, param) in args.iter().zip(&self.params) {
			assert_eq!(arg.get_type(), param.typ);
		}
		
		// store args in scope stack
		let mut vars: Vec<(String, Value)> = self.params.iter()
			.map(|param| param.name.clone())
			.zip(args.into_iter())
			.collect();
		
		// execute actual code
		if dir == Dir::Fore {
			for stmt in &self.code {
				stmt.eval(&mut vars, m);
			}
		} else {
			for stmt in &self.code {
				stmt.clone().invert().eval(&mut vars, m);
			}
		}
		
		// verify number of arguments and their types again
		assert_eq!(vars.len(), self.params.len());
		for (var, param) in vars.iter().zip(&self.params) {
			assert_eq!(var.1.get_type(), param.typ);
		}
			
		// store arg values back in parameters
		vars.into_iter()
			.map(|(_, val)| val)
			.collect()
	}
	
	pub fn call(&self, args: Vec<Value>, m: &Module) -> Vec<Value> {
		self.call_base(Dir::Fore, args, m)
	}
	
	pub fn uncall(&self, args: Vec<Value>, m: &Module) -> Vec<Value> {
		self.call_base(Dir::Back, args, m)
	}
	/*
	// add the procedure to the scope
	pub fn eval(&self, t: &mut Scope) -> EvalResult {
		unimplemented!()
	}
	*/
}
