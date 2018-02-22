use super::*;
//use super::super::interpret::Value;
use super::super::compile::State;
use rel;

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
	
	pub fn compile(&self, state: &mut State, code: &mut Vec<rel::Op>) -> rel::Reg {
		use rel::Op;
		let r = state.get_reg(code);
		match *self {
			Literal::Nil => {}
			Literal::Int(i) => match i {
				0       => {}
				1...255 => code.push(Op::XorImm(r, i as u8)),
				i       => code.extend(vec![
					Op::XorImm(r, (i >> 8) as u8),
					Op::LRotImm(r, 8),
					Op::XorImm(r, i as u8)
				])
			},
			
			_ => unimplemented!()
			/*
			Literal::IntArray(ref mut v) => {
				let mut acc = vec![];
				
				for lit in v {
					acc.append(&mut lit.compile());
					// TODO
				}
				
				acc
			}
			*/
		}
		r
	}
	/*
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
	*/
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
