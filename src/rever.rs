#![allow(dead_code)]

use std::str;


macro_rules! reb_parse {
	($i:expr, $e:expr) => {
		map_res!(
			$i,
			map_res!(re_bytes_find!($e), str::from_utf8),
			str::parse
		);
	}
}


named!(ident<String>, reb_parse!("^[A-Za-z_][A-Za-z0-9_]*"));
named!(num<u16>, reb_parse!("^[-+]?[1-9][0-9]*"));


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

named!(binexpr<BinExpr>, alt!(
	ws!(do_parse!(
		l: factor >>
		tag!("=") >>
		r: factor >>
		
		(BinExpr::Eq(l, r))
	))
	| ws!(do_parse!(
		l: factor >>
		alt!(tag!("!=") | tag!("≠")) >>
		r: factor >>
		
		(BinExpr::Neq(l, r))
	))
	| ws!(do_parse!(
		l: factor >>
		tag!("<") >>
		r: factor >>
		
		(BinExpr::Lt(l, r))
	))
	// Should I really have `<=` and `>=`? They look so much like arrows.
	| ws!(do_parse!(
		l: factor >>
		alt!(tag!("<=") | tag!("≤")) >>
		r: factor >>
		
		(BinExpr::Lte(l, r))
	))
	| ws!(do_parse!(
		l: factor >>
		tag!(">") >>
		r: factor >>
		
		(BinExpr::Gt(l, r))
	))
	| ws!(do_parse!(
		l: factor >>
		alt!(tag!(">=") | tag!("≥")) >>
		r: factor >>
		
		(BinExpr::Gte(l, r))
	))
	| ws!(do_parse!(
		l: factor >>
		tag!("&") >>
		r: factor >>
		
		(BinExpr::And(l, r))
	))
	| ws!(do_parse!(
		l: factor >>
		tag!("|") >>
		r: factor >>
		
		(BinExpr::Or(l, r))
	))
	| ws!(do_parse!(
		l: factor >>
		tag!("^") >>
		r: factor >>
		
		(BinExpr::Xor(l, r))
	))
));

#[derive(Debug)]
pub enum Type {
	Unit,
	Bool,
	U16, I16, Usize, Isize,
	Pointer(Box<Type>),
	Array(Box<Type>, usize),
	Fn(Vec<Type>),
	Composite(String),
}

named!(type_<Type>, alt!(
	map!(tag!("unit"), |_| Type::Unit)
	| map!(tag!("bool"), |_| Type::Bool)
	| map!(tag!("u16"), |_| Type::U16)
	| map!(tag!("i16"), |_| Type::I16)
	| map!(tag!("usize"), |_| Type::Usize)
	| map!(tag!("isize"), |_| Type::Isize)
	| map!(ws!(preceded!(tag!("^"), type_)), |t| Type::Pointer(Box::new(t)))
	| ws!(do_parse!(
		tag!("[") >> t: type_ >> tag!(";") >> n: num >> tag!("]") >>
		
		(Type::Array(Box::new(t), n as usize))
	))
	| ws!(do_parse!(
		tag!("fn") >>
		p: ws!(delimited!(
			tag!("("),
			separated_list!(tag!(","), type_),
			tag!(")")
		)) >>
		
		(Type::Fn(p))
	))
	| map!(ws!(preceded!(tag!("type"), ident)), Type::Composite)
));

#[derive(Debug)]
pub struct Procedure {
	name: String,
	args: Vec<(bool, String, Type)>,
	code: Vec<Statement>,
}

named!(proc_<Procedure>, ws!(do_parse!(
	tag!("proc") >>
	name: ident >>
	args: ws!(delimited!(
		tag!("("),
		ws!(separated_list!(
			tag!(","),
			ws!(do_parse!(
				m: opt!(tag!("var")) >>
				name: ident >>
				tag!(":") >>
				t: type_ >>
				
				(m.is_some(), name, t)
			))
		)),
		tag!(")")
	)) >>
	code: block >>
	
	// some extra work can be done here if it's really needed
	(Procedure {
		name: name,
		args: args,
		code: code.into_iter()
			.collect(),
	})
)));


