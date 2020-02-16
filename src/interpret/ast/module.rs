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
	pub fn parse(mut tokens: &[Token]) -> ParseResult<Self> {
		// `mod` keyword
		if tokens.first() != Some(&Token::Mod) {
			return Err(format!("expected `mod` @ module start"));
		}
		tokens = &tokens[1..];
		
		// get name
		let name = match tokens.first() {
			Some(Token::Ident(name)) => name.clone(),
			Some(t) =>
				return Err(format!("expected identifier, got {:?} {:?}", t, &tokens[1..])),
			None =>
				return Err(format!("eof @ mod name")),
		};
		
		// get newline
		if tokens.first() != Some(&Token::Newline) {
			return Err(format!("expected newline @ module start"));
		}
		tokens = &tokens[1..];
		
		// parse as many items as possible
	    let mut items = Vec::new();
		while tokens.first() != Some(&Token::End) {
    		let (item, t) = Item::parse(tokens)?;
    		tokens = t;
    		items.push(item);
		}
		
		Ok((Module { name, items }, tokens))
	}
}
