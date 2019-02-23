use crate::ast::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Num(i32),
    Bool(bool),
    //Char(char),
    //Str(String),
}
impl Literal {
    pub fn eval(&self, _: &ScopeTable) -> Value {
        match self {
            Literal::Num(n) => Value::Int(*n),
            Literal::Bool(b) => Value::Bool(*b),
        }
    }
    
    // num ::= -?[0-9]+
    //     ::= -?[0-9][0-9]*
    fn num(i: &str) -> ParseResult<i32> {
        let mut idx = 0;
        
        // -?
        if i.starts_with('-') {
            idx += 1;
        }
        
        // [0-9]
        if !i[idx..].starts_with(|c: char| c.is_ascii_digit()) {
            return Err("not a number".to_string())
        }
        idx += 1;
        
        // [0-9]*
        while i[idx..].starts_with(|c: char| c.is_ascii_digit()) {
            idx += 1;
        }
        
        match i32::from_str_radix(&i[..idx], 10) {
            Ok(n) => Ok((n, &i[idx..])),
            Err(_) => Err("number too big".to_string()),
        }
    }
    
    pub fn ch(mut i: &str) -> ParseResult<char> {
	    if !i.starts_with('\'') {
	        return Err("missing starting quote for char".to_string());
        }
        i = &i[1..];
	    
	    let c =
		    // escape character
		    if i.starts_with('\\') {
			    i = &i[1..];
			    
			    match &i[..1] {
				    "\\" => '\\',
				    "'"  => '\'',
				    "\n" => '\n',
				    "\t" => '\t',
				    _ => return Err("unrecognized escaped character".to_string())
			    }
		    }
		    // anything else
		    else if i.starts_with('\'') {
			    return Err("single quote needs to be escaped".to_string());
		    }
		    else {
		        match i.chars().nth(0) {
		            Some(c) => c,
		            None => return Err("invalid character".to_string()),
		        }
		    }
	    ;
	    
	    if !i.starts_with('\'') {
	        return Err("missing ending quote for char".to_string());
        }
        i = &i[1..];
	    
	    Ok((c, i))
    }
    /*
    named!(st<String>, delimited!(
        tag!("\""),
        map_res!(
            escaped_transform!(is_not!("\\\""), '\\', alt!(
                value!(b"\\", tag!("\\"))
                | value!(b"\"", tag!("\""))
                | value!(b"\n", tag!("n"))
                | value!(b"\t", tag!("t"))
            )),
            String::from_utf8
        ),
        tag!("\"")
    ));

    pub fn st(mut i: &str) -> ParseResult<String> {
	    let mut s = String::new();
	    
	    // "
	    has!(i, "\"");
	    
	    
	    
	    // "
	    has!(i, "\"");
    }
    */
    
    pub fn parse(s: &str) -> ParseResult<Self> {
        // TODO check starts_with's properly
        if s.starts_with("true")
        && !s[4..].starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_') {
            return Ok((Literal::Bool(true), &s[4..]));
        }
        
        if s.starts_with("false")
        && !s[5..].starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_') {
            return Ok((Literal::Bool(false), &s[5..]));
        }
        
        if s.starts_with(|c: char| c.is_ascii_digit() || c == '-') {
            let (n, sx) = Literal::num(s)?;
            return Ok((Literal::Num(n.into()), sx));
        }
        
        Err("invalid literal value".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn boolean() {
        assert_eq!(Literal::parse("true").unwrap(), (Literal::Bool(true), ""));
        assert_eq!(Literal::parse("false").unwrap(), (Literal::Bool(false), ""));
    }
    #[test] #[should_panic]
    fn not_bool() {
        Literal::parse("true_").unwrap();
        Literal::parse("false_").unwrap();
        Literal::parse("truea").unwrap();
        Literal::parse("falsea").unwrap();
        Literal::parse("true1").unwrap();
        Literal::parse("false0").unwrap();
    }
    #[test]
    fn int() {
        assert_eq!(Literal::parse("0").unwrap(), (Literal::Num(0), ""));
        assert_eq!(Literal::parse("-1").unwrap(), (Literal::Num(-1), ""));
        assert_eq!(Literal::parse("10").unwrap(), (Literal::Num(10), ""));
    }
}
