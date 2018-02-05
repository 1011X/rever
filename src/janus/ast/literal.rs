use super::*;
use super::super::interpret::Value;
//use rel;

#[derive(Debug, PartialEq, Eq)]
pub enum Literal {
	Nil,
	Int(i16),
	Array(Vec<Literal>)
}

impl Literal {
	named!(pub parse<Self>, alt_complete!(
		value!(Literal::Nil, tag!("nil"))
		| map!(reb_parse!("^[-+]?[0-9]+"), Literal::Int)
		| sp!(do_parse!(
			tag!("{") >>
			lits: separated_nonempty_list!(tag!(","), Literal::parse) >>
			tag!("}")
			>> (Literal::Array(lits))
		))
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

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn parse() {
		assert_eq!(
			Literal::parse(b"1").unwrap().1,
			Literal::Int(1),
			"single digit int"
		);
		assert_eq!(
			Literal::parse(b"12383").unwrap().1,
			Literal::Int(12383),
			"multi-digit int"
		);
		assert_eq!(
			Literal::parse(b"+123").unwrap().1,
			Literal::Int(123),
			"int with positive prefix"
		);
		assert_eq!(
			Literal::parse(b"-383").unwrap().1,
			Literal::Int(-383),
			"int with negative prefix"
		);
	}
}
