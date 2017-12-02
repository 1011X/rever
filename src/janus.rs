#![allow(dead_code)]

use std::str::{self, FromStr};

use nom::{
	IResult, Needed,
	Err, ErrorKind,
	
	multispace, digit, alpha
};

#[derive(Debug)]
pub enum Binop {
	Add,
	Sub,
	Xor,
	Lt,
	Gt,
	And,
	Or,
	Eq,
	Neq,
	Lte,
	Gte,
	Mul,
	Div,
	Mod,
}

#[derive(Debug)]
pub struct Program<'a> {
	globals: Vec<(&'a str, usize)>,
	procedures: Vec<(&'a str, Vec<Statement<'a>>)>,
}

#[derive(Debug)]
pub enum Statement<'a> {
	If(Ifstmt<'a>),
	Do(Dostmt<'a>),
	Call(Callstmt<'a>),
	Read(&'a str),
	Write(&'a str),
	Mod(Modstmt<'a>),
}

#[derive(Debug)]
pub struct Ifstmt<'a> {
	_if: Expression<'a>,
	then: Vec<Statement<'a>>,
	_else: Vec<Statement<'a>>,
	fi: Expression<'a>,
}

#[derive(Debug)]
pub struct Dostmt<'a> {
	from: Expression<'a>,
	_do: Vec<Statement<'a>>,
	_loop: Vec<Statement<'a>>,
	until: Expression<'a>,
}

#[derive(Debug)]
pub enum Callstmt<'a> {
	Call(&'a str),
	Uncall(&'a str),
}

#[derive(Debug)]
pub enum Modstmt<'a> {
	Add(Lvalue<'a>, Expression<'a>),
	Sub(Lvalue<'a>, Expression<'a>),
	Xor(Lvalue<'a>, Expression<'a>),
	Swap(Lvalue<'a>, Lvalue<'a>),
}

#[derive(Debug)]
pub struct Expression<'a> {
	min: Minexp<'a>,
	more: Vec<(Binop, Minexp<'a>)>,
}

#[derive(Debug)]
pub enum Expression<'a> {
	Constant(usize),
	Variable(&'a str, Option<usize>),
	BinOp(Box<Expression<'a>>, BinOp, Box<Expression<'a>>),
}

#[derive(Debug)]
pub enum Minexp<'a> {
	Group(Box<Expression<'a>>),
	Neg(Box<Expression<'a>>),
	Not(Box<Expression<'a>>),
	Lval(Lvalue<'a>),
	Constant(usize),
}

#[derive(Debug)]
pub struct Lvalue<'a> {
	name: &'a str,
	off: Box<Expression<'a>>,
}

named!(ident<&str>, map_res!(alpha, str::from_utf8));
named!(num<usize>, map_res!(
	map_res!(digit, str::from_utf8),
	FromStr::from_str
));

named!(binop<Binop>, alt!(
	  value!(Binop::Add, tag!("+"))
	| value!(Binop::Sub, tag!("-"))
	| value!(Binop::Xor, tag!("!"))
	| value!(Binop::Lt,  tag!("<"))
	| value!(Binop::Gt,  tag!(">"))
	| value!(Binop::And, tag!("&"))
	| value!(Binop::Or,  tag!("|"))
	| value!(Binop::Eq,  tag!("="))
	| value!(Binop::Neq, tag!("#"))
	| value!(Binop::Lte, tag!("<="))
	| value!(Binop::Gte, tag!(">="))
	| value!(Binop::Mul, tag!("*"))
	| value!(Binop::Div, tag!("/"))
	| value!(Binop::Mod, tag!("\\"))
));

named!(pub program<Program>, ws!(do_parse!(
	globals: ws!(many0!(ws!(do_parse!(
		name: ident >>
		size: opt!(ws!(delimited!(
			tag!("["),
			num,
			tag!("]")
		))) >>
	
		(name, size.unwrap_or(1))
	)))) >>
	procedures: ws!(many0!(ws!(do_parse!(
		tag!("procedure") >>
		name: ident >>
		body: ws!(many0!(stmt)) >>
	
		(name, body)
	)))) >>
	
	(Program {
		globals: globals,
		procedures: procedures,
	})
));

named!(stmt<Statement>, alt!(
	ws!(do_parse!(
		ident
	))
	| ifstmt => { Statement::If }
	| dostmt => { Statement::Do }
	| callstmt => { Statement::Call }
	| preceded!(tag!("read"), ident) => { Statement::Read }
	| preceded!(tag!("write"), ident) => { Statement::Write }
	| modstmt => { Statement::Mod }
));

named!(ifstmt<Ifstmt>, ws!(do_parse!(
	tag!("if") >>
	_if: expr >>
	then: opt!(ws!(preceded!(
		tag!("then"),
		many0!(parse_statement)
	))) >>
	_else: opt!(ws!(preceded!(
		tag!("else"),
		many0!(parse_statement)
	))) >>
	tag!("fi") >>
	fi: expr
	
	>> (Ifstmt {
		_if: _if,
		then: then.unwrap_or(Vec::new()),
		_else: _else.unwrap_or(Vec::new()),
		fi: fi,
	})
)));

named!(dostmt<Dostmt>, ws!(do_parse!(
	tag!("from") >>
	from: expr >>
	_do: opt!(ws!(preceded!(
		tag!("do"),
		many0!(stmt)
	))) >>
	_loop: opt!(ws!(preceded!(
		tag!("loop"),
		many0!(stmt)
	))) ~
	tag!("until") >>
	until: expr >>
	
	(Dostmt {
		from: from,
		_do: _do.unwrap_or(Vec::new()),
		_loop: _loop.unwrap_or(Vec::new()),
		until: until,
	})
)));

named!(callstmt<Callstmt>, alt!(
	ws!(preceded!(tag!("call"), ident)) => { Callstmt::Call }
	| ws!(preceded!(tag!("uncall"), ident)) => { Callstmt::Uncall }
));

named!(modstmt<Modstmt>, alt!(
	ws!(separated_pair!(lval, tag!(":"), lval)) => { |(l,r)| Modstmt::Swap(l, r) }
));

named!(expr<Expression>, chain!(
	m: parse_minexp ~
	b: ws!(many0!(ws!(do_parse!(
		op: binop >>
		e: minexp >>
		
		(op, e)
	)))),
	
	(Expression {min: m, more: b})
));

named!(minexp<Minexp>, alt!(
	delimited!(
		char!('('),
		chain!(
			multispace? ~
			e: parse_expression ~
			multispace?,
			
			|| Minexp::Group(Box::new(e))
		),
		char!(')')
	)
	| chain!(
		tag!("-") ~ multispace ~ e: parse_expression,
		|| Minexp::Neg(Box::new(e))
	)
	| lval => { Minexp::Lval }
	| num => { Minexp::Constant }
));

named!(lval<LValue>, ws!(do_parse!(
	name: ident >>
	off: ws!(delimited!(
		tag!("["),
		expr,
		tag!("]")
	)) >>
	
	(LValue {
		name: name,
		off: Box::new(off),
	})
));
