use super::*;

#[derive(Debug, Clone)]
pub struct Arg {
	pub name: String,
	pub mutable: bool,
	pub typ: Type,
}

impl Arg {
	pub fn parse(mut s: &str) -> ParseResult<Self> {
	    let mut mutable = false;
	    
	    if s.starts_with("var")
	    && s[3..].starts_with(|c: char| !c.is_ascii_alphanumeric()) {
	        mutable = true;
	        s = s[3..].trim_start();
	    }
	    
	    let (name, mut s) = ident(s)?;
	    s = s.trim_start();
	    
	    if !s.starts_with(":") {
	        return Err(format!(
	            "expected `:`, found {}",
	            s.chars().nth(0).map(|c| c.to_string()).unwrap_or("eof".to_string())
            ));
	    }
	    s = s[1..].trim_start();
	    
	    let (typ, s) = Type::parse(s)?;
	    
	    Ok((Arg {
	        name: name.to_string(),
	        mutable,
	        typ,
        }, s))
	}
}
