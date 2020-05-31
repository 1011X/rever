use super::*;

struct Scope {
	vars:  Vec<(String, Value)>,
	procs: HashMap<String, hir::Procedure>,
	fns:   HashMap<String, hir::Function>,
}

impl Scope {
	pub fn new() -> Self {
		Self { vars: Vec::new() }
	}
	
	pub fn push(&mut self, name: String, val: Value) {
		self.vars.push((name, val));
	}
	
	pub fn pop(&mut self, name: String, val: Value) {
		assert_eq!(self.vars.pop(), (name, val));
	}
}
