use std::collections::HashMap;
use rel;

#[derive(Debug)]
pub enum Loc {
	Reg(rel::Reg),
	Mem(usize),
}

#[derive(Debug, Default)]
pub struct State {
	regfile: rel::RegisterFile,
	hashmap: HashMap<String, Loc>
}

impl State {
	pub fn new() -> Self {
		State { hashmap: HashMap::new() }
	}
	
	fn get_available_reg(&mut self) {
		
	}
}
