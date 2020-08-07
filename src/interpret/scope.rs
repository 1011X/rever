use super::*;

struct Scope {
	params: HashMap<String, Value>,
	vars:   Vec<(String, Value)>,
	items:  HashMap<String, Value>,
}

impl Scope {
	pub fn new(module, params) -> Self {
		Self {
			params,
			vars: Vec::new(),
			items: module,
		}
	}
	
	pub fn push(&mut self, name: String, val: Value) {
		self.vars.push((name, val));
	}
	
	pub fn pop(&mut self, name: String, val: Value) {
		assert_eq!(self.vars.pop(), (name, val));
	}
	
	pub fn get_lval(&self, given_name: &str) -> Option<&Value> {
		self.vars.iter()
			.rfind(|(var_name, _)| var_name == given_name)
			.or_else(|| self.params.get(given_name))
	}
	
	pub fn get_mut_lval(&mut self, given_name: &str) -> Option<&mut Value> {
		todo!()
	}
}
