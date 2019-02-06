use std::collections::BTreeSet;

use crate::ast::*;

#[derive(Debug)]
pub enum Deref {
	Direct,
	Indexed(Factor),
	Field(String),
}

#[derive(Debug)]
pub struct LValue {
	pub id: String,
	//pub ops: Vec<Deref>,
}

impl LValue {
    /*
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
	*/
	
	pub fn eval(&self, t: &VarTable) -> Value {
	    t[self.id].clone()
	}
	
	/*
	pub fn compile(&self, st: &mut SymbolTable) -> (rel::Reg, Vec<rel::Op>) {
		// TODO maybe move some of the stuff SymbolTable::get does over here?
		st.get(&self.id)
	}
	*/
}
