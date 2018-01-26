//use super::parse::*;
use super::*;

#[derive(Debug)]
pub enum Type {
	Int, Stack,
	IntArray(Vec<Option<Expr>>),
}

#[derive(Debug)]
pub struct Decl {
	pub name: String,
	pub typ: Type,
}

impl Decl {
	named!(pub parse<Self>, alt_complete!(
		sp!(do_parse!(
			tag!("int") >>
			name: ident >>
			dims: sp!(many0!(delimited!(
				tag!("["),
				opt!(Expr::parse),
				tag!("]")
			)))
			>> (if dims.is_empty() {
				Decl {name, typ: Type::Int}
			} else {
				Decl {name, typ: Type::IntArray(dims)}
			})
		))
		| sp!(do_parse!(
			tag!("stack") >>
			name: ident
			>> (Decl {name, typ: Type::Stack})
		))
	));
}
