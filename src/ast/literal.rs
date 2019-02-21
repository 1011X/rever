use crate::ast::*;

#[derive(Debug, Clone)]
pub enum Literal {
    Num(i32),
    Bool(bool),
    //Char(char),
    //Str(String),
}

// num ::= [-+]?[0-9]+
//     ::= [-+]?[0-9][0-9]*
impl Literal {
    pub fn eval(&self, _: &ScopeTable) -> Value {
        match self {
            Literal::Num(n) => Value::Int(*n),
            Literal::Bool(b) => Value::Bool(*b),
        }
    }
    
    fn num(i: &str) -> ParseResult<i16> {
        let mut idx = 0;
        
        // [-+]?
        if i.starts_with(|c| c == '-' || c == '+') {
            idx += 1;
        }
        
        // [0-9]
        if !i.starts_with(|c: char| c.is_ascii_digit()) {
            return Err("not a number".to_string())
        }
        idx += 1;
        
        // [0-9]*
        loop {
            if !i.starts_with(|c: char| c.is_ascii_digit()) {
                break;
            }
            
            idx += 1;
        }
        
        match i16::from_str_radix(&i[..idx], 10) {
            Ok(n) => Ok((n, &i[idx..])),
            Err(_) => Err("number too big".to_string()),
        }
    }
    
    pub fn parse(s: &str) -> ParseResult<Self> {
        // TODO check starts_with's properly
        if s.starts_with("true") {
            return Ok((Literal::Bool(true), &s[4..]));
        }
        
        if s.starts_with("false") {
            return Ok((Literal::Bool(false), &s[5..]));
        }
        
        if s.starts_with(|c: char| c.is_ascii_digit()) {
            let (n, sx) = Literal::num(s)?;
            return Ok((Literal::Num(n.into()), sx));
        }
        
        Err("invalid literal value".to_string())
    }
    
    //pub parse(i: &[u8]) -> Result<(&[u8], Self), String>
    /*
    named!(pub parse<Self>, alt!(
        map!(num, Literal::Num)
        | value!(Literal::Bool(true), tag!("true"))
        | value!(Literal::Bool(false), tag!("false"))
        //| map!(ch, Literal::Char)
        //| map!(st, Literal::Str)
    ));
    */
}
