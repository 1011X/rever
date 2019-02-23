use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
	pub name: String,
	pub mutable: bool,
	pub typ: Type,
}

impl Param {
	pub fn parse(mut s: &str) -> ParseResult<Self> {
	    // check mutability
	    let mut mutable = false;
	    
	    if s.starts_with("var")
	    && s[3..].starts_with(|c: char| !c.is_ascii_alphanumeric()) {
	        mutable = true;
	        s = s[3..].trim_start();
	    }
	    
	    // read name
	    let (name, sx) = ident(s)?;
	    s = sx.trim_start();
	    
	    // check type separator
	    if !s.starts_with(":") {
	        return Err(format!(
	            "expected `:`, found {}",
	            s.chars().nth(0)
	                .map(|c| c.to_string())
	                .unwrap_or("eof".to_string())
            ));
	    }
	    s = s[1..].trim_start();
	    
	    // read type
	    let (typ, sx) = Type::parse(s)?;
	    
	    Ok((Param {
	        name: name.to_string(),
	        mutable,
	        typ,
        }, sx))
	}
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple() {
        assert_eq!(
            Param::parse("a:bool").unwrap(),
            (Param {
                name: "a".to_string(),
                mutable: false,
                typ: Type::Bool,
            }, "")
        );
    }
    #[test]
    fn mutable() {
        assert_eq!(
            Param::parse("var a:bool").unwrap(),
            (Param {
                name: "a".to_string(),
                mutable: true,
                typ: Type::Bool,
            }, "")
        );
    }
    #[test]
    fn bad_mutable() {
        assert_eq!(
            Param::parse("vara:bool").unwrap(),
            (Param {
                name: "vara".to_string(),
                mutable: false,
                typ: Type::Bool,
            }, "")
        );
    }
    #[test]
    fn whitespace() {
        assert_eq!(
            Param::parse("var   abc   :    bool").unwrap(),
            (Param {
                name: "abc".to_string(),
                mutable: true,
                typ: Type::Bool,
            }, "")
        );
    }
}
