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
	    let mut var = t.iter()
			.rfind(|(id, _)| *id == self.id)
			.map(|(_, val)| val.clone())
			.ok_or("could not find variable")?;
		
		for op in &self.ops {
			var = match op {
				Deref::Index(expr) => {
					let index = expr.eval(t)?;
					match (var, index) {
						(Value::String(s), Value::Uint(i)) =>
							s.chars().nth(i as usize).unwrap().into(),
						_ => todo!()
					}
				}
				_ => todo!()
			};
		}
		
		Ok(var)
	}
}
