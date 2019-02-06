use std::str::FromStr;

use super::*;

#[derive(Debug)]
pub enum Item {
	//Static(bool, String, Type, ConstExpr),
	//Mod(Vec<Item>),
	//Fn(Function),
	Proc(Procedure),
}

/*
impl Item {
	named!(pub parse<Self>, map!(Procedure::parse, Item::Proc));
}
*/

impl FromStr for Item {
    type Err = String;
    fn from_str(s: &str) -> Result<(Self, &str), Self::Err> {
        Ok(Procedure::from_str(s)?)
    }
}
