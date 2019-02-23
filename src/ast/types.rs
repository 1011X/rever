use crate::ast::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
	Unit,
	Bool,
	U16, I16, U32, I32, Usize, Isize,
    Char,
	Pointer(Box<Type>),
	Array(Box<Type>, usize),
	Fn(Vec<Type>, Box<Type>),
	Proc(Vec<(bool, Type)>),
	Composite(String),
}

impl Type {
	pub fn parse(s: &str) -> ParseResult<Self> {
	    // TODO check start_with's properly
	    if s.starts_with("bool") {
	        return Ok((Type::Bool, &s[4..]));
        }
        
        if s.starts_with("u16") {
            return Ok((Type::U16, &s[3..]));
        }
        
        if s.starts_with("i16") {
            return Ok((Type::I16, &s[3..]));
        }
        
        if s.starts_with("u32") {
            return Ok((Type::U32, &s[3..]));
        }
        
        if s.starts_with("i32") {
            return Ok((Type::I32, &s[3..]));
        }
        
        Err(format!("unknown type: {}", s))
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn boolean() { assert_eq!(Type::parse("bool").unwrap(), (Type::Bool, "")); }
    #[test]
    fn int() { assert_eq!(Type::parse("i32").unwrap(), (Type::I32, "")); }
}
