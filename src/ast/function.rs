use super::*;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub ret: Type,
    pub body: BlockExpr,
}

// param ::= ident [":" type]
// params ::= [ param { "," param } [","] ]
// fn ::= "fn" ident "(" params ")" ":" type
//            complex-expr
//        "end"
//    ::= "fn" ident "(" params ")" ":" type "=" line-expr
impl Parser<'_> {
	pub fn parse_fn(&mut self) -> ParseResult<Function> {
		// keyword `fn`
		self.expect(Token::Fn).ok_or("`fn`")?;
		
		// function name
		let fn_name = match self.peek() {
			Some(Token::VarIdent) => self.slice().to_string(),
			_ => Err("function name")?,
		};
		self.next();
		
		// parse parameter list
		let mut params = Vec::new();
		
		// starting '('
		self.expect(Token::LParen)
			.ok_or("`(` before parameter list")?;
		
		loop {
			// TODO add case for newline for multiline param declaration?
			match self.peek() {
				// ending ')'
				Some(Token::RParen) => break,
				
				// parse as parameter
				Some(_) => {
					// get parameter name
					let param_name = match self.peek() {
						Some(Token::VarIdent) => self.slice().to_string(),
						_ => Err("a parameter name")?,
					};
					self.next();
					
					// get optional type
					let typ = match self.expect(Token::Colon) {
						Some(_) => self.parse_type()?,
						None => Type::Infer,
					};
					
					// ensure param name is unique
					// TODO leave until hir translation?
					for (name, _) in &params {
						if *name == param_name {
							eprintln!(
								"A parameter name in `fn {}` was repeated: {:?}",
								fn_name, param_name
							);
							Err("parameter names to be unique")?;
						}
					}
					
					// push to list of parameters
					params.push((param_name, typ));
					
					match self.peek() {
						Some(Token::Comma) => { self.next(); }
						Some(Token::RParen) => {}
						_ => Err("`,` or `)`")?,
					}
				}
				
				None => Err("`,` or `)`")?,
			}
		}
		self.next();
		
		// get return type
		self.expect(Token::Colon)
			.ok_or("`:` after function parameters")?;
			//.ok_or(FuncErr::NoRetType)?;
		
		let ret = self.parse_type()?;
		
		// choose parsing style based on next token
		let body = match self.peek() {
			// fn f(x): _
			//     <block-expr>
			Some(Token::Newline) => {
				self.next();
				
				let body = self.parse_block_expr()?;
		
				self.expect(Token::Newline)
					.ok_or("newline after function body")?;
				
				body
			}
			
			// fn f(x): _ = <inline-expr>
			Some(Token::Eq) => {
				self.next();
				
				let body = self.parse_expr()?;
				
				self.expect(Token::Newline)
					.ok_or("newline after function body")?;
				
				BlockExpr::Inline(body)
			}
			
			_ => Err("`=` or newline after function declaration")?,
		};
		
		Ok(Function { name: fn_name, params, body, ret })
	}
}
