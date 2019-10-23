use crate::tokenize::Token;
use super::*;

#[derive(Debug)]
pub enum Item {
	//Static(bool, String, Type, ConstExpr),
	//Mod(Vec<Item>),
	Proc(Procedure),
	//Fn(Function),
	Statement(Statement),
	//Type(Type),
}

impl Item {
	pub fn parse(t: &[Token]) -> ParseResult<Self> {
	    if let Ok((p, tx)) = Procedure::parse(t) {
		    Ok((Item::Proc(p), tx))
	    }
	    else {
	    	let (s, tx) = Statement::parse(t)?;
	    	Ok((Item::Statement(s), tx))
    	}
	}
}
