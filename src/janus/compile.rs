use std::collections::HashMap;
use rel;

#[derive(Debug, Clone, Copy)]
pub enum Loc {
	Reg(rel::Reg),
	Mem(isize),
}

#[derive(Debug, Default)]
pub struct State {
	pub regfile: [bool; 7],
	pub hashmap: HashMap<String, Loc>
}

impl State {
	pub fn new() -> Self {
		State {
			regfile: [false; 7],
			hashmap: HashMap::new(),
		}
	}
	
	pub fn get(&mut self, name: &str, code: &mut Vec<rel::Op>) -> rel::Reg {
		use rel::{Reg, Op};
		let loc = *self.hashmap.get(name).expect("unknown variable");
		match loc {
			Loc::Reg(r) => r,
			Loc::Mem(offset) => {
				let read = self.get_reg(code);
				let res = self.get_reg(code);
				
				code.push(Op::Xor(read, Reg::SP));
				
				match offset {
					0 => {}
					1...0xFF => {
						code.push(Op::AddImm(read, offset as u8));
					}
					-0x100...-1 => {
						code.push(Op::SubImm(read, -offset as u8));
					}
					_ => unimplemented!()
				}
				
				code.push(Op::Exchange(res, read));
				
				match offset {
					0 => {}
					1...0xFF => {
						code.push(Op::SubImm(read, offset as u8));
					}
					-0x100...-1 => {
						code.push(Op::AddImm(read, -offset as u8));
					}
					_ => unimplemented!()
				}
				
				code.push(Op::Xor(read, Reg::SP));
				
				// don't need address (for now); it's in a register
				self.ret_reg(code, read);
				
				// reflect the fact that value is now in a register
				self.hashmap.remove(name);
				self.hashmap.insert(name.to_owned(), Loc::Reg(res));
				self.regfile[res as usize] = true;
				
				res
			}
		}
	}
	
	/// Gets an available register
	fn get_reg(&mut self, _: &mut Vec<rel::Op>) -> rel::Reg {
		let pos = self.regfile.iter()
			.position(|avail| !avail)
			.expect(&format!(
				"Register(s) need to be spilled: {:?}",
				self.regfile
			));
		self.regfile[pos] = true;
		rel::Reg::from(pos)
	}
	
	pub fn ret_reg(&mut self, _: &mut Vec<rel::Op>, reg: rel::Reg) {
		self.regfile[reg as usize] = false;
	}
}
