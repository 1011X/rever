use super::*;

#[derive(Clone, Debug)]
pub enum Item {
	//Use(Path, Option<String>),
	//Static(bool, String, Type, ConstExpr),
	Mod(Module),
	Proc(Procedure),
	Fn(Function),
	//Type(Type),
	
	InternProc(&'static str, fn(Box<[Value]>), fn(Box<[Value]>)),
}

impl Item {
	pub fn get_name(&self) -> &str {
		match self {
			Item::Mod(m) => &m.name,
			Item::Proc(p) => &p.name,
			Item::Fn(f) => &f.name,
			Item::InternProc(name, _, _) => name,
		}
	}
}

impl Parser {
	pub fn parse_item(&mut self) -> ParseResult<Item> {
		let item = match self.peek() {
			Some(Token::Proc) => {
				let p = self.parse_proc()?;
				Item::Proc(p)
			}
			Some(Token::Mod) => {
				let m = self.parse_mod()?;
				Item::Mod(m)
			}
			Some(Token::Fn) => {
				let f = self.parse_fn()?;
				Item::Fn(f)
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
