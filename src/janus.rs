use std::str;
use std::str::FromStr;

use nom::{
	IResult, Needed,
	Err, ErrorKind,
	
	multispace, digit, alpha
};

#[derive(Debug)]
pub enum Binop {
	Plus,
	Minus,
	Xor,
	LessThan,
	GreaterThan,
	And,
	Or,
	Equal,
	NotEqual,
	LessThanOrEqual,
	GreaterThanOrEqual,
	Times,
	DividedBy,
	Modulus,
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
	  tag!("+")  => { |_| Binop::Plus               }
	| tag!("-")  => { |_| Binop::Minus              }
	| tag!("!")  => { |_| Binop::Xor                }
	| tag!("<")  => { |_| Binop::LessThan           }
	| tag!(">")  => { |_| Binop::GreaterThan        }
	| tag!("&")  => { |_| Binop::And                }
	| tag!("|")  => { |_| Binop::Or                 }
	| tag!("=")  => { |_| Binop::Equal              }
	| tag!("#")  => { |_| Binop::NotEqual           }
	| tag!("<=") => { |_| Binop::LessThanOrEqual    }
	| tag!(">=") => { |_| Binop::GreaterThanOrEqual }
	| tag!("*")  => { |_| Binop::Times              }
	| tag!("/")  => { |_| Binop::DividedBy          }
	| tag!("\\") => { |_| Binop::Modulus            }
));

named!(pub parse_program<Program>, chain!(
	multispace? ~
	globals: many0!(chain!(
		name: ident ~
		multispace? ~
		size: delimited!(
			char!('['),
			chain!(multispace? ~ n: num ~ multispace?, || n),
			char!(']')
		)?,
	
		|| (name, size.unwrap_or(1))
	)) ~
	procedures: many0!(chain!(
		tag!("procedure") ~ multispace ~
		name: ident ~ multispace ~
		body: many0!(parse_statement),
	
		|| (name, body)
	)),
	
	|| Program {
		globals: globals,
		procedures: procedures,
	}
));

named!(parse_statement<Statement>, alt!(
	parse_ifstmt   => { |s| Statement::If(s) }
	| parse_dostmt => { |s| Statement::Do(s) }
	| parse_callstmt => { |s| Statement::Call(s) }
	| chain!(tag!("read") ~ multispace ~ name: ident, || Statement::Read(name))
	| chain!(tag!("write") ~ multispace ~ name: ident, || Statement::Write(name))
	| parse_modstmt => { |s| Statement::Mod(s) }
));

named!(parse_ifstmt<Ifstmt>, chain!(
	tag!("if") ~ multispace ~ _if: parse_expression ~ multispace ~
	then:  chain!(
		tag!("then") ~ multispace ~
		b: many0!(parse_statement),
		
		|| b
	)? ~
	_else: chain!(
		tag!("else") ~ multispace ~
		b: many0!(parse_statement),
		
		|| b
	)? ~
	tag!("fi") ~ multispace ~ fi: parse_expression ~ multispace,
	
	|| Ifstmt {
		_if: _if,
		then: then.unwrap_or(Vec::new()),
		_else: _else.unwrap_or(Vec::new()),
		fi: fi,
	}
));

named!(parse_dostmt<Dostmt>, chain!(
	tag!("from") ~ multispace ~ from: parse_expression ~ multispace ~
	_do:  chain!(
		tag!("do") ~ multispace ~
		b: many0!(parse_statement),
		
		|| b
	)? ~
	_loop: chain!(
		tag!("loop") ~ multispace ~
		b: many0!(parse_statement),
		
		|| b
	)? ~
	tag!("until") ~ multispace ~ until: parse_expression ~ multispace,
	
	|| Dostmt {
		from: from,
		_do: _do.unwrap_or(Vec::new()),
		_loop: _loop.unwrap_or(Vec::new()),
		until: until,
	}
));

named!(parse_callstmt<Callstmt>, alt!(
	chain!(tag!("call") ~ multispace ~ f: ident, || Callstmt::Call(f))
	| chain!(tag!("uncall") ~ multispace ~ f: ident, || Callstmt::Uncall(f))
));

named!(parse_modstmt<Modstmt>, alt!(
	chain!(l: parse_lvalue ~ multispace? ~ tag!(":") ~ multispace? ~ r: parse_lvalue, || Modstmt::Swap(l, r))
));

named!(parse_expression<Expression>, chain!(
	m: parse_minexp ~
	b: many0!(chain!(
		multispace? ~
		op: binop ~ multispace? ~
		e: parse_minexp,
		
		|| (op, e)
	)),
	
	|| Expression {min: m, more: b}
));

named!(parse_minexp<Minexp>, alt!(
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
	| chain!(
		tag!("~") ~ multispace ~ e: parse_expression,
		|| Minexp::Not(Box::new(e))
	)
	| parse_lvalue => { |l| Minexp::Lval(l) }
	| num => { |n| Minexp::Constant(n) }
));

named!(parse_lvalue<Lvalue>, chain!(
	name: ident ~ multispace ~
	off: delimited!(
		char!('['),
		chain!(multispace? ~ e: parse_expression ~ multispace?, || e),
		char!(']')
	),
	
	|| Lvalue {name: name, off: Box::new(off)}
));
