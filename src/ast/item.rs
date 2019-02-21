use super::*;

#[derive(Debug)]
pub enum Item {
	//Static(bool, String, Type, ConstExpr),
	//Mod(Vec<Item>),
	//Fn(Function),
	Proc(Procedure),
}

impl Item {
	pub fn parse(s: &str) -> ParseResult<Self> {
	    let (p, s) = Procedure::parse(s)?;
	    Ok((Item::Proc(p), s))
	}
}
