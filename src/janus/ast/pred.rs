use super::*;

#[derive(Debug)]
pub enum Pred {
	Bool(bool),
	Empty(LValue),
	
	Not(Box<Pred>),
	And(Vec<Pred>),
	Or(Vec<Pred>),
	
	Eq(Expr, Expr),
	Neq(Expr, Expr),
	Gt(Expr, Expr),
	Lt(Expr, Expr),
	Gte(Expr, Expr),
	Lte(Expr, Expr),
}

impl Pred {
	named!(pub parse<Self>, sp!(do_parse!(
		leaf: call!(Pred::leaf) >>
		ands: many0!(Pred::and) >>
		ors: many0!(Pred::or)
		>> ({
			let leaf = if ands.is_empty() {
				leaf
			} else {
				let mut ands = ands;
				ands.insert(0, leaf);
				Pred::And(ands)
			};
	
			if ors.is_empty() { leaf }
			else {
				let mut ors = ors;
				ors.insert(0, leaf);
				Pred::Or(ors)
			}
		})
	)));
	
	named!(leaf<Self>, sp!(alt_complete!(
		map!(preceded!(tag!("!"), Pred::not), |x| Pred::Not(Box::new(x)))
		| map!(boolean, Pred::Bool)
		| delimited!( // ( pred )
			tag!("("),
			call!(Pred::parse),
			tag!(")")
		)
		| do_parse!( // empty(x)
			tag!("empty") >> tag!("(") >>
			lval: call!(LValue::parse) >>
			tag!(")")
			>> (Pred::Empty(lval))
		)
		| do_parse!( // cmp
			left: call!(Expr::parse) >>
			cmp: alt!(
				tag!("=")
				| tag!("!=")
				| tag!(">=")
				| tag!("<=")
				| tag!(">")
				| tag!("<")
			) >>
			right: call!(Expr::parse)
			>> (match cmp {
				b"=" => Pred::Eq(left, right),
				b"!=" => Pred::Neq(left, right),
				b">=" => Pred::Gte(left, right),
				b"<=" => Pred::Lte(left, right),
				b">" => Pred::Gt(left, right),
				b"<" => Pred::Lt(left, right),
				_ => unreachable!()
			})
		)
	)));
	
	named!(not<Self>, sp!(alt_complete!(
		map!(preceded!(tag!("!"), Pred::not), |x| Pred::Not(Box::new(x)))
		| map!(boolean, Pred::Bool)
		| delimited!( // ( pred )
			tag!("("),
			call!(Pred::parse),
			tag!(")")
		)
		| do_parse!( // empty(x)
			tag!("empty") >> tag!("(") >>
			lval: call!(LValue::parse) >>
			tag!(")")
			>> (Pred::Empty(lval))
		)
	)));
	
	named!(or<Self>, sp!(do_parse!(
		tag!("||") >>
		leaf: call!(Pred::leaf) >>
		ands: many0!(Pred::and)
		>> (if ands.is_empty() {
			leaf
		} else {
			let mut ands = ands;
			ands.insert(0, leaf);
			Pred::And(ands)
		})
	)));
	
	named!(and<Self>, sp!(preceded!(
		tag!("&&"),
		call!(Pred::leaf)
	)));
}
