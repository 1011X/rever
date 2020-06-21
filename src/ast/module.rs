use super::*;

/// A named module holding multiple items.
///
/// An AST node that takes a name and zero or more items.
#[derive(Clone, Debug)]
pub struct Module {
	pub name: String,
	pub items: Vec<(Item, Span)>,
}

impl Parser {
	pub fn parse_mod(&mut self) -> ParseResult<Module> {
		let (_, start) = self.expect(&Token::Mod)
			.ok_or("`mod`")?;
		
		let name = self.expect_ident()
			.ok_or("module name")?;
		
		self.expect(&Token::Newline)
			.ok_or("newline after module name")?;
		
		// parse as many items as possible
	    let mut items = Vec::new();
		loop {
			match self.peek() {
				Some(Token::End) =>
					break,
				Some(_) =>
					items.push(self.parse_item()?),
				None =>
					return Err("an item or `end`"),
			}
		}
		let (_, end) = self.next().unwrap();
		
		// the likely newline afterwards
		self.expect(&Token::Newline);
		
		Ok((Module { name, items }, start.merge(&end)))
	}
}
