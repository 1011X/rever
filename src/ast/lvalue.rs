use std::fmt;
use std::error;
use std::ops::Range;

use super::*;

#[derive(Debug, Clone)]
pub struct Deref {
	pub name: Option<String>,
	// "why not just use Vec<Expr>?"
	// because there's a diff between `.field` and `.method()`.
	pub args: Option<Vec<Expr>>,
	
	// "what if both are None? what should just `ident.` do?"
	// so, i'm thinking of having suffix `.` be like a "specialized field", the
	// same way `.()` is a "specialized method". biggest difference would be
	// that methods are run (and their result calculated) at runtime, while for
	// fields the result 
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
		let id = match self.peek() {
			Some(Token::VarIdent) => self.slice().to_string(),
			_ => Err("variable name in left-value expression")?,
			//_ => return Err(LValErr::Name),
		};
		let name_span = self.span();
		self.next();
		
		let mut end_span = name_span.clone();
		
		// get deref ops
		let mut ops = Vec::new();
		while self.peek() == Some(&Token::Period) {
			self.next();
			
			// .access_name
			let name = match self.peek() {
				Some(Token::VarIdent) => {
					let name = self.slice().to_string();
					end_span = self.span();
					self.next();
					Some(name)
				}
				_ => None,
			};
			
			// ( args, )
			let args = if self.peek() == Some(&Token::LParen) {
				let mut args = Vec::new();
				self.next();
				
				loop {
					match self.peek() {
						Some(Token::RParen) => break,
						
						Some(_) => {
							args.push(self.parse_expr()?);
							
							match self.peek() {
								Some(Token::Comma) => { self.next(); }
								Some(Token::RParen) => {}
								_ => todo!()
							}
						}
						
						None => todo!(),
					}
				}
				end_span = self.span();
				self.next();
				
				Some(args)
			} else {
				None
			};
			
			ops.push(Deref { name, args });
		}
		
		Ok(LValue {
			id, ops,
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
