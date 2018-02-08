use super::*;
use super::super::interpret::{Value, SymTab};
use super::super::compile::{State, Loc};
use rel;

#[derive(Debug, PartialEq, Eq)]
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
	
	pub fn compile(&self, state: &mut State, code: &mut Vec<rel::Op>) -> rel::Reg {
		use rel::{Op, Reg};
		
		if self.indices.is_empty() {
			state.get(&self.name, code)
		} else {
			unimplemented!();
		}
	}
	
	pub fn uncompile(&self, state: &mut State, code: &mut Vec<rel::Op>) {
		
	}
	
	// TODO deal with indices
	pub fn eval(&self, symtab: &SymTab) -> Result<Value, String> {
		Ok(symtab[&self.name].clone())
	}
}

#[cfg(test)]
mod tests {
	use std::collections::HashMap;
	use super::super::super::interpret::Value;
	use super::*;
	
	#[test]
	fn parse() {
		assert_eq!(
			LValue::parse(b"a").unwrap().1,
			LValue {
				name: String::from("a"),
				indices: vec![],
			},
			"simple variable"
		);
		assert_eq!(
			LValue::parse(b"homu[0]").unwrap().1,
			LValue {
				name: String::from("homu"),
				indices: vec![Expr::Factor(Factor::Literal(Literal::Int(0)))],
			},
			"array variable with simple index"
		);
		assert_eq!(
			LValue::parse(b"mado[1 + 2]").unwrap().1,
			LValue {
				name: String::from("mado"),
				indices: vec![Expr::Add(
					Box::new(Expr::Factor(Factor::Literal(Literal::Int(1)))),
					Box::new(Expr::Factor(Factor::Literal(Literal::Int(2)))),
				)],
			},
			"array variable with expressive index"
		);
	}
	
	#[test]
	fn eval() {
		let mut symtab = HashMap::new();
		symtab.insert(String::from("a"), Value::Int(1));
		symtab.insert(String::from("yuno"), Value::Stack(vec![69]));
		symtab.insert(String::from("yuki"), Value::Array(vec![Value::Int(420)]));
		
		assert_eq!(
			LValue::parse(b"a").unwrap().1.eval(&symtab).unwrap(),
			Value::Int(1),
			"int variable"
		);
		assert_eq!(
			LValue::parse(b"yuno[0]").unwrap().1.eval(&symtab).unwrap(),
			Value::Stack(vec![69]),
			"stack variable"
		);
		assert_eq!(
			LValue::parse(b"yuki[0]").unwrap().1.eval(&symtab).unwrap(),
			Value::Array(vec![Value::Int(420)]),
			"stack variable"
		);
	}
}
