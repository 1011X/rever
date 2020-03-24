use super::*;

#[derive(Debug, Clone)]
pub enum Item {
	//Use(),
	//Static(bool, String, Type, ConstExpr),
	Mod(Module),
	Proc(Procedure),
	//Fn(Function),
	//Type(Type),
}

impl Parse for Item {
	fn parse(tokens: &mut Tokens) -> ParseResult<Self> {
		match tokens.peek() {
			Some(Token::Proc) => {
				let p = Procedure::parse(tokens)?;
				Ok(Item::Proc(p))
			}
			Some(Token::Mod) => {
				let m = Module::parse(tokens)?;
				Ok(Item::Mod(m))
			}
			//t => Err(("item", t.clone())),
			_ => Err("a module or procedure")
		}
	}
}

impl From<Module> for Item {
	fn from(m: Module) -> Item { Item::Mod(m) }
}

impl From<Procedure> for Item {
	fn from(p: Procedure) -> Item { Item::Proc(p) }
}
