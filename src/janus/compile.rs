use std::collections::HashMap;
use rel;

#[derive(Debug, Clone, Copy)]
pub enum Loc {
	Reg(rel::Reg),
	Mem(isize),
}

#[derive(Debug, Default)]
pub struct State {
	regfile: [bool; 8],
	hashmap: HashMap<String, Loc>
}

impl State {
	pub fn new() -> Self {
		State {
			regfile: [false; 7],
			hashmap: HashMap::new(),
		}
	}
	
	pub fn insert(&mut self, name: &str, val: Loc) {
		self.hashmap.insert(String::from(name), val);
	}
	
	pub fn get(&mut self, name: &str, code: &mut Vec<rel::Op>) -> rel::Reg {
		use rel::{Reg, Op};
		let loc = *self.hashmap.get(name).expect("unknown variable");
		match loc {
			Loc::Reg(r) => r,
			Loc::Mem(offset) => {
				let read = self.get_reg(code);
				let res = self.get_reg(code);
				
				code.push(Op::CNot(read, Reg::SP));
				
				match offset {
					0 => {}
					1...0xFF => {
						let tmp = self.get_reg(code);
						code.push(Op::Immediate(tmp, offset as u8));
						code.push(Op::Add(read, tmp));
					}
					_ => unimplemented!()
				}
				
				code.push(Op::Exchange(res, read));
				self.hashmap.remove(name);
				self.insert(name, Loc::Reg(res));
				self.regfile[res as usize] = true;
				
				res
			}
		}
	}
	
	/// Gets an available register
	fn get_reg(&mut self, code: &mut Vec<rel::Op>) -> rel::Reg {
		let pos = self.regfile.iter()
			.position(|avail| *avail)
			.expect("Register(s) need to be spilled");
		rel::Reg::from(pos)
	}
}
