use super::*;
use super::super::interpret::SymTab;

#[derive(Debug)]
pub enum Pred {
	Bool(bool),
	Empty(String),
	
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
			id: ident >>
			tag!(")")
			>> (Pred::Empty(id))
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
			id: ident >>
			tag!(")")
			>> (Pred::Empty(id))
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
	
	pub fn eval(&self, symtab: &SymTab) -> Result<bool, String> {
		Ok(match *self {
			Pred::Bool(b)      => b,
			Pred::Empty(ref v) => v.is_empty(),
			
			Pred::Not(ref p)
				=> !p.eval(symtab)?,
			Pred::And(ref v) => v.iter()
				.map(|p| p.eval(symtab))
				.collect::<Result<Vec<_>, _>>()?
				.into_iter()
				.all(|b| b),
			Pred::Or(ref v) => v.iter()
				.map(|p| p.eval(symtab))
				.collect::<Result<Vec<_>, _>>()?
				.into_iter()
				.any(|b| b),
			
			Pred::Eq(ref e0, ref e1)
				=> e0.eval(symtab)? == e1.eval(symtab)?,
			Pred::Neq(ref e0, ref e1)
				=> e0.eval(symtab)? != e1.eval(symtab)?,
			Pred::Gt(ref e0, ref e1)
				=> e0.eval(symtab)? > e1.eval(symtab)?,
			Pred::Lt(ref e0, ref e1)
				=> e0.eval(symtab)? < e1.eval(symtab)?,
			Pred::Gte(ref e0, ref e1)
				=> e0.eval(symtab)? >= e1.eval(symtab)?,
			Pred::Lte(ref e0, ref e1)
				=> e0.eval(symtab)? <= e1.eval(symtab)?,
		})
	}
}
