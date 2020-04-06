use super::*;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub body: Expr,
}

impl Parse for Function {
	fn parse(tokens: &mut Tokens) -> ParseResult<Self> {
		// keyword `fn`
		if tokens.next() != Some(Token::Fn) {
			return Err("`fn`");
		}
		
		// get function name
		let fn_name = match tokens.next() {
			Some(Token::Ident(n)) => n,
			_ => return Err("function name")
		};
		
		// parse parameter list
		let mut params = Vec::new();
		
		// starting '('
		if tokens.next() != Some(Token::LParen) {
			return Err("`(` before parameter list");
		}
		
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
					if tokens.next() != Some(Token::Colon) {
						return Err("`:` after parameter name");
					}
					
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
		
		// check for newline
		if tokens.next() != Some(Token::Newline) {
			return Err("newline after parameter list");
		}
		
		// code block section
		// TODO check result of Expr::parse
		let body = Expr::parse(tokens)?;
		
		// check for `end`
		if tokens.next() != Some(Token::End) {
			return Err("`end`");
		}
		
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
		
		Ok(Function { name: fn_name, params, body })
	}
}
