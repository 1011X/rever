use std::fmt;

use super::*;

pub enum Item {
	//Use(),
	//Static(bool, String, Type, ConstExpr),
	Mod(Module),
	Proc(Procedure),
	Fn(Function),
	//Type(Type),
	InternalProc(fn(Box<[Value]>))
}

impl Parse for Item {
	fn parse(tokens: &mut Tokens) -> ParseResult<Self> {
		match tokens.peek() {
			Some(Token::Proc) => {
				let p = Procedure::parse(tokens)?;
				Ok(Item::Proc(p))
			}
			Some(Token::Mod) => {
				let m = Module::parse(tokens)?;
				Ok(Item::Mod(m))
			}
			Some(Token::Fn) => {
				let f = Function::parse(tokens)?;
				Ok(Item::Fn(f))
			}
			_ => Err("a module, function, or procedure")
		}
	}
}

impl From<Module> for Item {
	fn from(m: Module) -> Item { Item::Mod(m) }
}

impl From<Procedure> for Item {
	fn from(p: Procedure) -> Item { Item::Proc(p) }
}

impl From<Function> for Item {
	fn from(f: Function) -> Item { Item::Fn(f) }
}

impl fmt::Debug for Item {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Item::Mod(m) => m.fmt(f),
			Item::Proc(p) => p.fmt(f),
			Item::Fn(func) => func.fmt(f),
			Item::InternalProc(_) => f.write_str("<internal proc>"),
		}
	}
}
