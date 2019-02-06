use crate::ast::*;

#[derive(Debug)]
pub enum Expr {
    Not(Box<Expr>),
    Group(Box<Expr>),
    
	Eq(Factor, Factor),
	Neq(Factor, Factor),
	Lte(Factor, Factor),
	Gte(Factor, Factor),
	Lt(Factor, Factor),
	Gt(Factor, Factor),
	
	And(Factor, Factor),
	Or(Factor, Factor),
	Xor(Factor, Factor),
}

impl Expr {
	pub fn eval(&self, t: &EnvTable) -> Value {
	    
	}
	
    /*
	named!(pub parse<Self>, ws!(do_parse!(
		l: call!(Factor::parse) >>
		op: alt!(
			tag!("=")
			| tag!("!=") | tag!("≠")
			// Should I really have `<=`?
			// It looks so much like an arrow...
			| tag!("<=") | tag!("≤")
			| tag!(">=") | tag!("≥")
			| tag!("<")
			| tag!(">")
			| tag!("&") | tag!("and")
			| tag!("|") | tag!("or")
			| tag!("^") | tag!("xor")
		) >>
		r: call!(Factor::parse)
		
		>> (match op {
			b"="                => BinExpr::Eq(l, r),
			b"!=" | b"\x22\x60" => BinExpr::Neq(l, r),
			b"<=" | b"\x22\x64" => BinExpr::Lte(l, r),
			b">=" | b"\x22\x65" => BinExpr::Gte(l, r),
			b"<"                => BinExpr::Lt(l, r),
			b">"                => BinExpr::Gt(l, r),
			b"&" | b"and"       => BinExpr::And(l, r),
			b"|" | b"or"        => BinExpr::Or(l, r),
			b"^" | b"xor"       => BinExpr::Xor(l, r),
			_ => unreachable!()
		})
	)));
	*/
}
