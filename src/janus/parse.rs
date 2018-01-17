use std::str;
//use std::collections::HashMap;
use super::Statement;
use super::interpret::{SymTab, Value};

macro_rules! reb_parse {
	($i:expr, $e:expr) => {
		map_res!(
			$i,
			map_res!(re_bytes_find!($e), str::from_utf8),
			str::parse
		);
	}
}

/// removes whitespace, inline comments, and block comments
named!(pub useless, recognize!(many1!(alt_complete!(
	preceded!(
		tag!("//"),
		take_until!("\n")
	)
	| delimited!(
		tag!("/*"),
		take_until!("*/"),
		tag!("*/")
	)
	| call!(::nom::sp)
))));

/// custom macro to remove whitespace and comments
macro_rules! sp (
  ($i:expr, $($args:tt)*) => (
    {
      sep!($i, useless, $($args)*)
    }
  )
);

/// parses an identifier
named!(pub ident<String>, reb_parse!("^[A-Za-z_][A-Za-z0-9_]*"));
/// parses a boolean literal
named!(pub boolean<bool>, reb_parse!("^(true|false)"));
/// parses a string literal
named!(pub st<String>, delimited!(
    tag!("\""),
    map_res!(
        escaped_transform!(is_not!("\\\""), '\\', alt_complete!(
            value!(b"\\", tag!("\\"))
            | value!(b"\"", tag!("\""))
            | value!(b"\n", tag!("n"))
            | value!(b"\t", tag!("t"))
        )),
        String::from_utf8
    ),
    tag!("\"")
));

#[derive(Debug)]
enum Type {
	Int, Stack,
	IntArray(Vec<Option<Expr>>),
}

#[derive(Debug)]
pub struct Decl {
	pub name: String,
	typ: Type,
}

impl Decl {
	named!(pub parse<Decl>, alt_complete!(
		sp!(do_parse!(
			tag!("int") >>
			name: ident >>
			dims: sp!(many0!(delimited!(
				tag!("["),
				opt!(Expr::parse),
				tag!("]")
			)))
			>> (if dims.is_empty() {
				Decl {name, typ: Type::Int}
			} else {
				Decl {name, typ: Type::IntArray(dims)}
			})
		))
		| sp!(do_parse!(
			tag!("stack") >>
			name: ident
			>> (Decl {name, typ: Type::Stack})
		))
	));
	/*
	named!(pub parse<Decl>, ws!(do_parse!(
		typ: alt!(tag!("int") | tag!("stack")) >>
		name: ident >>
		lens: delimited!(tag!("["), opt!(Expr::parse), tag!("]"))
		
		>> 
		sp!(do_parse!(
			>> (Decl {name, _type: Type::IntArray(len)})
		))
		| sp!(do_parse!(
			>> (Decl {name, _type: Type::Int})
		))
		| sp!(do_parse!(
			>> (Decl {name, _type: Type::Stack})
		))
	)));
	*/
}

#[derive(Debug)]
pub enum Factor {
	Literal(Literal),
	LValue(LValue),
}

impl Factor {
	named!(pub parse<Factor>, alt_complete!(
		map!(Literal::parse, Factor::Literal)
		| map!(LValue::parse, Factor::LValue)
	));
}

#[derive(Debug)]
pub struct LValue {
	pub name: String,
	indices: Vec<Expr>,
}

impl LValue {
	named!(pub parse<LValue>, sp!(do_parse!(
		name: ident >>
		indices: sp!(many0!(delimited!(
			tag!("["),
			call!(Expr::parse),
			tag!("]")
		)))
		>> (LValue {name, indices})
	)));
	/*
	pub fn deref<'a>(&self, globs: &'a mut SymTab) -> &'a mut Value {
		let base = globs.get_mut(&self.name).unwrap();
		match *base {
			Value::Int(_) | Value::Stack(_) => base,
			Value::Array(ref vec) => {
				// we can unwrap bc lists are nonempty
				//let val = vec.remove(0).to_value();
				for idx in vec {
					//let idx = idx.to_value();
				}
				base
			}
		}
	}
	*/
}

#[derive(Debug)]
pub enum Literal {
	Nil,
	Int(i16),
	IntArray(Vec<Literal>)
}

impl Literal {
	named!(pub parse<Literal>, alt_complete!(
		value!(Literal::Nil, tag!("nil"))
		| map!(reb_parse!("^[-+]?[0-9]+"), Literal::Int)
		| map!(
			sp!(delimited!(
				tag!("{"),
				separated_nonempty_list!(tag!(","), Literal::parse),
				tag!("}")
			)),
			Literal::IntArray
		)
	));
	/*
	fn to_value(&self) -> Value {
		match *self {
			Literal::Int(i) => Value::Int(i),
			Literal::IntArray(ref vals) => Value::IntArray(vals.clone()),
		}
	}
	*/
}

