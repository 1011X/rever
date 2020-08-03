use super::*;

/// A named module holding multiple items.
///
/// An AST node that takes a name and zero or more items.
#[derive(Clone, Debug)]
pub struct Module {
	pub name: String,
	pub items: Vec<Item>,
}

impl Module {
	pub fn new(name: String, items: Vec<Item>) -> Self {
		Module { name, items }
	}
}

impl Parser<'_> {
	pub fn parse_mod(&mut self) -> ParseResult<Module> {
		self.expect(&Token::Mod).ok_or("`mod`")?;
		
		let name = self.expect_ident()
			.ok_or("module name")?;
		
		self.expect(&Token::Newline)
			.ok_or("newline after module name")?;
		
		// parse as many items as possible
	    let mut items = Vec::new();
		loop {
			match self.peek() {
				Some(Token::End) => break,
				Some(_) => items.push(self.parse_item()?),
				None => Err("an item or `end`")?,
			}
		}
		self.next();
		
		Ok(Module { name, items })
	}
}


/*
impl From<Vec<ast::Item>> for Module {
	fn from(items: Vec<ast::Item>) -> Self {
		let mut map = HashMap::new();
		for item in items {
			match item {
				ast::Item::Proc(p) =>
					map.insert(p.name.clone(), Item::Proc(p.into())),
				ast::Item::Mod(m) =>
					map.insert(m.name.clone(), Item::Mod(m.into())),
				ast::Item::Fn(f) =>
					map.insert(f.name.clone(), Item::Fn(f.into())),
			};
		}
		Module(map)
	}
}
*/
