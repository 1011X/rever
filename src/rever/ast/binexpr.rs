use super::*;

#[derive(Debug)]
pub enum BinExpr {
	Eq(Factor, Factor),
	Neq(Factor, Factor),
	Lt(Factor, Factor),
	Lte(Factor, Factor),
	Gt(Factor, Factor),
	Gte(Factor, Factor),
	And(Factor, Factor),
	Or(Factor, Factor),
	Xor(Factor, Factor),
}

impl BinExpr {
	named!(pub parse<Self>, ws!(alt_complete!(
		do_parse!( // a = b
			l: call!(Factor::parse) >>
			tag!("=") >>
			r: call!(Factor::parse)
			>> (BinExpr::Eq(l, r))
		)
		| do_parse!( // a != b, a ≠ b
			l: call!(Factor::parse) >>
			alt!(tag!("!=") | tag!("≠")) >>
			r: call!(Factor::parse)
			>> (BinExpr::Neq(l, r))
		)
		| do_parse!( // a < b
			l: call!(Factor::parse) >>
			tag!("<") >>
			r: call!(Factor::parse)
			>> (BinExpr::Lt(l, r))
		)
		// Should I really have `<=` and `>=`? They look so much like arrows.
		| do_parse!( // a <= b, a ≤ b
			l: call!(Factor::parse) >>
			alt!(tag!("<=") | tag!("≤")) >>
			r: call!(Factor::parse)
			>> (BinExpr::Lte(l, r))
		)
		| do_parse!( // a > b
			l: call!(Factor::parse) >>
			tag!(">") >>
			r: call!(Factor::parse)
			>> (BinExpr::Gt(l, r))
		)
		| do_parse!( // a >= b, a ≥ b
			l: call!(Factor::parse) >>
			alt!(tag!(">=") | tag!("≥")) >>
			r: call!(Factor::parse)
			>> (BinExpr::Gte(l, r))
		)
		| do_parse!( // a & b, a and b
			l: call!(Factor::parse) >>
			alt!(tag!("&") | tag!("and")) >>
			r: call!(Factor::parse)
			>> (BinExpr::And(l, r))
		)
		| do_parse!( // a | b, a or b
			l: call!(Factor::parse) >>
			alt!(tag!("|") | tag!("or")) >>
			r: call!(Factor::parse)
			>> (BinExpr::Or(l, r))
		)
		| do_parse!( // a ^ b, a xor b
			l: call!(Factor::parse) >>
			alt!(tag!("^") | tag!("xor")) >>
			r: call!(Factor::parse)
			>> (BinExpr::Xor(l, r))
		)
	)));
}
