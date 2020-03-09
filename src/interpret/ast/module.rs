// TODO: Perhaps modules should be orthogonal to files?
// Items for a module *could* be defined in an entirely separate file by giving
// a path for, say, a procedure name, instead of just an identifier.
// Think about the "layer" concept of organization: separating code into
// "features" and appending new code at the end, so that stuff at the start of
// the file is used as a base and stuff at the bottom builds off of that.

use super::*;

#[derive(Debug)]
pub struct Module {
	name: String,
	items: Vec<Item>,
}

impl Module {
	pub fn parse(tokens: &mut Tokens) -> ParseResult<Self> {
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
		
		// TODO check for optional `mod` after `end`
		
		Ok(Module { name, items })
	}
}
