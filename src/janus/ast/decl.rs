use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
	Int, Stack,
	IntArray(Vec<Option<Expr>>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Decl {
	pub name: String,
	pub typ: Type,
}

impl Decl {
	named!(pub parse<Self>, sp!(alt_complete!(
		do_parse!(
			tag!("int") >>
			name: ident >>
			dims: many0!(delimited!(
				tag!("["),
				opt!(Expr::parse),
				tag!("]")
			))
			>> (if dims.is_empty() {
				Decl {name, typ: Type::Int}
			} else {
				Decl {name, typ: Type::IntArray(dims)}
			})
		)
		| do_parse!(
			tag!("stack") >>
			name: ident
			>> (Decl {name, typ: Type::Stack})
		)
	)));
}
