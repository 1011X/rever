use super::*;

#[derive(Debug)]
pub enum Literal {
	Num(u16),
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
}

impl FromStr for Literal {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
    }
}
