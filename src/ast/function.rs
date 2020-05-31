use super::*;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub ret: Type,
    pub body: Expr,
}

impl Parse for Function {
	fn parse(tokens: &mut Tokens) -> ParseResult<Self> {
		// keyword `fn`
		tokens.expect(&Token::Fn)
			.ok_or("`fn`")?;
		
		// get function name
		let fn_name = match tokens.next() {
			Some(Token::Ident(n)) => n,
			_ => return Err("function name")
		};
		
		// parse parameter list
		let mut params = Vec::new();
		
		// starting '('
		tokens.expect(&Token::LParen)
			.ok_or("`(` before parameter list")?;
		
		loop {
			// TODO add case for newline for multiline param declaration?
			match tokens.peek() {
				// ending ')'
				Some(Token::RParen) => {
					tokens.next();
					break;
				}
				
				None => return Err("`,` or `)`"),
				
				// parse as parameter
				_ => {
					// get parameter name
					let param_name = match tokens.next() {
						Some(Token::Ident(n)) => n,
						_ => return Err("a parameter name")
					};
					
					// ':'
					tokens.expect(&Token::Colon)
						.ok_or("`:` after parameter name")?;
					
					// get type
					let typ = Type::parse(tokens)?;
					
					// ensure param name is unique
					for (name, _) in &params {
						if *name == param_name {
							eprintln!(
								"A parameter name in `fn {}` was repeated: {:?}",
								fn_name, param_name
							);
							return Err("parameter name to be unique");
						}
					}
					
					// push to list of parameters
					params.push((param_name, typ));
					
					match tokens.next() {
						Some(Token::RParen) => break,
						Some(Token::Comma) => {}
						_ => return Err("`,` or `)`")
					}
				}
			}
		}
		
		// get return type
		tokens.expect(&Token::Colon)
			.ok_or("`:` after function parameter list")?;
		
		let ret = Type::parse(tokens)?;
		
		// check for newline
		tokens.expect(&Token::Newline)
			.ok_or("newline after return type")?;
		
		// code block section
		// TODO check result of Expr::parse
		let body = Expr::parse(tokens)?;
		
		// check for newline
		tokens.expect(&Token::Newline)
			.ok_or("newline after function body")?;
		
		// check for `end`
		tokens.expect(&Token::End)
			.ok_or("`end` after function body")?;
		
		// the optional `fn` in `end fn`
		if tokens.peek() == Some(&Token::Fn) {
			tokens.next();
			
			// the optional name of function after `end fn`
			if tokens.peek() == Some(&Token::Ident(fn_name.clone())) {
				tokens.next();
			}
		}
		
		// the likely newline afterwards
		if tokens.peek() == Some(&Token::Newline) { tokens.next(); }
		
		Ok(Function { name: fn_name, params, body, ret })
	}
}
