use std::fmt;
use std::error;
use std::ops::Range;

use super::*;

#[derive(Debug, Clone)]
pub enum Deref {
	Direct,
	Field(String),
	Index(Expr),
}

#[derive(Debug, Clone)]
pub struct LValue {
	pub id: String,
	pub ops: Vec<Deref>,
	pub span: Range<usize>,
}

#[derive(Debug, Clone)]
pub enum LValErr {
	Name,
	EndParen,
	//InvalidDeref,
	
	Index(Box<ExprErr>),
}

impl fmt::Display for LValErr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Name => f.write_str("variable name in left-value expression"),
			Self::EndParen => f.write_str("`)` after index expression"),
			//Self::InvalidDeref => f.write_str("`*`, `(`, or field name"),
			
			Self::Index(_) => f.write_str("invalid indexing expression"),
		}
	}
}

impl error::Error for LValErr {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		if let Self::Index(err) = self {
			Some(err)
		} else {
			None
		}
	}
}

impl Parser<'_> {
	pub fn parse_lval(&mut self) -> ParseResult<LValue> {
		
		// get lval name
		let name = match self.peek() {
			Some(Token::VarIdent) => self.slice().to_string(),
			_ => Err("variable name in left-value expression")?,
			//_ => return Err(LValErr::Name),
		};
		let name_span = self.span();
		self.next();
		
		let mut end_span = name_span.clone();
		
		let mut ops = Vec::new();
		while self.peek() == Some(&Token::Period) {
			self.next();
			
			match self.peek() {
				// .*
				Some(Token::Star) => {
					self.next();
					end_span = self.span();
					ops.push(Deref::Direct);
				}
				
				// .(expr)
				Some(Token::LParen) => {
					self.next();
					
					let expr = self.parse_expr()?;
						//.map_err(|e| LValErr::Index(Box::new(e)))?;
					
					self.expect(Token::RParen)
						.ok_or("`)` after index expression")?;
						//.ok_or(LValErr::EndParen)?;
					
					end_span = self.span();
					
					ops.push(Deref::Index(expr));
				}
				
				// .field
				Some(Token::VarIdent) => {
					let name = self.slice().to_string();
					end_span = self.span();
					self.next();
					
					ops.push(Deref::Field(name));
				}
				
				_ => return Err("`*`, `(`, or field name")?,
			}
		}
		
		Ok(LValue {
			id: name,
			ops,
			span: name_span.start .. end_span.end,
		})
	}
}

impl Eval for LValue {
	fn eval(&self, t: &StackFrame) -> EvalResult<Value> {
		let var = t.get(self)?.clone();
		
		/*for op in &self.ops {
			var = match op {
				Deref::Index(expr) => match (var, expr.eval(t)?) {
					(Value::String(s), Value::Int(i)) =>
						s.chars().nth(i as usize).unwrap().into(),
					
					(Value::Array(a), Value::Int(i)) =>
						a.get(i as usize).unwrap().clone(),
					
					(_, index) => todo!("{}.({})", self.id, index)
				}
				Deref::Field(field) => match (var, field.as_str()) {
					(Value::String(s), "len") => (s.len() as i64).into(),
					(Value::Array(arr), "len") => Value::Uint(arr.len() as u64),
					(l, r) => {
						eprintln!("ops for {}: {:?}", &self.id, &self.ops);
						eprintln!("{:#?}", t);
						todo!("{}.{}", l, r);
					}
				}
				Deref::Direct => todo!()
			};
		}*/
		
		Ok(var)
	}
}