#[derive(Debug)]
pub enum Item {
	//Static(bool, String, Type, ConstExpr),
	//Mod(Vec<Item>),
	Proc(Procedure),
}

named!(item<Item>, map!(proc_, Item::Proc));

#[derive(Debug)]
pub enum Factor {
	Lit(Literal),
	LVal(LValue),
}

named!(factor<Factor>, alt!(
	map!(lit, Factor::Lit)
	| map!(lval, Factor::LVal)
));

#[derive(Debug)]
pub enum Literal {
	Num(u16),
}

named!(lit<Literal>, map!(num, Literal::Num));
/*
#[derive(Debug)]
enum Number {
	//Unknown(u16),
	U16(u16),
	//I16(i16),
	//Usize(u16),
	//Isize(i16),
}

named!(num<Number>, alt!(
	map_res!(
		re_bytes_find!("^[-+]?[1-9][0-9]*"),
		str::from_utf8_unchecked
	)
	| re_bytes_find!("^0x[A-Fa-f0-9_]+")
	| re_bytes_find!("^0b[01_]+")
));
*/

#[derive(Debug)]
enum Deref {
	Direct,
	Indexed(Factor),
	Field(String),
}

#[derive(Debug)]
pub struct LValue {
	id: String,
	ops: Vec<Deref>,
}

named!(lval<LValue>, ws!(do_parse!(
	id: ident >>
	ops: ws!(many0!(alt!(
		map!(tag!("*"), |_| Deref::Direct)
		| ws!(delimited!(
			tag!("["),
			map!(factor, Deref::Indexed),
			tag!("]")
		))
		| ws!(preceded!(tag!("."), map!(ident, Deref::Field)))
	))) >>
	
	(LValue {
		id: id,
		ops: ops,
	})
)));

#[derive(Debug)]
pub enum FlatOp {
	Add(Factor),
	Sub(Factor),
}

#[derive(Debug)]
pub enum Statement {
	Let(String, Option<Type>, Literal),
	Var(String, Option<Type>, Literal),
	Drop(String, Literal),
	
	Not(LValue),
	Neg(LValue),
	
	RotLeft(LValue, Factor),
	RotRight(LValue, Factor),
	
	CCNot(LValue, Factor, Factor),
	Xor(LValue, Vec<Factor>),
	
	Add(LValue, Vec<FlatOp>),
	Sub(LValue, Vec<FlatOp>),
	
	Swap(LValue, LValue),
	CSwap(Factor, LValue, LValue),
	
	If(BinExpr, Vec<Statement>, Option<Vec<Statement>>, BinExpr),
	
	From(BinExpr, Option<Vec<Statement>>, Option<Vec<Statement>>, BinExpr),
	
	Call(LValue, Vec<Factor>),
	Uncall(LValue, Vec<Factor>),
	
	//Switch(String, Vec<String, Vec<Statement>>),
	//Unsafe(Vec<Statement>),
}

named!(args<Vec<Factor> >, ws!(delimited!(
	tag!("("),
	ws!(separated_list!(
		tag!(","),
		factor
	)),
	tag!(")")
)));

named!(block<Vec<Statement>>, ws!(delimited!(
	tag!("{"),
	ws!(many0!(
		ws!(terminated!(stmt, tag!(";")))
	)),
	tag!("}")
)));

//named!(stmt<Statement>, value!(Statement::Not(LValue {id: String::from("hi"), ops: vec![]})));

