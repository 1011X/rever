use crate::tokenize::Token;
use super::*;

#[derive(Debug)]
pub enum Item {
	//Use(),
	//Static(bool, String, Type, ConstExpr),
	Mod(Module),
	Proc(Procedure),
	//Fn(Function),
	//Type(Type),
}

impl Item {
	pub fn parse(t: &[Token]) -> ParseResult<Self> {
		match t.first() {
			Some(Token::Proc) => {
				let (p, tx) = Procedure::parse(t)?;
				Ok((Item::Proc(p), tx))
			}
			Some(Token::Mod) => {
				let (m, tx) = Module::parse(t)?;
				Ok((Item::Mod(m), tx))
			}
			Some(_) =>
				Err(format!("unrecognized item")),
			None =>
				Err(format!("eof @ item")),
		}
	}
}
