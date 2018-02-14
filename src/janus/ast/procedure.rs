use super::*;
use super::super::compile::{State, Loc};
use rel;

#[derive(Debug)]
pub struct Procedure {
	pub name: String,
	args: Vec<Decl>,
	body: Vec<Statement>
}

impl Procedure {
	named!(pub parse<Self>, sp!(do_parse!(
		tag!("procedure") >>
		name: ident >>
		tag!("(") >>
		args: separated_list!(tag!(","), Decl::parse) >>
		tag!(")") >>
		body: many1!(Statement::parse)
		
		>> (Procedure {name, args, body})
	)));
	
	pub fn compile(&self) -> Vec<rel::Op> {
		let mut state = State::new();
		let mut code = Vec::new();
		
		for (i, param) in self.args.iter().enumerate() {
			state.hashmap.insert(
				param.name.clone(),
				Loc::Mem(-((i + 1) as isize))
			);
		}
		
		for stmt in &self.body {
			stmt.compile(&mut state, &mut code);
		}
		/*
		for (i, arg) in self.args.iter().enumerate() {
			if let Loc::Reg(r) = state.hashmap.get(&arg.name).unwrap() {
				let tmp = state.get_reg(code);
				code.push(Op::CNot(
				code.push(Op::Immediate(tmp, -(i + 1)));
				code.push(Op::
			}
		}
		*/
		code
	}
	
	pub fn verify(&self) {
		unimplemented!();
	}
}
