use super::*;

/// A named module holding multiple items.
///
/// An AST node that takes a name and zero or more items.
pub struct Module {
	pub name: String,
	pub items: Vec<Item>,
}

impl Module {
	/// Constructs a new module with the given name and items.
	pub fn new<T: ToString>(name: T, items: Vec<Item>) -> Module {
		Module { name: name.to_string(), items }
	}
}

impl Parse for Module {
	fn parse(tokens: &mut Tokens) -> ParseResult<Self> {
		// `mod` keyword
		if tokens.next() != Some(Token::Mod) {
			return Err("`mod`");
		}
		
		// get name
		let name = match tokens.next() {
			Some(Token::Ident(name)) => name,
			//t => return Err(("module name", t)),
			_ => return Err("module name"),
		};
		
		// get newline
		if tokens.next() != Some(Token::Newline) {
			return Err("newline after module name");
		}
		
		// parse as many items as possible
	    let mut items = Vec::new();
	    
		loop {
			match tokens.peek() {
				Some(Token::End) => {
					tokens.next();
					break;
				}
				Some(_) => {
					let item = Item::parse(tokens)?;
					items.push(item);
				}
				None => return Err("an item or `end`")
			}
		}
		
		// the optional `mod` in `end mod`
		if tokens.peek() == Some(&Token::Mod) {
			tokens.next();
			
			// the optional name of procedure after `end mod`
			if tokens.peek() == Some(&Token::Ident(name.clone())) {
				tokens.next();
			}
		}
		
		// the likely newline afterwards
		if tokens.peek() == Some(&Token::Newline) { tokens.next(); }
		
		Ok(Module { name, items })
	}
}
