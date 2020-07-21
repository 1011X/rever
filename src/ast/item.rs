use super::*;

#[derive(Clone, Debug)]
pub enum Item {
	//Use(),
	//Static(bool, String, Type, ConstExpr),
	Mod(Module),
	Proc(Procedure),
	Fn(Function),
	//Type(Type),
}

impl Parser {
	pub fn parse_item(&mut self) -> ParseResult<Item> {
		let item = match self.peek() {
			Some(Token::Proc) => {
				let (p, span) = self.parse_proc()?;
				(Item::Proc(p), span)
			}
			Some(Token::Mod) => {
				let (m, span) = self.parse_mod()?;
				(Item::Mod(m), span)
			}
			Some(Token::Fn) => {
				let (f, span) = self.parse_fn()?;
				(Item::Fn(f), span)
			}
			_ => Err("a module, function, or procedure")?,
		};
		
		// mandatory newline (or EOF) after item
		match self.peek() {
			Some(Token::Newline) | None => {}
			Some(_) => return Err("newline after item"),
		}
		
		// eat all extra newlines
		while self.expect(&Token::Newline).is_some() {}
		
		Ok(item)
	}
}

impl Item {
	pub fn get_name(&self) -> &str {
		match self {
			Item::Mod(m) => &m.name,
			Item::Proc(p) => &p.name,
			Item::Fn(f) => &f.name,
		}
	}
}
/*
impl From<Module> for Item {
	fn from(m: (Module, Span)) -> Item { (Item::Mod(m.0), m.1) }
}

impl From<Procedure> for Item {
	fn from(p: (Procedure, Span)) -> Item { (Item::Proc(p.0), p.1) }
}

impl From<Function> for Item {
	fn from(f: (Function, Span)) -> Item { (Item::Fn(f.0), f.1) }
}
*/
