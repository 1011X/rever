use super::*;

#[derive(Debug)]
pub struct Program {
	items: Vec<Item>
}

impl Program {
	named!(pub parse<Self>, do_parse!(
		items: many1!(Item::parse)
		>> (Program {items})
	));
}
