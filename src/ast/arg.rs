use super::*;

#[derive(Debug)]
pub struct Arg {
	pub name: String,
	mutable: bool,
	typ: Type,
}

impl Arg {
	named!(pub parse<Self>, ws!(do_parse!(
		m: opt!(tag!("mut")) >>
		name: ident >>
		tag!(":") >>
		typ: call!(Type::parse)
		>> (Arg { name, mutable: m.is_some(), typ })
	)));
}
