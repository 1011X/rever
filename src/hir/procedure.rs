use super::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir { Fore, Back }

#[derive(Debug, Clone)]
pub struct Param {
	pub name: String,
	pub typ: Type,
}

#[derive(Debug, Clone)]
pub struct Procedure {
	/// List of parameters for the procedure.
	pub params: Vec<Param>,
	/// Sequence of statements that define the procedure.
	pub code: Vec<Statement>,
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

impl From<ast::Param> for Param {
	fn from(param: ast::Param) -> Self {
		Param {
			name: param.name,
			typ: param.typ.0.into(),
		}
	}
}

impl From<ast::Procedure> for Procedure {
	fn from(v: ast::Procedure) -> Self {
		Procedure {
			params: v.params.into_iter()
				.map(|p| p.0.into())
				.collect(),
			code: v.code.into_iter()
				.map(|s| s.0.into())
				.collect(),
		}
	}
}
