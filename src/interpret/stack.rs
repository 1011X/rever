use super::*;

pub type Stack = Vec<StackFrame>;

/// Stores values of parameters and local variables during a function or
/// procedure call.
#[derive(Debug, Clone)]
pub struct StackFrame {
	names: Vec<String>,
	values: Vec<Value>,
}

impl StackFrame {
	pub fn new(args: Vec<(String, Value)>) -> Self {
		let (names, values) = args.into_iter()
			.unzip(); // owo
		Self { names, values }
	}
	
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
	
	pub fn get(&self, given_name: &str) -> EvalResult<&Value> {
		let pos = self.names.iter()
			.rposition(|var_name| var_name == given_name)
			.ok_or(EvalError::UnknownIdent(given_name.to_string()))?;
		Ok(&self.values[pos])
	}
	
	pub fn get_mut(&mut self, given_name: &str) -> EvalResult<&mut Value> {
		let pos = self.names.iter()
			.rposition(|var_name| var_name == given_name)
			.ok_or(EvalError::UnknownIdent(given_name.to_string()))?;
		Ok(&mut self.values[pos])
	}
}
