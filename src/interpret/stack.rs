use super::*;

use crate::ast::{LValue, Deref, Module, Function, Procedure};

/// Contains the various items that can be used within the evoking item.
#[derive(Debug, Clone)]
pub struct Context {
	pub funcs: Vec<Function>,
	pub procs: Vec<Procedure>,
	pub mods: Vec<Module>,
}

/// Stores values of parameters and local variables during a function or
/// procedure call.
#[derive(Debug, Clone)]
pub struct StackFrame {
	names: Vec<String>,
	pub(crate) values: Vec<Value>,
	pub(crate) items: Context,
}

pub type Stack = Vec<StackFrame>;

impl Context {
	pub fn new() -> Self {
		Self {
			funcs: Vec::new(),
			procs: Vec::new(),
			mods: Vec::new(),
		}
	}
	
	pub fn from_module(module: &Module) -> Self {
		let mut ctx = Context::new();
		for item in &module.items {
			ctx.insert(item.clone());
		}
		ctx
	}
	
	pub fn insert(&mut self, item: Item) {
		match item {
			Item::Proc(p) => self.procs.push(p),
			Item::Fn(f) => self.funcs.push(f),
			_ => todo!()
		}
	}
}

impl StackFrame {
	pub fn new(items: Context, args: Vec<(String, Value)>) -> Self {
		let (names, values) = args.into_iter()
			.unzip(); // owo
		Self { names, values, items }
	}
	
	// used when calling internal procedures
	pub fn values(&mut self) -> &mut [Value] {
		&mut self.values
	}
	
	pub fn into_inner(self) -> Vec<Value> {
		self.values
	}
	
	pub fn push(&mut self, name: String, val: Value) {
		self.names.push(name);
		self.values.push(val);
	}
	
	pub fn pop(&mut self) -> Option<(String, Value)> {
		let name = self.names.pop();
		let value = self.values.pop();
		match (name, value) {
			(Some(name), Some(value)) => Some((name, value)),
			_ => None,
		}
	}
	
	pub fn remove(&mut self, given_name: &str) -> EvalResult<Value> {
		let idx = self.names.iter()
			.rposition(|var_name| *var_name == given_name)
			.ok_or(EvalError::UnknownIdent(given_name.to_string()))?;
		self.names.remove(idx);
		Ok(self.values.remove(idx))
	}
	
	pub fn swap(&mut self, left: &str, right: &str) -> EvalResult<()> {
		let left_idx = self.names.iter()
			.rposition(|name| *name == left)
			.ok_or(EvalError::UnknownIdent(left.to_string()))?;
		let right_idx = self.names.iter()
			.rposition(|name| *name == right)
			.ok_or(EvalError::UnknownIdent(right.to_string()))?;
		
		// ensure types are the same
		assert_eq!(
			self.values[left_idx].get_type(),
			self.values[right_idx].get_type(),
			"tried to swap variables with different types"
		);
		
		self.values.swap(left_idx, right_idx);
		
		Ok(())
	}
	
	// this function returns `Value` instead of `&Value` in order to allow us to
	// do ~~ h a c k s ~~. See: `string.len`.
	pub fn get(&self, deref_path: &LValue) -> EvalResult<Value> {
		let pos = self.names.iter()
			.rposition(|var_name| *var_name == deref_path.id)
			.ok_or(EvalError::UnknownIdent(deref_path.id.clone()))?;
		
		let mut value = self.values[pos].clone();
		
		for deref_op in &deref_path.ops {
			value = match value {
				Value::Stack(stack, _) => match deref_op {
					// this *should* be a temporary hack for now. remove once
					// stack values can store their length.
					Deref { name: Some(field), args: None } if field == "len" =>
						Value::U32(stack.len() as u32),
					
					Deref { name: None, args: Some(args) } =>
						match args[0].eval(self)? {
							Value::U32(i) =>
								stack.get(i as usize).unwrap().clone(),
							
							value => panic!("tried to do {:?}.({})", stack, value),
						}
					
					_ => todo!()
				}
				
				Value::String(string) => match deref_op {
					// this *should* be a temporary hack for now. remove once
					// string values can store their length.
					Deref {name: Some(field), args: None} if field == "len" =>
						Value::U32(string.len() as u32),
					
					Deref {name: None, args: Some(args)} =>
						match args[0].eval(self)? {
							Value::U32(i) => match string.chars().nth(i as usize) {
								Some(c) => c.into(),
								None => panic!("XX str: {:?}, i: {}, len: {}", string, i, string.len()),
							}
							
							value => todo!("{:?}.({:?})", string, value)
						}
					
					_ => todo!()
				}
				
				val => todo!("{} {:?}", val, deref_op)
			};
		}
		
		Ok(value)
	}
	
	pub fn get_mut(&mut self, deref_path: &LValue) -> EvalResult<&mut Value> {
		let pos = self.names.iter()
			.rposition(|var_name| *var_name == deref_path.id)
			.ok_or(EvalError::UnknownIdent(deref_path.id.clone()))?;
		
		// v important variables. `clone` is here because we can't borrow the
		// stack frame while it's already mutably borrowed. `value` cannot be
		// decoupled from the loop's body or we'd be returning a temporary
		// value.
		let clone = self.clone();
		let mut value = &mut self.values[pos];
		
		for deref in &deref_path.ops {
			match (value, deref) {
				(Value::Array(array),
				Deref { name: None, args: Some(args) }) =>
					match args[0].eval(&clone)? {
						Value::U32(idx) => {
							value = &mut array[idx as usize];
						}
						value => return Err(EvalError::TypeMismatch {
							expected: Type::U32,
							got: value.get_type(),
						}),
					}
				_ => todo!()
			}
		}
		
		Ok(value)
	}
}
