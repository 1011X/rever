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
	    let name = tokens.expect_ident()
	    	.ok_or("variable name in left-value expression")?;
	    
	    loop {
	    	match tokens.peek() {
	    		// '!'
	    		Some(Token::Bang) => {
	    			tokens.next();
	    			ops.push(Deref::Direct);
    			}
    			// '.'
    			Some(Token::Period) => {
    				tokens.next();
    				
    				match tokens.next() {
    					Some(Token::LParen) => {
							let expr = Expr::parse(tokens)?;
							
							tokens.expect(&Token::RParen)
								.ok_or("`)` after index expression")?;
							
							ops.push(Deref::Index(expr));
    					}
    					Some(Token::Ident(name)) => {
	    					ops.push(Deref::Field(name));
    					}
    					_ => return Err("field name or `(` after variable"),
    				}
    			}
    			_ => break
			}
		}
        
        Ok(LValue { id: name, ops })
	}
}
