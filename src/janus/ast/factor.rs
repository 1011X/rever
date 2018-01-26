use super::*;

#[derive(Debug)]
pub enum Factor {
	Literal(Literal),
	LValue(LValue),
}

impl Factor {
	named!(pub parse<Factor>, alt_complete!(
		map!(Literal::parse, Factor::Literal)
		| map!(LValue::parse, Factor::LValue)
	));
}
