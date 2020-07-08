use super::*;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, Option<(Type, Span)>)>,
    pub ret: Option<(Type, Span)>,
    pub body: (Expr, Span),
}

impl Parser {
	pub fn parse_fn(&mut self) -> ParseResult<Function> {
		// keyword `fn`
		let (_, start) = self.expect(&Token::Fn)
			.ok_or("`fn`")?;
		
		// function name
		let fn_name = self.expect_ident()
			.ok_or("function name")?;
		
		// parse parameter list
		let mut params = Vec::new();
		
		// starting '('
		self.expect(&Token::LParen)
			.ok_or("`(` before parameter list")?;
		
		loop {
			// TODO add case for newline for multiline param declaration?
			match self.peek() {
				// ending ')'
				Some(Token::RParen) => break,
				
				// parse as parameter
				Some(_) => {
					// get parameter name
					let param_name = self.expect_ident()
						.ok_or("a parameter name")?;
					
					// get optional type
					let typ = match self.expect(&Token::Colon) {
						Some(_) => Some(self.parse_type()?),
						None => None,
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
						_ => Err("`,` or `)`")?
					}
				}
				
				None => Err("`,` or `)`")?
			}
		}
		self.next();
		
		// get return type
		// in case below code don't work:
		let mut ret = None;
		if self.expect(&Token::Colon).is_some() {
			ret = Some(self.parse_type()?);
		}
		/*
		let ret = self.expect(&Token::Colon)
			.and_then(|_| self.parse_type())
			.transpose()?;
		*/
		// finish function declaration
		self.expect(&Token::Newline)
			.ok_or("newline after function declaration")?;
		
		// code block section
		// TODO check result?
		let body = self.parse_expr()?;
		
		self.expect(&Token::Newline)
			.ok_or("newline after function body")?;
		
		// reached `end`
		let (_, end) = self.expect(&Token::End)
			.ok_or("`end` after function body")?;
		
		self.expect(&Token::Newline);
		
		Ok((Function { name: fn_name, params, body, ret }, start.merge(&end)))
	}
}
