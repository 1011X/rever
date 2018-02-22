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
					-0xFF...-1 => {
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
					-0xFF...-1 => {
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
	pub fn get_reg(&mut self, _: &mut Vec<rel::Op>) -> rel::Reg {
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

pub fn optimize(mut v: Vec<rel::Op>) -> Vec<rel::Op> {
	use rel::Op;
	let mut prev_len = v.len() + 1;
	
	// stop when optimizations no longer do anything
	while prev_len > v.len() {
		let mut vc = v.clone();
		prev_len = v.len();
		for (i, window) in v.windows(2).enumerate() {
			// we're at the end of code
			if window.len() < 2 { continue; }
			
			// have to break on every optimization because using drain() more
			// than once in the same vector means the index used for the range
			// may not be within the vector anymore
			match (&window[0], &window[1]) {
				// xor r, v;  xor r, v;
				(&Op::Xor(lra, lrb), &Op::Xor(rra, rrb))
				| (&Op::Add(lra, lrb), &Op::Sub(rra, rrb))
				| (&Op::Sub(lra, lrb), &Op::Add(rra, rrb))
				if lra == rra && lrb == rrb
				=> {
					vc.drain(i..i + 2);
					break;
				}
				
				// addi r, a;  subi r, b;
				(&Op::AddImm(lr, a), &Op::SubImm(rr, b))
				| (&Op::SubImm(lr, b), &Op::AddImm(rr, a))
				if lr == rr
				=> {
					use std::cmp::Ordering;
					vc.splice(i..i + 2, match a.cmp(&b) {
						Ordering::Equal
							=> None,
						Ordering::Greater
							=> Some(Op::AddImm(lr, a - b)),
						Ordering::Less
							=> Some(Op::SubImm(lr, b - a)),
					});
					break;
				}
				// enable only when actually encountered in generated code
				/*
				(&Op::XorImm(lr, a), &Op::XorImm(rr, b))
				if lr == rr
				=> {
					vc.splice(i..i + 2, if a == b {
						None
					} else {
						Some(Op::XorImm(lr, a ^ b))
					});
					break;
				}
				
				(&Op::Swap(lra, lrb), &Op::Swap(rra, rrb))
				if lra == rra && lrb == rrb || lra == rrb && lrb == rra
				=> {
					vc.drain(i..i + 2);
					break;
				}
				*/
				_ => {}
			}
		}
		v = vc;
	}
	v
}

/*
trait Compile {
	type Isa;
	type Reg;
	
	fn compile(&self, &mut State, &mut Vec<Self::Isa>) -> Option<Self::Reg>;
	
	fn uncompile(&self, reg: Self::Reg, state: &mut State, code: &mut Vec<Self::Isa>) {
		
	}
}
*/
