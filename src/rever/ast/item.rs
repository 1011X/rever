use super::*;

#[derive(Debug)]
pub enum Item {
	//Static(bool, String, Type, ConstExpr),
	//Mod(Vec<Item>),
	Fn(Function),
}

impl Item {
	named!(pub parse<Self>, map!(Function::parse, Item::Fn));
}
