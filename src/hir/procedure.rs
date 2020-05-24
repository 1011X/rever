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

impl From<ast::Param> for Param {
	fn from(v: ast::Param) -> Self {
		Param {
			name: v.name,
			typ: v.typ.into(),
		}
	}
}

impl From<ast::Procedure> for Procedure {
	fn from(v: ast::Procedure) -> Self {
		Procedure {
			params: v.params.into_iter().map(Param::from).collect(),
			code: v.code.into_iter().map(Statement::from).collect(),
		}
	}
}
