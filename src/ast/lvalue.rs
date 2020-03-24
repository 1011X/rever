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
    				
    				if tokens.next() != Some(Token::RBracket) {
    					return Err("`]` after indexing");
    				}
    				
    				ops.push(Deref::Index(expr));
    			}
    			// '.'
    			Some(Token::Period) => {
    				tokens.next();
    				
    				if let Some(Token::Ident(name)) = tokens.next() {
    					ops.push(Deref::Field(name));
    				}
    				else {
    					return Err("field name after variable");
    				}
    			}
    			_ => break
			}
		}
        
        Ok(LValue { id: name, ops })
	}
}

impl LValue {
	pub fn eval(&self, t: &Scope) -> Value {
	    t.iter().rfind(|(id, _)| *id == self.id).unwrap().1.clone()
	}
}

#[cfg(test)]
mod tests {
	use crate::tokenize::tokenize;
    use super::*;
    
    #[test]
    fn simple() {
        assert_eq!(
            LValue::parse(&tokenize("a").unwrap()).unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: Vec::new(),
            }, &[][..])
        );
    }
    #[test]
    fn direct() {
        assert_eq!(
            LValue::parse(&tokenize("a!").unwrap()).unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![Deref::Direct],
            }, &[][..])
        );
        assert_eq!(
            LValue::parse(&tokenize("a   !").unwrap()).unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![Deref::Direct],
            }, &[][..])
        );
        assert_eq!(
            LValue::parse(&tokenize("a!!").unwrap()).unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![Deref::Direct, Deref::Direct],
            }, &[][..])
        );
    }
    #[test]
    fn index() {
        assert_eq!(
            LValue::parse(&tokenize("a[0]").unwrap()).unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![Deref::Index(Term::Lit(Literal::Unsigned(0)))],
            }, &[][..])
        );
        assert_eq!(
            LValue::parse(&tokenize("a   [   0   ]").unwrap()).unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![Deref::Index(Term::Lit(Literal::Unsigned(0)))],
            }, &[][..])
        );
        assert_eq!(
            LValue::parse(&tokenize("a[0][b]").unwrap()).unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![
                    Deref::Index(Term::Lit(Literal::Unsigned(0))),
                    Deref::Index(Term::LVal(LValue {
                        id: "b".to_string(),
                        ops: Vec::new(),
                    })),
                ],
            }, &[][..])
        );
    }
    #[test]
    fn field() {
        assert_eq!(
            LValue::parse(&tokenize("a.b").unwrap()).unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![Deref::Field("b".to_string())],
            }, &[][..])
        );
        assert_eq!(
            LValue::parse(&tokenize("a   .    b").unwrap()).unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![Deref::Field("b".to_string())],
            }, &[][..])
        );
        assert_eq!(
            LValue::parse(&tokenize("a.b.c").unwrap()).unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![
                    Deref::Field("b".to_string()),
                    Deref::Field("c".to_string()),
                ],
            }, &[][..])
        );
    }
}
