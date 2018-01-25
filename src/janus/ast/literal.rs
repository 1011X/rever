use super::*;

#[derive(Debug)]
pub enum Literal {
	Nil,
	Int(i16),
	IntArray(Vec<Literal>)
}

impl Literal {
	named!(pub parse<Literal>, alt_complete!(
		value!(Literal::Nil, tag!("nil"))
		| map!(reb_parse!("^[-+]?[0-9]+"), Literal::Int)
		| map!(
			sp!(delimited!(
				tag!("{"),
				separated_nonempty_list!(tag!(","), Literal::parse),
				tag!("}")
			)),
			Literal::IntArray
		)
	));
	/*
	fn to_value(&self) -> Value {
		match *self {
			Literal::Int(i) => Value::Int(i),
			Literal::IntArray(ref vals) => Value::IntArray(vals.clone()),
		}
	}
	*/
}
