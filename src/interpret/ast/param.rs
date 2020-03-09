use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
	pub name: String,
	pub mutable: bool,
	pub typ: Type,
}

impl Param {
	pub fn parse(tokens: &mut Tokens) -> ParseResult<Self> {
	    // check mutability
	    let mut mutable = false;
	    
	    if tokens.peek() == Some(&Token::Var) {
	    	mutable = true;
	    	tokens.next();
    	}
    	
    	// get parameter name
    	let name = match tokens.next() {
    		Some(Token::Ident(n)) => n,
    		_ => return Err("param name")
		};
		
		// ':'
		if tokens.next() != Some(Token::Colon) {
			return Err("`:` @ param");
		}
	    
	    // get its type
	    let typ = Type::parse(tokens)?;
	    
	    Ok(Param { name, mutable, typ })
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
