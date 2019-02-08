use std::str::FromStr;

use super::*;

#[derive(Debug, Clone)]
pub struct Arg {
	pub name: String,
	pub mutable: bool,
	pub typ: Type,
}

impl Arg {
	named!(pub parse<Self>, ws!(do_parse!(
		m: opt!(tag!("var")) >>
		name: ident >>
		tag!(":") >>
		typ: call!(Type::parse)
		>> (Arg { name, mutable: m.is_some(), typ })
	)));
	
	pub fn parse(s: &str) -> Result<(Self, &str), String> {
	    let mut idx = 0;
	    let mut mutable = false;
	    
	    if s.starts_with("var") {
	        mutable = true;
	        idx += 3;
	    }
	    
	    let name = ident(s)?;
	    
	    if s.starts_with(':') {
	        
	    }
	}
}