#[derive(Debug)]
pub enum Expr {
	Factor(Factor),
	Size(LValue),
	Top(LValue),
	
	Mul(Box<Expr>, Box<Expr>),
	Div(Box<Expr>, Box<Expr>),
	Mod(Box<Expr>, Box<Expr>),
	
	Add(Box<Expr>, Box<Expr>),
	Sub(Box<Expr>, Box<Expr>),
	
	BitXor(Box<Expr>, Box<Expr>),
	BitAnd(Box<Expr>, Box<Expr>),
	BitOr(Box<Expr>, Box<Expr>),
}

impl Expr {
	named!(pub parse<Expr>, sp!(do_parse!(
		leaf: call!(Expr::leaf) >>
		prods: sp!(many0!(Expr::product)) >>
		sums: sp!(many0!(Expr::sum)) >>
		bitops: sp!(many0!(Expr::bitop))
		
		>> (leaf.to_product(prods).to_sum(sums).to_bitop(bitops))
	)));
	
	named!(leaf<Expr>, alt_complete!(
		sp!(do_parse!(
			tag!("size") >> tag!("(") >>
			lval: call!(LValue::parse) >>
			tag!(")")
			>> (Expr::Size(lval))
		))
		| sp!(do_parse!(
			tag!("top") >> tag!("(") >>
			lval: call!(LValue::parse) >>
			tag!(")")
			>> (Expr::Top(lval))
		))
		| sp!(delimited!(
			tag!("("),
			call!(Expr::parse),
			tag!(")")
		))
		| map!(Factor::parse, Expr::Factor)
	));
	
	named!(product<(&[u8], Expr)>, sp!(do_parse!(
		op: alt!(tag!("*") | tag!("/") | tag!("%")) >>
		leaf: call!(Expr::leaf)
		>> (op, leaf)
	)));
	
	named!(sum<(&[u8], Expr)>, sp!(do_parse!(
		op: alt!(tag!("+") | tag!("-")) >>
		leaf: call!(Expr::leaf) >>
		prods: sp!(many0!(Expr::product))
		
		>> (op, leaf.to_product(prods))
	)));
	
	named!(bitop<(&[u8], Expr)>, sp!(do_parse!(
		op: alt!(tag!("&") | tag!("|")) >>
		leaf: call!(Expr::leaf) >>
		prods: sp!(many0!(Expr::product)) >>
		sums: sp!(many0!(Expr::sum))
		
		>> (op, leaf.to_product(prods).to_sum(sums))
	)));
	
	fn to_product(self, mut prods: Vec<(&[u8], Expr)>) -> Expr {
		if prods.is_empty() {
			return self;
		}
		
		let (mut curr_op, last) = prods.pop().unwrap();
		let expr = prods.into_iter()
			.rev()
			.fold(last, |acc, (op, e)| {
				let res = match curr_op {
					b"*" => Expr::Mul(Box::new(e), Box::new(acc)),
					b"/" => Expr::Div(Box::new(e), Box::new(acc)),
					b"%" => Expr::Mod(Box::new(e), Box::new(acc)),
					_ => unreachable!()
				};
				curr_op = op;
				res
			});
		
		match curr_op {
			b"*" => Expr::Mul(Box::new(self), Box::new(expr)),
			b"/" => Expr::Div(Box::new(self), Box::new(expr)),
			b"%" => Expr::Mod(Box::new(self), Box::new(expr)),
			_ => unreachable!()
		}
	}
	
	fn to_sum(self, mut sums: Vec<(&[u8], Expr)>) -> Expr {
		if sums.is_empty() {
			return self;
		}
		
		let (mut curr_op, last) = sums.pop().unwrap();
		let expr = sums.into_iter()
			.rev()
			.fold(last, |acc, (op, e)| {
				let res = match curr_op {
					b"+" => Expr::Add(Box::new(e), Box::new(acc)),
					b"-" => Expr::Sub(Box::new(e), Box::new(acc)),
					_ => unreachable!()
				};
				curr_op = op;
				res
			});
		
		match curr_op {
			b"+" => Expr::Add(Box::new(self), Box::new(expr)),
			b"-" => Expr::Sub(Box::new(self), Box::new(expr)),
			_ => unreachable!()
		}
	}
	
