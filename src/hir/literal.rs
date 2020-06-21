use super::*;

#[derive(Debug, Clone)]
pub enum Literal {
	Nil,
	Bool(bool),
	Int(i64),
	UInt(u64),
	Char(char),
	String(String),
	Array(Vec<Expr>),
	Fn(Vec<String>, Box<Expr>),
}

impl Literal {
	pub fn get_type(&self) -> Type {
		match self {
			Literal::Nil       => Type::Unit,
			Literal::Bool(_)   => Type::Bool,
			Literal::Int(_)    => Type::Int,
			Literal::UInt(_)   => Type::UInt,
			Literal::Char(_)   => Type::Char,
			Literal::String(_) => Type::String,
			
			Literal::Array(_) => todo!(),
			Literal::Fn(..)   => todo!(),
		}
	}
}

impl Eval for Literal {
	fn eval(&self, t: &Scope) -> EvalResult {
		Ok(match self {
			Literal::Nil       => Value::Nil,
			Literal::Bool(b)   => Value::Bool(*b),
			Literal::Int(n)    => Value::Int(*n),
			Literal::UInt(n)   => Value::Uint(*n),
			Literal::Char(c)   => Value::Char(*c),
			Literal::String(s) => Value::String(s.clone()),
			
			Literal::Array(arr) => Value::Array({
				let mut vec = Vec::with_capacity(arr.len());
				for expr in arr.iter() {
					vec.push(expr.eval(t)?);
				}
				vec.into_boxed_slice()
			}),
			Literal::Fn(args, ret) => todo!(),
		})
	}
}

impl From<ast::Literal> for Literal {
	fn from(v: ast::Literal) -> Self {
		match v {
			ast::Literal::Nil       => Literal::Nil,
			ast::Literal::Bool(b)   => Literal::Bool(b),
			ast::Literal::Int(i)    => Literal::Int(i),
			ast::Literal::UInt(u)   => Literal::UInt(u),
			ast::Literal::Char(c)   => Literal::Char(c),
			ast::Literal::String(s) => Literal::String(s),
			
			ast::Literal::Array(arr) =>
				Literal::Array(arr.into_iter().map(|e| e.0.into()).collect()),
			ast::Literal::Fn(args, ret) =>
				Literal::Fn(args.clone(), Box::new((*ret).0.into())),
		}
	}
}
