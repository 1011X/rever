use super::parse::Item;

#[derive(Debug)]
pub struct Program {
	items: Vec<Item>
}

impl Program {
	named!(pub parse<Program>, do_parse!(
		items: many1!(Item::parse)
		>> (Program {items})
	));
}
