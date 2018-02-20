use super::*;
use super::super::interpret::{Value, SymTab};
use super::super::compile::State;
use rel;

#[derive(Debug, PartialEq, Eq)]
pub enum Factor {
	Literal(Literal),
	LValue(LValue),
}

impl Factor {
	named!(pub parse<Self>, alt_complete!(
		map!(Literal::parse, Factor::Literal)
		| map!(LValue::parse, Factor::LValue)
	));
	
	pub fn compile(&self, state: &mut State, code: &mut Vec<rel::Op>) -> rel::Reg {
		use super::Literal;
		use rel::Op;
		match *self {
			Factor::LValue(ref lval) => lval.compile(state, code),
			// TODO: fix this
			Factor::Literal(ref lit) => match *lit {
				Literal::Int(n) => {
					let reg = state.get_reg(code);
					if n < 256 {
						code.push(Op::XorImm(reg, n as u8));
					} else {
						code.push(Op::XorImm(reg, (n >> 8) as u8));
						code.push(Op::LRotImm(reg, 8));
						code.push(Op::XorImm(reg, n as u8));
					}
					reg
				}
				_ => unimplemented!()
			},
		}
	}
	
	pub fn eval(&self, symtab: &SymTab) -> Result<Value, String> {
		match *self {
			Factor::Literal(ref lit) => Ok(lit.to_value()),
			Factor::LValue(ref lval) => lval.eval(symtab),
		}
	}
}
