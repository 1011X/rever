use std::collections::HashMap;

use super::*;

pub type Stack = Vec<StackFrame>;

/// Stores values of parameters and local variables during a function or
/// procedure call.
#[derive(Debug, Clone)]
pub struct StackFrame {
	pub args:  HashMap<String, Value>,
	pub vars:  Vec<(String, Value)>,
	//items:  HashMap<String, Value>,
}

impl StackFrame {
	pub fn new(args: HashMap<String, Value>) -> Self {
		Self {
			args,
			vars: Vec::new(),
			//items: module,
		}
	}
	
	pub fn push(&mut self, name: String, val: Value) {
		self.vars.push((name, val));
	}
	
	pub fn pop(&mut self) -> Option<(String, Value)> {
		self.vars.pop()
	}
	
	pub fn remove(&mut self, given_name: &str) -> EvalResult<Value> {
		let idx = self.vars.iter()
			.enumerate()
			.rfind(|(_, (var_name, _))| *var_name == given_name)
			.map(|(i,_)| i)
			.ok_or(EvalError::UnknownIdent(given_name.to_string()))?;
		
		Ok(self.vars.remove(idx).1)
	}
	/*
	pub fn swap(&mut self, left: &LValue, right: &LValue) {
		todo!();
		
		/*
		let left_idx = self.vars.iter()
			.rposition(|(name, _)| *name == left.id)
			.ok_or(EvalError::UnknownIdent(left.id.clone()))?;
		let right_idx = self.vars.iter()
			.rposition(|(name, _)| *name == right.id)
			.ok_or(EvalError::UnknownIdent(right.id.clone()))?;
		
		// ensure types are the same
		assert_eq!(
			self.vars[left_idx].1.get_type(),
			self.vars[right_idx].1.get_type(),
			"tried to swap variables with different types"
		);
		
		// get names of values being swapped for later
		let left_name = self.vars[left_idx].clone();
		let right_name = self.vars[right_idx].clone();
		
		self.vars.swap(left_idx, right_idx);
		
		// rectify names
		self.vars[left_idx] = left_name;
		self.vars[right_idx] = right_name;
		*/
	}
	*/
	pub fn get(&self, given_name: &str) -> EvalResult<&Value> {
		self.vars.iter()
			.rfind(|(var_name, _)| var_name == given_name)
			.map(|(_, value)| value)
			.or_else(|| self.args.get(given_name))
			.ok_or(EvalError::UnknownIdent(given_name.to_string()))
	}
	
	pub fn get_mut(&mut self, given_name: &str) -> EvalResult<&mut Value> {
		self.vars.iter_mut()
			.rfind(|(var_name, _)| var_name == given_name)
			.map(|(_, value)| value)
			// not .or_else() bcuz the closure would have to borrow more than
			// one mutable reference.
			.or(self.args.get_mut(given_name))
			.ok_or(EvalError::UnknownIdent(given_name.to_string()))
	}
}
