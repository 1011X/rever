use super::*;

#[derive(Debug)]
pub enum Item {
	Global(Decl, Option<Expr>),
	Proc(Procedure),
}

impl Item {
	named!(pub parse<Self>, sp!(alt_complete!(
		map!(Procedure::parse, Item::Proc)
		| do_parse!(
			decl: call!(Decl::parse) >>
			val: opt!(preceded!(tag!("="), Expr::parse))
			>> (Item::Global(decl, val))
		)
	)));
}
