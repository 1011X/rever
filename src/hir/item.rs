use super::*;

#[derive(Clone)]
pub enum Item {
	//Static(bool, String, Type, ConstExpr),
	Mod(Module),
	Proc(Procedure),
	Fn(Function),
	//Type(Type),
	InternProc(Proc, Proc),
}

impl From<Module> for Item {
	fn from(m: Module) -> Item { Item::Mod(m) }
}

impl From<Procedure> for Item {
	fn from(p: Procedure) -> Item { Item::Proc(p) }
}

impl From<Function> for Item {
	fn from(p: Function) -> Item { Item::Fn(p) }
}

impl From<ast::Item> for Item {
	// keep in mind this will lose track of the name of the item.
	fn from(v: ast::Item) -> Self {
		match v {
			ast::Item::Mod(m) => Item::Mod(m.into()),
			ast::Item::Proc(p) => Item::Proc(p.into()),
			ast::Item::Fn(p) => Item::Fn(p.into()),
		}
	}
}

impl std::fmt::Debug for Item {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Item::Mod(m) => m.fmt(f),
			Item::Proc(p) => p.fmt(f),
			Item::Fn(func) => func.fmt(f),
			Item::InternProc(..) => f.write_str("<internal procedure>"),
		}
	}
}
