use super::*;
use super::super::interpret::{self, SymTab, Value};
use rel;

#[derive(Debug)]
pub enum Literal {
	Nil,
	Int(i16),
	Array(Vec<Literal>)
}

impl Literal {
	named!(pub parse<Self>, alt_complete!(
		value!(Literal::Nil, tag!("nil"))
		| map!(reb_parse!("^[-+]?[0-9]+"), Literal::Int)
		| map!(
			sp!(delimited!(
				tag!("{"),
				separated_nonempty_list!(tag!(","), Literal::parse),
				tag!("}")
			)),
			Literal::Array
		)
	));
	
	/*
	// TODO needs way to choose register
	fn compile(&self) -> Vec<rel::Op> {
		match *self {
			Literal::Nil => vec![],
			Literal::Int(i) => vec![
				Op::Immediate(_, i >> 8),
				Op::LRotateImm(_, 8),
				Op::Immediate(_, i & 0xFF)
			],
			Literal::IntArray(ref mut v) => {
				let mut acc = vec![];
				
				for lit in v {
					acc.append(&mut lit.compile());
					// TODO
				}
				
				acc
			}
		}
	}
	*/
	
	pub fn to_value(&self) -> Value {
		match *self {
			Literal::Nil => Value::Stack(vec![]),
			Literal::Int(i) => Value::Int(i),
			Literal::Array(ref lits) => {
				let vals = lits.iter()
					.map(|l| l.to_value())
					.collect();
				Value::Array(vals)
			}
		}
	}
}
