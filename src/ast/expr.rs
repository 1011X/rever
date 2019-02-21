/*!
Expressions in Rever have 5 levels of precendence. From strongest to weakest:
1. Parentheses
2. Function calls
3. Unary operators: `not`, `-`, maybe `@`, `!`, and `~`
4. Exponential operators: `**`, `<<`, `>>`, `shl`, `shr`, `rol`, `ror`
5. Multiplicative operators: `*`, `/`, `div`, `mod`, `as`, `and`
6. Additive operators: `+`, `-`, `or`, `xor`
7. Relational operators: `=`, `!=`/`≠`, `<`, `>`, `<=`/`≤`, `>=`/`≥`, `in`
*/

use crate::ast::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Not(Box<Expr>),
    Group(Box<Expr>),
    
	Eq(Factor, Factor),
	Neq(Factor, Factor),
	//Lte(Factor, Factor),
	//Gte(Factor, Factor),
	Lt(Factor, Factor),
	Gt(Factor, Factor),
	
	And(Box<Expr>, Box<Expr>),
	Or(Box<Expr>, Box<Expr>),
	//Xor(Factor, Factor),
}

impl Expr {
	pub fn eval(&self, t: &ScopeTable) -> Value {
	    use self::Expr::*;
	    match self {
	        Group(e) => e.eval(t),
	        Not(e) => match e.eval(t) {
	            Value::Bool(true) => Value::Bool(false),
	            Value::Bool(false) => Value::Bool(true),
	            _ => panic!("tried negating non-boolean expression")
            }
            
            Eq(l, r) => Value::from(l.eval(t) == r.eval(t)),
            Neq(l, r) => Value::from(l.eval(t) != r.eval(t)),
            
            Lt(l, r) => match (l.eval(t), r.eval(t)) {
                (Value::Int(l), Value::Int(r)) => Value::from(l < r),
                _ => panic!("tried comparing non-integer values")
            }
            Gt(l, r) => match (l.eval(t), r.eval(t)) {
                (Value::Int(l), Value::Int(r)) => Value::from(l > r),
                _ => panic!("tried comparing non-integer values")
            }
            
            And(l, r) => match (l.eval(t), r.eval(t)) {
                (Value::Bool(l), Value::Bool(r)) => Value::from(l && r),
                _ => panic!("tried ANDing non-boolean expressions")
            }
            Or(l, r) => match (l.eval(t), r.eval(t)) {
                (Value::Bool(l), Value::Bool(r)) => Value::from(l || r),
                _ => panic!("tried ANDing non-boolean expressions")
            }
	    }
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
