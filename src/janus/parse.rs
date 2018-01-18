use std::str;
//use std::collections::HashMap;
use super::Statement;
use super::interpret::{SymTab, Value};
use super::Expr;
use super::LValue;

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
pub enum Type {
	Int, Stack,
	IntArray(Vec<Option<Expr>>),
}

#[derive(Debug)]
pub struct Decl {
	pub name: String,
	pub typ: Type,
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
