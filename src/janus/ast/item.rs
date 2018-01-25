use super::*;

#[derive(Debug)]
pub enum Item {
	Global(Decl, Option<Expr>),
	Proc(Procedure),
}

impl Item {
	named!(pub parse<Item>, sp!(alt!(
		map!(Procedure::parse, Item::Proc)
		| sp!(do_parse!(
			decl: call!(Decl::parse) >>
			val: opt!(sp!(preceded!(tag!("="), Expr::parse)))
			>> (Item::Global(decl, val))
		))
	)));
}
