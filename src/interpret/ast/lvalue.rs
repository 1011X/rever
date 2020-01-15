use crate::tokenize::Token;
use crate::interpret::{Scope, Value};
use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Deref {
	Direct,
	Index(Term),
	Field(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LValue {
	pub id: String,
	pub ops: Vec<Deref>,
}

impl LValue {
	pub fn parse(mut tokens: &[Token]) -> ParseResult<Self> {
	    let mut ops = Vec::new();
	    
	    let name =
	    	if let Some(Token::Ident(n)) = tokens.first() {
				tokens = &tokens[1..];
				n.clone()
			} else {
				return Err(format!("expected identifier"));
			};
	    
	    loop {
	    	match tokens.first() {
	    		// '!'
	    		Some(Token::Bang) => {
	    			tokens = &tokens[1..];
	    			ops.push(Deref::Direct);
    			}
    			// '['
    			Some(Token::LBracket) => {
    				tokens = &tokens[1..];
    				
    				let (fact, tx) = Term::parse(tokens)?;
    				tokens = tx;
    				
    				if tokens.first() != Some(&Token::RBracket) {
    					return Err(format!("expected `]` @ deref"));
    				}
    				
    				ops.push(Deref::Index(fact));
    				tokens = &tokens[1..];
    			}
    			// '.'
    			Some(Token::Period) => {
    				tokens = &tokens[1..];
    				
    				if let Some(Token::Ident(name)) = tokens.first() {
    					ops.push(Deref::Field(name.clone()));
    				}
    				else {
    					return Err(format!("expected identifier @ deref"));
    				}
    				
					tokens = &tokens[1..];
    			}
    			_ => break,
			}
		}
        
        Ok((LValue { id: name, ops }, tokens))
	}
	
	pub fn eval(&self, t: &Scope) -> Value {
	    t.iter().rfind(|(id, _)| *id == self.id).unwrap().1.clone()
	}
	
	/*
	pub fn compile(&self, st: &mut SymbolTable) -> (rel::Reg, Vec<rel::Op>) {
		// TODO maybe move some of the stuff SymbolTable::get does over here?
		st.get(&self.id)
	}
	*/
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
