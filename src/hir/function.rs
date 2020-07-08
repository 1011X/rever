use super::*;

#[derive(Debug, Clone)]
pub struct Function {
	pub params: Vec<(String, Type)>,
	pub ret: Type,
	pub body: Expr,
}

impl From<ast::Function> for Function {
	fn from(v: ast::Function) -> Self {
		Function {
			params: v.params.into_iter()
				.map(|(n, t)| (n, t.unwrap().0))
				.collect(),
			ret: v.ret.unwrap().0,
			body: v.body.0.into(),
		}
	}
}
