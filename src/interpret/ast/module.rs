use super::*;

#[derive(Debug)]
pub struct Module {
	items: Vec<Item>,
}

impl Module {
	pub fn parse(mut tokens: &[Token]) -> ParseResult<Self> {
	    let mut items = Vec::new();
		while !tokens.is_empty() {
    		let (item, t) = Item::parse(tokens)?;
    		tokens = t;
    		items.push(item);
		}
		Ok((Module { items }, tokens))
	}
    /*
	named!(pub parse<Self>, ws!(do_parse!(
		items: many1!(Item::parse)
		>> (Module { items })
	)));
	
	pub fn verify(&mut self) {
		for &mut Item::Fn(ref mut f) in &mut self.items {
			f.verify();
		}
	}
	
	pub fn compile(&self) -> Vec<rel::Op> {
		self.items.iter()
		.map(|&Item::Fn(ref f)| f.compile())
		.collect::<Vec<_>>()
		.concat()
	}
	*/
}
