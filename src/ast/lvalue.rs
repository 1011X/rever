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

// TODO ponder: is `var name` and `drop name` within statements part of a bigger pattern?
impl Parse for LValue {
	fn parse(tokens: &mut Tokens) -> ParseResult<Self> {
	    let mut ops = Vec::new();
	    
	    // get lval name
	    let name = match tokens.next() {
	    	Some(Token::Ident(n)) => n,
	    	_ => return Err("a variable name")
		};
	    
	    loop {
	    	match tokens.peek() {
	    		// '!'
	    		Some(Token::Bang) => {
	    			tokens.next();
	    			ops.push(Deref::Direct);
    			}
    			// '['
    			Some(Token::LBracket) => {
    				tokens.next();
    				let expr = Expr::parse(tokens)?;
    				
    				tokens.expect(&Token::RBracket)
    					.ok_or("`]` after index expression")?;
    				
    				ops.push(Deref::Index(expr));
    			}
    			// '.'
    			Some(Token::Period) => {
    				tokens.next();
    				
    				if let Some(Token::Ident(name)) = tokens.next() {
    					ops.push(Deref::Field(name));
    				} else {
    					return Err("field name after variable");
    				}
    			}
    			_ => break
			}
		}
        
        Ok(LValue { id: name, ops })
	}
}
