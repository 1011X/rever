use super::*;

#[derive(Debug, Clone)]
pub enum Deref {
	Direct,
	Index(Expr),
	Field(String),
}

#[derive(Debug, Clone)]
pub struct LValue {
	pub id: String,
	pub ops: Vec<Deref>,
}

impl From<ast::LValue> for LValue {
	fn from(v: ast::LValue) -> Self {
		unimplemented!()
	}
}

impl LValue {
	pub fn eval(&self, t: &Scope) -> Value {
	    t.iter().rfind(|(id, _)| *id == self.id).unwrap().1.clone()
	}
}
