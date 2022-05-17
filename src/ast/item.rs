use super::*;

#[derive(Clone)]
pub enum Item {
	//Use(Path, Option<String>),
	//Static(bool, String, Type, ConstExpr),
	Mod(Module),
	Proc(Procedure),
	Fn(Function),
	//Type(Type),
}

impl Item {
	pub fn get_name(&self) -> &str {
		match self {
			Item::Mod(m)  => &m.name,
			Item::Proc(p) => &p.name,
			Item::Fn(f)   => &f.name,
		}
	}
}

impl Parser<'_> {
	pub fn parse_item(&mut self) -> ParseResult<Item> {
		let item = match self.peek() {
			Some(Token::Proc) => Item::Proc(self.parse_proc()?),
			Some(Token::Mod)  => Item::Mod(self.parse_mod()?),
			Some(Token::Fn)   => Item::Fn(self.parse_fn()?),
			
			_ => Err("a module, function, or procedure")?,
		};
		
		// mandatory newline (or EOF) after item
		match self.peek() {
			Some(Token::Newline) | None => {}
			Some(_) => Err("newline after item")?,
		}
		
		// eat all extra newlines
		self.skip_newlines();
		
		Ok(item)
	}
}

use std::fmt;
impl fmt::Debug for Item {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Item::Fn(f)   => f.fmt(fmt),
			Item::Proc(p) => p.fmt(fmt),
			Item::Mod(m)  => m.fmt(fmt),
		}
	}
}
