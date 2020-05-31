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
			params: v.params,
			ret: v.ret,
			body: v.body.into(),
		}
	}
}
