use super::*;

pub enum Item {
	//Use(),
	//Static(bool, String, Type, ConstExpr),
	Mod(Module),
	Proc(Procedure),
	Fn(Function),
	//Type(Type),
	InternalProc(Box<dyn Fn(Box<[Value]>)>)
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
			Some(Token::Fn) => {
				let f = Function::parse(tokens)?;
				Ok(Item::Fn(f))
			}
			_ => Err("a module, function, or procedure")
		}
	}
}

impl From<Module> for Item {
	fn from(m: Module) -> Item { Item::Mod(m) }
}

impl From<Procedure> for Item {
	fn from(p: Procedure) -> Item { Item::Proc(p) }
}

impl From<Function> for Item {
	fn from(f: Function) -> Item { Item::Fn(f) }
}

impl From<fn()> for Item {
	fn from(f: fn()) -> Item {
		Item::InternalProc(Box::new(move |b| {
			assert!(b.is_empty(), "more than 0 arguments given");
			f();
		}))
	}
}
