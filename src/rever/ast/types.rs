use super::*;

#[derive(Debug, Clone)]
pub enum Type {
	Unit,
	Bool,
	U16, I16, Usize, Isize,
    Char,
	Pointer(Box<Type>),
	Array(Box<Type>, usize),
	Fn(Vec<Type>),
	Composite(String),
}

impl Type {
	named!(pub parse<Self>, ws!(alt_complete!(
		value!(Type::Unit, tag!("unit"))
		| value!(Type::Bool, tag!("bool"))
		| value!(Type::U16, tag!("u16"))
		| value!(Type::I16, tag!("i16"))
		| value!(Type::Usize, tag!("usize"))
		| value!(Type::Isize, tag!("isize"))
		| value!(Type::Char, tag!("char"))
		| map!(preceded!(tag!("^"), Type::parse), |t| Type::Pointer(Box::new(t)))
		| ws!(do_parse!(
			tag!("[") >>
			t: call!(Type::parse) >>
			tag!(";") >>
			n: num >>
			tag!("]")
			>> (Type::Array(Box::new(t), n as usize))
		))
		| do_parse!(
			tag!("fn") >>
			tag!("(") >>
			types: separated_list!(tag!(","), Type::parse) >>
			tag!(")")
			>> (Type::Fn(types))
		)
		| map!(ws!(preceded!(tag!("type"), ident)), Type::Composite)
	)));
}



