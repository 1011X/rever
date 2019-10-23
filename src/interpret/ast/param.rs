use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
	pub name: String,
	pub mutable: bool,
	pub typ: Type,
}

impl Param {
	pub fn parse(mut tokens: &[Token]) -> ParseResult<Self> {
	    // check mutability
	    let mut mutable = false;
	    
	    if tokens.first() == Some(&Token::Var) {
	    	mutable = true;
	    	tokens = &tokens[1..];
    	}
    	
    	// read parameter name
    	let name =
    		if let Some(Token::Ident(n)) = tokens.first() {
    			tokens = &tokens[1..];
    			n.clone()
			}
			else {
				return Err(format!("expected identifier @ param"));
			};
		
		// ':'
		if tokens.first() != Some(&Token::Colon) {
			return Err(format!("expected `:` @ param"));
		}
		tokens = &tokens[1..];
	    
	    // read type
	    let (typ, tx) = Type::parse(tokens)?;
	    tokens = tx;
	    
	    Ok((Param { name, mutable, typ }, tokens))
	}
}


#[cfg(test)]
mod tests {
	use crate::tokenize::tokenize;
    use super::*;
    
    #[test]
    fn simple() {
        assert_eq!(
            Param::parse(&tokenize("a:bool").unwrap()).unwrap(),
            (Param {
                name: "a".to_string(),
                mutable: false,
                typ: Type::Bool,
            }, &[][..])
        );
    }
    #[test]
    fn mutable() {
        assert_eq!(
            Param::parse(&tokenize("var a:bool").unwrap()).unwrap(),
            (Param {
                name: "a".to_string(),
                mutable: true,
                typ: Type::Bool,
            }, &[][..])
        );
    }
    #[test]
    fn bad_mutable() {
        assert_eq!(
            Param::parse(&tokenize("vara:bool").unwrap()).unwrap(),
            (Param {
                name: "vara".to_string(),
                mutable: false,
                typ: Type::Bool,
            }, &[][..])
        );
    }
    #[test]
    fn whitespace() {
        assert_eq!(
            Param::parse(&tokenize("var   abc   :    bool").unwrap()).unwrap(),
            (Param {
                name: "abc".to_string(),
                mutable: true,
                typ: Type::Bool,
            }, &[][..])
        );
    }
}
