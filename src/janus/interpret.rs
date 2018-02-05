use std::collections::HashMap;
use std::cmp::{Ordering, PartialOrd};
//use super::Program;
use super::ast::{Item, Type, Program};

pub type SymTab = HashMap<String, Value>;
//pub type ProcTab = HashMap<String, Procedure>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
	Int(i16),
	Stack(Vec<i16>),
	Array(Vec<Value>),
}

impl PartialOrd for Value {
	fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
		match (self, other) {
			(&Value::Int(i0), &Value::Int(i1))
				=> Some(i0.cmp(&i1)),
			_ => None
		}
	}
}

pub fn run(prog: Program) -> Result<(), String> {
	let mut globs = HashMap::new();
	let mut procs = HashMap::new();
	
	for item in prog.items {
		// populate globs
		if let Item::Global(decl, init) = item {
			let val = match decl.typ {
				Type::Stack => Value::Stack(vec![]),
				Type::Int => {
					if let Some(init) = init {
						init.eval(&globs)?
					} else {
						Value::Int(0)
					}
				}
				
				Type::IntArray(dims) => {
					if let Some(init) = init {
						// TODO: handle init code
						unimplemented!();
					} else {
						// default value will be used, but all
						// dimensions must be known
						if dims.iter().all(|x| x.is_some()) {
							let dims = dims.into_iter()
								.collect::<Option<Vec<_>>>()
								.unwrap();
							
							for dim in dims.iter().rev() {
								//let v = vec![];
								
							}
							
							unimplemented!();
						} else {
							return Err(format!("All array lengths must be specified."));
						}
					}
				}
			};
			
			match (decl.typ, &val) {
				(Type::Int, &Value::Int(_)) => {}
				
				(Type::IntArray(ref expr), &Value::Array(ref vals))
				//if expr.eval(&globs).to_int().unwrap() >= vals.len()
					=> unimplemented!(),
				
				_ => return Err("Type and assigned value don't match.".to_string())
			}
			
			globs.insert(decl.name.clone(), val);
		}
		
		// store procedure
		else if let Item::Proc(pr) = item {
			if procs.contains_key(&pr.name) {
				return Err(format!("Procedure {} is already defined.", pr.name));
			}
			
			procs.insert(pr.name.clone(), pr);
		}
	}
	
	//procs["main"].execute();
	Ok(())
}
/*
fn reduce(mut p: Procedure) -> Procedure {
	for stmt in &mut p.body {
		
	}
}
*/
