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

impl Parser {
	pub fn parse_proc(&mut self) -> ParseResult<Procedure> {
		self.expect(&Token::Proc).ok_or("`proc`")?;
		
		let proc_name = self.expect_ident()
			.ok_or("procedure name")?;
		
		let mut params = Vec::new();
		
		// parse parameter list
		// starting '('
		if self.expect(&Token::LParen).is_some() {
			loop {
				// TODO add case for newline for multiline param declaration?
				match self.peek() {
					// ending ')'
					Some(Token::RParen) => break,
					
					// parse as parameter
					Some(_) => {
						let mutable = self.expect(&Token::Var).is_some();
						
						let param_name = self.expect_ident()
							.ok_or("parameter name in procedure declaration")?;
						
						self.expect(&Token::Colon)
							.ok_or("`:` after parameter name")?;
						
						// get type
						let typ = self.parse_type()?;
						
						for Param { name, .. } in &params {
							if *name == param_name {
								eprintln!(
									"Some parameter names in `proc {}` overlap: {:?}",
									proc_name, name
								);
								return Err("parameter names to be unique");
							}
						}
						
						params.push(Param { mutable, name: param_name, typ });
						
						match self.peek() {
							Some(Token::Comma) => { self.next(); }
							Some(Token::RParen) => {}
							_ => return Err("`,` or `)` in parameter list")
						}
					}
					
					None => return Err("`,` or `)` in parameter list"),
				}
			}
			self.next();
		}
		
		self.expect(&Token::Newline)
			.ok_or("newline after parameter list")?;
		
		// code block section
		let mut code = Vec::new();
		loop {
			match self.peek() {
				Some(Token::End) => break,
				Some(_) => code.push(self.parse_stmt()?),
				None => return Err("a statement or `end`"),
			}
		};
		self.next();
		
		Ok(Procedure { name: proc_name, params, code })
	}
}

impl Procedure {
	// TODO perhaps the arguments should be stored in a HashMap, the local vars
	// in a vector, and then turn the vector into a hashmap and compare keys at
	// the end to verify everything is there.
	fn call_base(&self, dir: Dir, mut args: Vec<Value>, m: &Module) -> Vec<Value> {
		// verify number of arguments and their types
		debug_assert_eq!(args.len(), self.params.len());
		for (arg, param) in args.iter().zip(&self.params) {
			debug_assert_eq!(arg.get_type(), param.typ);
		}
		
		let mut vars = Vec::new();
		
		// store args in scope stack
		for param in &self.params {
			vars.push((param.name.clone(), args.pop().unwrap()));
		}
		
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
		
		// store arg values back in parameters
		for param in &self.params {
			args.push(vars.pop().unwrap().1);
		}
		
		drop(vars);
		
		// verify number of arguments and their types again
		debug_assert_eq!(args.len(), self.params.len());
		for (arg, param) in args.iter().zip(&self.params) {
			debug_assert_eq!(arg.get_type(), param.typ);
		}
		
		args
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
