use super::*;
use super::super::compile::*;
use rel;

#[derive(Debug)]
pub enum Deref {
	Direct,
	Indexed(Factor),
	Field(String),
}

#[derive(Debug)]
pub struct LValue {
	pub id: String,
	pub ops: Vec<Deref>,
}

impl LValue {
	named!(pub parse<Self>, ws!(do_parse!(
		id: ident >>
		ops: many0!(alt_complete!(
			value!(Deref::Direct, tag!("*"))
			| delimited!(
				tag!("["),
				map!(Factor::parse, Deref::Indexed),
				tag!("]")
			)
			| preceded!(tag!("."), map!(ident, Deref::Field))
		))
		>> (LValue { id, ops })
	)));
	
	pub fn compile<F>(&self, st: &mut SymbolTable, f: F) -> Vec<rel::Op>
	where F: Fn(rel::Reg) -> Vec<rel::Op> {
		// need to know: symbol's value
		match st[&self.id] {
			Location::Reg(r) => f(r),
			
			Location::Memory(offset) => {
				use rel::{Reg, Op};
				
				// TODO: generalize to any register, not just r0
				let mut code = vec![];
				let loc = st.get_mut(&self.id).unwrap();
				
				// copy sp to r0
				*loc = Location::Reg(Reg::R0);
				code.push(Op::CNot(Reg::R0, Reg::SP));
				
				// store offset in immediate instruction(s)
				// if it's small enough, we only have to use 1 instruction.
				// perhaps some of this isn't necessary? maybe leave it for the
				// optimizer later on
				match offset {
					0 => {}
					1...255 => code.extend(vec![
						Op::Immediate(Reg::R1, offset as u8),
						Op::CAdd(Reg::R0, Reg::R1)
					]),
					_ => code.extend(vec![
						Op::Immediate(Reg::R1, (offset >> 8) as u8),
						Op::RotLeftImm(Reg::R1, 8),
						Op::Immediate(Reg::R1, offset as u8),
						Op::CAdd(Reg::R0, Reg::R1)
					])
				}
				
				code.push(Op::Exchange(Reg::R2, Reg::R0));
				code.extend(f(Reg::R2));
				// Assuming value is still at the same register...
				code.push(Op::Exchange(Reg::R2, Reg::R0));
				
				// store offset in immediate instruction(s)
				match offset {
					0 => {}
					1...255 => code.extend(vec![
						Op::CSub(Reg::R0, Reg::R1),
						Op::Immediate(Reg::R1, offset as u8)
					]),
					_ => code.extend(vec![
						Op::CSub(Reg::R0, Reg::R1),
						Op::Immediate(Reg::R1, offset as u8),
						Op::RotRightImm(Reg::R1, 8),
						Op::Immediate(Reg::R1, (offset >> 8) as u8)
					])
				}
				
				code.push(Op::CNot(Reg::R0, Reg::SP));
				*loc = Location::Memory(offset);
				
				code
			}
		}
	}
}