	fn to_bitop(self, mut bitops: Vec<(&[u8], Expr)>) -> Expr {
		if bitops.is_empty() {
			return self;
		}
		
		let (mut curr_op, last) = bitops.pop().unwrap();
		let expr = bitops.into_iter()
			.rev()
			.fold(last, |acc, (op, e)| {
				let res = match curr_op {
					b"&" => Expr::BitAnd(Box::new(e), Box::new(acc)),
					b"|" => Expr::BitOr(Box::new(e), Box::new(acc)),
					b"^" => Expr::BitXor(Box::new(e), Box::new(acc)),
					_ => unreachable!()
				};
				curr_op = op;
				res
			});
		
		match curr_op {
			b"&" => Expr::BitAnd(Box::new(self), Box::new(expr)),
			b"|" => Expr::BitOr(Box::new(self), Box::new(expr)),
			b"^" => Expr::BitXor(Box::new(self), Box::new(expr)),
			_ => unreachable!()
		}
	}
	/*
	pub fn eval(&self, globs: &mut SymTab) -> Result<Value, String> {
		match *self {
			//Expr::Factor(ref fac) => fac.eval(),
			
			Expr::Size(ref lval) => {
				let value = globs[&lval.name];
				
				if let Value::Array(ref v) = value {
					Ok(Value::Int(v.len() as i16))
				} else {
					Err("not an array".to_owned())
				}
			} 
			
			_ => unimplemented!()
			//Expr::Top(ref lval) => 
		}
	}
	*/
}

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
	named!(pub parse<Pred>, sp!(do_parse!(
		leaf: call!(Pred::leaf) >>
		ands: sp!(many0!(Pred::and)) >>
		ors: sp!(many0!(Pred::or))
		>> ({
			let leaf = if ands.is_empty() { leaf }
			else {
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
	
	named!(leaf<Pred>, alt_complete!(
		map!(sp!(preceded!(tag!("!"), Pred::not)), |x| Pred::Not(Box::new(x)))
		| map!(boolean, Pred::Bool)
		| sp!(delimited!( // ( pred )
			tag!("("),
			call!(Pred::parse),
			tag!(")")
		))
		| sp!(do_parse!( // empty(x)
			tag!("empty") >> tag!("(") >>
			lval: call!(LValue::parse) >>
			tag!(")")
			>> (Pred::Empty(lval))
		))
		| sp!(do_parse!( // cmp
			left: call!(Expr::parse) >>
			cmp: alt!(
				tag!("=") | tag!("!=") | tag!(">=") | tag!("<=") | tag!(">") | tag!("<")
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
		))
	));
	
	named!(not<Pred>, alt_complete!(
		map!(sp!(preceded!(tag!("!"), Pred::not)), |x| Pred::Not(Box::new(x)))
		| map!(boolean, Pred::Bool)
		| sp!(delimited!( // ( pred )
			tag!("("),
			call!(Pred::parse),
			tag!(")")
		))
		| sp!(do_parse!( // empty(x)
			tag!("empty") >> tag!("(") >>
			lval: call!(LValue::parse) >>
			tag!(")")
			>> (Pred::Empty(lval))
		))
	));
	
	named!(or<Pred>, sp!(do_parse!(
		tag!("||") >>
		leaf: call!(Pred::leaf) >>
		ands: sp!(many0!(Pred::and))
		>> (if ands.is_empty() {
			leaf
		} else {
			let mut ands = ands;
			ands.insert(0, leaf);
			Pred::And(ands)
		})
	)));
	
	named!(and<Pred>, sp!(do_parse!(
		tag!("&&") >>
		right: call!(Pred::leaf)
		>> (right)
	)));
}

pub type Block = Vec<Statement>;

#[derive(Debug)]
pub struct Procedure {
	name: String,
	args: Vec<Decl>,
	body: Vec<Statement>
}

impl Procedure {
	named!(pub parse<Procedure>, sp!(do_parse!(
		tag!("procedure") >>
		name: ident >>
		args: delimited!(
			tag!("("),
			separated_list!(tag!(","), Decl::parse),
			tag!(")")
		) >>
		body: many1!(Statement::parse)
		
		>> (Procedure {name, args, body})
	)));
}

#[derive(Debug)]
pub enum Item {
	Global(Decl, Option<Expr>),
	Proc(Procedure),
}

impl Item {
	named!(pub parse<Item>, sp!(alt!(
		map!(Procedure::parse, Item::Proc)
		| sp!(do_parse!(
			decl: call!(Decl::parse) >>
			val: opt!(sp!(preceded!(tag!("="), Expr::parse)))
			>> (Item::Global(decl, val))
		))
	)));
}