named!(stmt<Statement>, alt!(
	ws!(preceded!(tag!("!"), map!(lval, Statement::Not)))
	| ws!(preceded!(tag!("-"), map!(lval, Statement::Neg)))
	| ws!(do_parse!(
		left: lval >>
		tag!("<>") >>
		right: lval >>
		
		(Statement::Swap(left, right))
	))
	| ws!(do_parse!(
		dest: lval >>
		tag!("^=") >>
		lctrl: factor >>
		tag!("&") >>
		rctrl: factor >>
		
		(Statement::CCNot(dest, lctrl, rctrl))
	))
	| ws!(do_parse!(
		control: factor >>
		tag!("?") >>
		left: lval >>
		tag!("<>") >>
		right: lval >>
		
		(Statement::CSwap(control, left, right))
	))
	| ws!(do_parse!(
		l: lval >>
		m: tag!("<<=") >>
		r: factor >>
		
		(Statement::RotLeft(l, r))
	))
	| ws!(do_parse!(
		l: lval >>
		m: tag!(">>=") >>
		r: factor >>
		
		(Statement::RotRight(l, r))
	))
	| ws!(do_parse!(
		l: lval >>
		tag!("^=") >>
		r: separated_nonempty_list!(
			tag!("^"),
			factor
		) >>
		
		(Statement::Xor(l, r))
	))
	| ws!(do_parse!(
		l: lval >>
		tag!("+=") >>
		r: factor >>
		m: ws!(many0!(alt!(
			ws!(preceded!(tag!("+"), factor)) => {FlatOp::Add}
			| ws!(preceded!(tag!("-"), factor)) => {FlatOp::Sub}
		))) >>
		
		(Statement::Add(l, {
			let mut m = m;
			m.insert(0, FlatOp::Add(r));
			m
		}))
	))
	// stuff is backwards here because we're distributing the negative sign
	| ws!(do_parse!(
		l: lval >>
		tag!("-=") >>
		r: factor >>
		m: ws!(many0!(alt!(
			ws!(preceded!(tag!("+"), factor)) => {FlatOp::Sub}
			| ws!(preceded!(tag!("-"), factor)) => {FlatOp::Add}
		))) >>
		
		(Statement::Add(l, {
			let mut m = m;
			m.insert(0, FlatOp::Sub(r));
			m
		}))
	))
	/*
	| ws!(do_parse!(
		tag!("unsafe") >>
		b: parse_block >>
		
		(Statement::Unsafe(b.into_iter()
			.filter_map(|s| s)
			.collect()
		))
	))
	*/
	| ws!(do_parse!(
		tag!("do") >>
		name: lval >>
		args: args >>
		(Statement::Call(name, args))
	))
	| ws!(do_parse!(
		tag!("undo") >>
		name: lval >>
		args: args >>
		(Statement::Uncall(name, args))
	))
	| ws!(do_parse!(
		tag!("let") >>
		name: ident >>
		ty: opt!(ws!(preceded!(tag!(":"), type_))) >>
		tag!("=") >>
		l: lit >>
		
		(Statement::Let(name, ty, l))
	))
	| ws!(do_parse!(
		tag!("var") >>
		name: ident >>
		ty: opt!(ws!(preceded!(tag!(":"), type_))) >>
		tag!("=") >>
		l: lit >>
		
		(Statement::Var(name, ty, l))
	))
	| ws!(do_parse!(
		tag!("drop") >>
		name: ident >>
		tag!("=") >>
		l: lit >>
		
		(Statement::Drop(name, l))
	))
	| ws!(do_parse!(
		tag!("if") >>
		p: binexpr >>
		t: block >>
		e: opt!(ws!(preceded!(tag!("else"), block))) >>
		tag!("fi") >>
		a: binexpr >>
		
		(Statement::If(p, t, e, a))
	))
	| ws!(do_parse!(
		tag!("from") >>
		a: binexpr >>
		d: opt!(block) >>
		l: opt!(block) >>
		tag!("until") >>
		p: binexpr >>
		
		(Statement::From(a, d, l, p))
	))
));

named!(pub program<Vec<Item> >, complete!(
	ws!(many0!(item))
));
