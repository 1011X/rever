use crate::ast::*;

#[derive(Debug)]
pub enum Literal {
	Num(i32),
	Bool(bool),
	//Char(char),
    //Str(String),
}

impl Literal {
	named!(pub parse<Self>, alt!(
		map!(num, Literal::Num)
		| value!(Literal::Bool(true), tag!("true"))
		| value!(Literal::Bool(false), tag!("false"))
		//| map!(ch, Literal::Char)
		//| map!(st, Literal::Str)
	));
	
	pub fn eval(&self, _: &VarTable) -> Value {
	    match self {
	        Literal::Num(n) => Value::Num(n),
	        Literal::Bool(b) => Value::Bool(b),
	    }
	}
	
	//pub parse(i: &[u8]) -> Result<(&[u8], Self), String>
}

impl FromStr for Literal {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
    }
}
