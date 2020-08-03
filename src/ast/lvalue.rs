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
}

// TODO ponder: is `var name` and `drop name` within statements part of a bigger pattern?
impl Parser<'_> {
	pub fn parse_lval(&mut self) -> ParseResult<LValue> {
	    let mut ops = Vec::new();
	    
	    // get lval name
	    let name = self.expect_ident()
	    	.ok_or("variable name in left-value expression")?;
    	
	    loop {
	    	match self.peek() {
	    		// '!'
	    		Some(Token::Bang) => {
	    			self.next();
	    			ops.push(Deref::Direct);
    			}
    			// '.'
    			Some(Token::Period) => {
    				self.next();
    				
    				match self.peek() {
    					Some(Token::LParen) => {
    						self.next();
    						
							let expr = self.parse_expr()?;
							
							self.expect(&Token::RParen)
								.ok_or("`)` after index expression")?;
							
							ops.push(Deref::Index(expr));
    					}
    					Some(Token::Ident(_)) => {
    						let name = self.expect_ident().unwrap();
	    					ops.push(Deref::Field(name));
    					}
    					_ => Err("field name or `(`")?,
    				}
    			}
    			
    			_ => break,
			}
		}
        
        Ok(LValue { id: name, ops })
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
