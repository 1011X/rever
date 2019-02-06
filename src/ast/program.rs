use super::*;

#[derive(Debug)]
pub struct Program {
	items: Vec<Item>,
}

impl Program {
    /*
	named!(pub parse<Self>, ws!(do_parse!(
		items: many1!(Item::parse)
		>> (Program { items })
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
