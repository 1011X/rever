use std::result;
use std::collections::HashMap;
//use super::Program;

pub type SymTab = HashMap<String, Value>;
pub type Result = result::Result<Value, String>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
	Int(i16),
	Stack(Vec<i16>),
	Array(Vec<Value>),
}

impl Value {
	
}
/*
pub fn run(prog: Program) -> Result<(), String> {
	let mut globs = HashMap::new();
	let mut main = None;
	
	for item in &self.items {
		// populate globs
		if let Item::Global(decl, init) = *item {
			let val = match decl.typ {
				Type::Stack => Value::Stack(vec![]),
				Type::Int => {
					init.map(|x| x.eval(&globs).to_value())
					.unwrap_or(Value::Int(0))
				}
				Type::IntArray(ref dims) => {
					if let Some(init) = init {
						
					} else {
						if dims.iter().all(|x| x.is_some()) {
							for dim in dims.iter().rev() {
								let v = vec![];
								
							}
						} else {
							return Err("All array lengths must be specified.".to_string());
						}
					}
				}
			};
			
			match (decl.typ, val) {
				(Type::Int, Value::Int(_)) => {}
				
				(Type::IntArray(ref expr), Value::IntArray(ref vals))
				if expr.eval(&globs).to_int().unwrap() >= vals.len() => {}
				
				_ => return Err("Type and assigned value don't match.".to_string())
			}
			
			globs.insert(decl.name.clone(), val);
		}
		// find main function
		else if let Item::Proc(ref pr) = *item {
			if pr.name == "main" {
				if main.is_none() {
					main = Some(pr);
					break;
				} else {
					return Err("There is more than one main procedure.".to_string());
				}
			}
		}
	}
	
	if main.is_none() {
		return Err("No main function found.".to_string());
	}
	
	main.execute();
	Ok(())
}

fn reduce(mut p: Procedure) -> Procedure {
	for stmt in &mut p.body {
		
	}
}
*/
