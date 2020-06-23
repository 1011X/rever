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
		LValue {
			id: v.id,
			ops: v.ops.into_iter().map(|op| op.0.into()).collect(),
		}
	}
}

impl From<ast::Deref> for Deref {
	fn from(v: ast::Deref) -> Self {
		match v {
			ast::Deref::Direct => Deref::Direct,
			ast::Deref::Index(idx) => Deref::Index(idx.into()),
			ast::Deref::Field(field) => Deref::Field(field),
		}
	}
}

impl LValue {
	pub fn get_ref<'var>(&self, t: &'var Scope) -> Result<&'var Value, &'static str> {
		t.iter()
		.rfind(|(id, _)| *id == self.id)
		.map(|(_, val)| val)
		.ok_or("could not find variable")
	}
	
	pub fn get_mut_ref<'var>(&self, t: &'var mut Scope) -> Result<&'var mut Value, &'static str> {
		t.iter_mut()
		.rfind(|(id, _)| *id == self.id)
		.map(|(_, val)| val)
		.ok_or("could not find variable")
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
				Deref::Index(expr) => match (var, expr.eval(t)?) {
					(Value::String(s), Value::Int(i)) =>
						s.chars().nth(i as usize).unwrap().into(),
					(Value::Array(a), Value::Int(i)) =>
						a.get(i as usize).unwrap().clone(),
					(_, index) => todo!("{:?}.({:?})", self.id, index)
				}
				Deref::Field(field) => match (var, field.as_str()) {
					(Value::String(s), "len") => (s.len() as i64).into(),
					_ => todo!()
				}
				Deref::Direct => todo!()
			};
		}
		
		Ok(var)
	}
}
