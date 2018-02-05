//use std::ops::Deref;
use super::*;
use super::super::interpret::{Value, SymTab};

#[derive(Debug)]
pub struct LValue {
	pub name: String,
	pub indices: Vec<Expr>,
}

impl LValue {
	named!(pub parse<Self>, sp!(do_parse!(
		name: ident >>
		indices: many0!(delimited!(
			tag!("["),
			call!(Expr::parse),
			tag!("]")
		))
		>> (LValue {name, indices})
	)));
	
	// TODO deal with indices
	pub fn eval(&self, symtab: &SymTab) -> Result<Value, String> {
		Ok(symtab[&self.name].clone())
	}
	/*
	pub fn get_mut<'a>(&self, symtab: &'a mut SymTab) -> Option<&'a mut Value> {
		self._get_mut(self.indices[0], symtab)
	}
	
	fn _get_mut<'a>(&self, dim: Expr, symtab: &'a mut SymTab) -> Option<&'a mut Value> {
		globs.get_mut(&self.name)
		.map(|base| match *base {
			Value::Int(_)
			| Value::Stack(_) => base,
			
			Value::Array(ref mut vec) => {
				let idx = self.indices[dim.eval(symtab) as usize];
				vec.get_mut
			}
		})
	}
	*/
}
