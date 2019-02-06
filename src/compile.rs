use std::collections::HashMap;

use super::Reverse;

/// Tracks location of a symbol or value.
#[derive(Debug, Clone, Copy)]
pub enum Location {
	/// In a CPU register
	Reg(rel::Reg),
	/// In memory at the specified offset in the stack. Variables have positive
	/// values, arguments don't.
	Memory(isize),
}

pub enum StateError {
	UnknownVariable,
}

#[derive(Debug, Default)]
pub struct SymbolTable {
	/// Stores the name of the variable/argument and its offset on the stack
	pub vars: HashMap<String, isize>,
	/// Tracks what registers are taken
	pub regfile: [bool; 7],
}

impl SymbolTable {
	pub fn new() -> Self {
		SymbolTable {
			regfile: [false; 7],
			vars: HashMap::new(),
		}
	}
	
	/// Loads the given symbol from the symbol table into a register, and
	/// returns the register it's in and the operations used to load it (which
	/// can be reversed to unload it if needed).
	//pub fn get(&mut self, name: &str) -> Result<(rel::Reg, Vec<rel::Op>), StateError> {
	pub fn get(&mut self, name: &str) -> (rel::Reg, Vec<rel::Op>) {
		use rel::{Reg, Op};
		//match *self.hashmap.get(name).ok_or(StateError::UnknownVariable)? {
		let mut code = Vec::new();
		// holds address of the symbol we want
		let read = self.get_reg(&mut code);
		
		// copy stack pointer
		code.push(Op::Xor(read, Reg::SP));
		
		// decide what instructions to generate based on how large the offset is
		match self.vars[name] {
			0 => {}
			off @ 1...0xFF => code.push(Op::AddImm(read, off as u8)),
			off @ -0xFF...-1 => code.push(Op::SubImm(read, -off as u8)),
			off => {
				let temp = self.get_reg(&mut code);
				code.extend_from_slice(&[
					Op::XorImm(temp, (off >> 8) as u8),
					Op::LRotImm(temp, 8),
					Op::XorImm(temp, off as u8),
					Op::Add(read, temp)
				]);
				self.ret_reg(&mut c, temp);
			}
		}
		
		let res = self.get_reg(&mut code);
		code.push(Op::Exchange(res, read));
		//code.extend_from_slice(&Reverse::reverse(offset_code));
		//code.push(Op::Xor(read, Reg::SP));
		
		// don't need address (for now); it's in a register
		self.ret_reg(&mut code, read);
		
		// reflect the fact that value is now in a register
		self.hashmap.remove(name);
		self.hashmap.insert(name.to_owned(), Location::Reg(res));
		self.regfile[res as usize] = true;
		
		(res, code)
	}
	
	/// Gets an available register
	pub fn get_reg(&mut self, _: &mut Vec<rel::Op>) -> rel::Reg {
		let pos = self.regfile.iter()
			.position(|avail| !avail)
			// TODO handle register spillover
			// SymbolTable already tracks the location in memory, so just find
			// a variable in a register and move it to the stack
			.unwrap();
		self.regfile[pos] = true;
		rel::Reg::from(pos)
	}
	
	pub fn ret_reg(&mut self, _: &mut Vec<rel::Op>, reg: rel::Reg) {
		// TODO move value back to memory
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
				if lra == rra && lrb == rrb => {
					vc.drain(i..i + 2);
					break;
				}
				
				// addi r, a;  subi r, b;
				(&Op::AddImm(lr, a), &Op::SubImm(rr, b))
				| (&Op::SubImm(lr, b), &Op::AddImm(rr, a))
				if lr == rr => {
					use std::cmp::Ordering;
					vc.splice(i..i + 2, match a.cmp(&b) {
						Ordering::Equal =>
							None,
						Ordering::Greater =>
							Some(Op::AddImm(lr, a - b)),
						Ordering::Less =>
							Some(Op::SubImm(lr, b - a)),
					});
					break;
				}
				// enable only when actually encountered in generated code
				/*
				(&Op::XorImm(lr, a), &Op::XorImm(rr, b))
				if lr == rr => {
					vc.splice(i..i + 2, if a == b {
						None
					} else {
						Some(Op::XorImm(lr, a ^ b))
					});
					break;
				}
				
				(&Op::Swap(la, lb), &Op::Swap(ra, rb))
				if la == ra && lb == rb || la == rb && lb == ra => {
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
