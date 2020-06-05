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

impl Eval for LValue {
	fn eval(&self, t: &Scope) -> EvalResult {
	    t.iter()
    	.rfind(|(id, _)| *id == self.id)
    	.map(|(_, val)| val.clone())
    	.ok_or("could not find variable")
	}
}
