use super::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir { Fore, Back }

#[derive(Debug, Clone)]
pub struct Procedure {
	/// Name of the function.
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
		let name = match tokens.next() {
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
					// try parsing as Param
					Some(_) => {
						params.push(Param::parse(tokens)?);
						
						match tokens.next() {
							Some(Token::RParen) => break,
							Some(Token::Comma) => {}
							_ => return Err("`,`")
						}
					}
					None => return Err("`,` or `)`")
				}
			}
		}
		
		// Verify that all parameter names are unique.
		/* Dev note: this is O((n^2 - n) / 2) but is actually better than the
		usual O(n log n + 2n) solution (copy, sort, then compare neighbors)
		because we expect a small number of parameters.
		Ideal is 9 parameters or less. */
		for (i, Param { name: first, .. }) in params.iter().enumerate() {
			for Param { name: second, .. } in &params[i + 1..] {
				if first == second {
					return Err("parameter name to be unique")
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
				// statement
				Some(_) => {
					let stmt = Statement::parse(tokens)?;
					code.push(stmt);
				}
				None => return Err("a statement or `end`")
			}
		}
		
		// the optional `proc` in `end proc`
		if tokens.peek() == Some(&Token::Proc) {
			tokens.next();
			
			// the optional name of procedure after `end proc`
			if tokens.peek() == Some(&Token::Ident(name.clone())) {
				tokens.next();
			}
		}
		
		// the likely newline afterwards
		if tokens.peek() == Some(&Token::Newline) { tokens.next(); }
		
		Ok(Procedure { name, params, code })
	}
}

impl Procedure {
	// TODO perhaps the arguments should be stored in a HashMap, the local vars
	// in a vector, and then turn the vector into a hashmap and compare keys at
	// the end to verify everything is there.
	fn call_base(&self, dir: Dir, args: Vec<Value>, m: &Module) -> Vec<Value> {
		// verify number of arguments and their types
		assert_eq!(
			args.iter().map(|arg| arg.get_type()).collect::<Vec<_>>(),
			self.params.iter().map(|param| &param.typ).cloned().collect::<Vec<_>>()
		);
		//for (arg, param) in args.iter
		
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
		assert_eq!(
			vars.iter().map(|(_, val)| val.get_type()).collect::<Vec<_>>(),
			self.params.iter().map(|param| &param.typ).cloned().collect::<Vec<_>>()
		);
			
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
