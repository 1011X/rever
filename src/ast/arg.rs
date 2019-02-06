use super::*;

#[derive(Debug, Clone)]
pub struct Arg {
	pub name: String,
	pub mutable: bool,
	pub typ: Type,
}

impl Arg {
	named!(pub parse<Self>, ws!(do_parse!(
		m: opt!(tag!("var")) >>
		name: ident >>
		tag!(":") >>
		typ: call!(Type::parse)
		>> (Arg { name, mutable: m.is_some(), typ })
	)));
}
