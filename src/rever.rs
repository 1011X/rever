/*
use std::str;
use nom::{
	multispace, space,
	alpha, digit,
};

#[derive(Debug)]
pub struct Pattern<'a>(bool, &'a str);

#[derive(Debug, Clone, Copy)]
pub enum Type { U16, I16, Usize, Isize }

#[derive(Debug)]
pub struct Procedure<'a> {
	name: &'a str,
	args: Vec<(Pattern<'a>, Type)>,
	code: Vec<Statement<'a>>,
}

#[derive(Debug)]
pub enum Item<'a> {
	//Static(bool, &'a str, Type, Expression<'a>),
	//Mod(Vec<Item>),
	Proc(Procedure<'a>)
}

#[derive(Debug)]
pub enum Expression<'a> {
	Variable(&'a str),
	Constant(&'a str, Option<Type>),
}
/*
#[derive(Debug)]
enum Test { Parity, Sign }
*/

type Var<'a> = (String, Option<Expression<'a>>);

#[derive(Debug)]
pub enum Statement<'a> {
	//Not(&'a str),
	//Neg(&'a str),
	Call(&'a str, Vec<Expression<'a>>),
	Uncall(&'a str, Vec<Expression<'a>>),
	RotLeft(&'a str, Expression<'a>),
	RotRight(&'a str, Expression<'a>),
	Swap(&'a str, &'a str),
	Xor(&'a str, Expression<'a>),
	Add(&'a str, Expression<'a>),
	Sub(&'a str, Expression<'a>),
	CCNot(&'a str, Expression<'a>, Expression<'a>),
	CSwap(Expression<'a>, &'a str, &'a str),
	Let(Pattern<'a>, Option<Type>, Expression<'a>),
	Delete(Pattern<'a>, Option<Type>, Expression<'a>),
	//If(Test, &'a str, Vec<Statement<'a>>),
	//Switch(&'a str, Vec<&'a str, Vec<Statement<'a>>>),
	//From(Test, &'a str, Vec<Statement<'a>>),
	//Unsafe(Vec<Statement<'a>>),
}

impl<'a> Statement<'a> {
	pub fn invert(self) -> Self {
		match self {
			//Statement::Not(..)         => self,
			//Statement::Neg(..)         => self,
			Statement::Call(a, b)      => Statement::Uncall(a, b),
			Statement::Uncall(a, b)    => Statement::Call(a, b),
			Statement::RotLeft(a, b)   => Statement::RotRight(a, b),
			Statement::RotRight(a, b)  => Statement::RotLeft(a, b),
			Statement::Swap(..)        => self,
			Statement::Xor(..)         => self,
			Statement::Add(a, b)       => Statement::Sub(a, b),
			Statement::Sub(a, b)       => Statement::Add(a, b),
			Statement::CSwap(..)       => self,
			Statement::Let(a, b, c)    => Statement::Delete(a, b, c),
			Statement::Delete(a, b, c) => Statement::Let(a, b, c),
			// ...
			Statement::Unsafe(..)      => self,
		}
	}
}

named!(identifier<&str>, map_res!(alpha, str::from_utf8));
named!(constant<&str>, map_res!(digit, str::from_utf8));
/*
named!(whitespace, alt!(
	multispace |
	do_parse!(tag!("/*") >> c: many0!(anychar) >> tag!("*/"), || c) |
	do_parse!(tag!("//") >> c: many0!(anychar) >> tag!("\n"), || c) |
	whitespace
));
*/
named!(expression<Expression>, alt!(
	map!(identifier, Expression::Variable)
	| do_parse!(
		c: constant >>
		t: opt!(preceded!(tag!("_"), parse_type)) >>
		(Expression::Constant(c, t))
	)
));

named!(parse_type<Type>, alt!(
	map!(tag!("u16"),     |_| Type::U16)
	| map!(tag!("i16"),   |_| Type::I16)
	| map!(tag!("usize"), |_| Type::Usize)
	| map!(tag!("isize"), |_| Type::Isize)
));

named!(parse_pattern<Pattern>, ws!(do_parse!(
	m: opt!(tag!("mut")) >>
	var: identifier >>
	
	(Pattern(m.is_some(), var))
)));

named!(tuple<Vec<Expression> >, ws!(delimited!(
	tag!("("),
	ws!(separated_list!(
		tag!(","),
		expression
	)),
	tag!(")")
)));

named!(parse_block<Vec<Option<Statement>>>, ws!(delimited!(
	tag!("{"),
	ws!(many0!(ws!(do_parse!(
		stmt: opt!(parse_statement) >>
		alt!(tag!(",") | tag!("\n") | tag!("\r\n")) >>
		(stmt)
	)))),
	tag!("}")
)));

named!(parse_statement<Statement>, alt!(
	ws!(do_parse!(
		tag!("!") >>
		var: identifier >>
		(Statement::Not(var))
	))
	| ws!(do_parse!(
		tag!("-") >>
		var: identifier >>
		(Statement::Neg(var))
	))
	| ws!(do_parse!(
		l: identifier >>
		m: alt!(
			tag!("+=") | tag!("-=") |
			tag!("^=") |
			tag!("<<=") | tag!(">>=")
		) >>
		r: expression >>
		
		(match m {
			b"+="  => Statement::Add(l, r),
			b"-="  => Statement::Sub(l, r),
			b"^="  => Statement::Xor(l, r),
			b"<<=" => Statement::LRot(l, r),
			b">>=" => Statement::RRot(l, r),
			_ => unreachable!()
		})
	))
	| ws!(do_parse!(
		left: identifier >>
		tag!("<>") >>
		right: identifier >>
		(Statement::Swap(left, right))
	))
	| ws!(do_parse!(
		dest: identifier >>
		tag!("^=") >>
		lctrl: expression >>
		tag!("&") >>
		rctrl: expression >>
		
		(Statement::CCNot(dest, lctrl, rctrl))
	))
	| ws!(do_parse!(
		control: expression >>
		tag!("?") >>
		left: identifier >>
		tag!("<>") >>
		right: identifier >>
		
		(Statement::CSwap(control, left, right))
	))
	| ws!(do_parse!(
		tag!("unsafe") >>
		b: parse_block >>
		
		(Statement::Unsafe(b.into_iter()
			.filter_map(|s| s)
			.collect()
		))
	))
	| ws!(do_parse!(
		name: identifier >>
		args: tuple >>
		(Statement::Call(name, args))
	)) |
	do_parse!(
		tag!("rev") >> space >> name: identifier >> space? >> args: tuple,
		|| Statement::Uncall(name, args)
	)
	| ws!(do_parse!(
		tag!("let") >>
		name: parse_pattern >>
		ty: opt!(ws!(do_parse!(
			tag!(":") >>
			ty: parse_type >>
			
			(ty)
		))) >>
		tag!("=") >>
		e: expression,
		
		(Statement::Let(name, ty, e))
	))
	| ws!(do_parse!(
		tag!("delete") >>
		name: parse_pattern >>
		ty: opt!(ws!(do_parse!(
			tag!(":") >>
			ty: parse_type >>
			
			(ty)
		)) >>
		tag!("=") >>
		e: expression >>
		
		(Statement::Delete(name, ty, e))
	))
));

named!(parse_procedure<Procedure>, ws!(do_parse!(
	tag!("proc") >>
	name: identifier >>
	args: ws!(delimited!(
		tag!("("),
		ws!(separated_list!(
			tag!(","),
			ws!(do_parse!(
				pat: parse_pattern >>
				tag!(":") >>
				ty: parse_type >>
				
				(pat, ty)
			))
		)),
		tag!(")")
	)) >>
	code: parse_block >>
	
	// some extra work can be done here if it's really needed
	(Procedure {
		name: name,
		args: args,
		code: code.into_iter()
			.filter_map(|s| s)
			.collect(),
	})
)));

named!(parse_item<Item>, alt!(
	ws!(do_parse!(p: parse_procedure >> (Item::Proc(p))))
));

named!(pub parse_program<Vec<Item> >, complete!(ws!(
	many0!(parse_item)
)));
/*
fn main() {
	println!("{:?}", parse_statement(b"a <> b"));
	println!("{:?}", parse_statement(b"a   <>    b"));
	println!("{:?}", parse_statement(b"a<>b"));
	
	println!("{:?}", parse_statement(b"a--"));
	println!("{:?}", parse_statement(b"a  --"));
	
	println!("{:?}", parse_statement(b"a++"));
	println!("{:?}", parse_statement(b"a  ++"));
	
	println!("{:?}", parse_statement(b"-a"));
	println!("{:?}", parse_statement(b"- a"));
	
	println!("{:?}", parse_statement(b"!a"));
	println!("{:?}", parse_statement(b"! a"));
	
	println!("{:?}", parse_block(b"{
		a <> b
		a --
		b++
		!a
		-b
	}"));
	println!("{:?}", parse_block(b"{a<>b;a--;b++;!a;-b;}"));
	
	println!("{:?}", parse_procedure(b"proc main() {}"));
	println!("{:?}", parse_procedure(b"proc f(a: u16) {}"));
	println!("{:?}", parse_procedure(b"proc f(mut a: u16) {}"));
	println!("{:?}", parse_procedure(b"proc f(mut a: usize){}"));
	println!("{:?}", parse_procedure(b"proc s(mut a: u16, mut b: u16) {
		a <> b
	}"));
	
	println!("{:?}", parse_procedure(b"proc swap(mut a: u16, mut b: u16) {
		a <> b
		a <<= 3
		b >>= a
	}"));
	
	println!("{:?}", parse_procedure(b"proc t(mut a: u16, b: u16) {
		a ^= b
		
		a <<= 3_usize
	}"));
	
	println!("{:?}", parse_program(b"
	proc hello(mut a: u16, mut b: u16) {
		3 ? a <> b
		a ^= 4 & b
		
		a += b
		b -= a
		
		unsafe {
			a ^= b
			
			hello(a, b)
			rev hello(a, b)
		}
	}
	
	proc main() {
		let a = 3
		let mut b=5
		let mut c = 6
		hello(b, c)
		rev hello(b, c)
		delete mut c = 6
		delete mut b=5
		delete a = 3
	}"));
}
*/
*/

named!(id, re_bytes_find!("^[A-Za-z_][A-Za-z0-9_]*"));
named!(mod_op, alt!(
	tag!("+=")
	| tag!("-=")
	| tag!("^=")
	| tag!("<<=")
	| tag!(">>=")
));

enum Statement {
	Let(bool, String, Option<Type>, Expr),
	Reset(String, Expr),
	Swap(String, String),
	Add(String, Option<Expr>, Expr),
	Sub(String, Option<Expr>, Expr),
	Xor(String, Option<Expr>, Expr),
	LRot(String, Option<Expr>, Expr),
	RRot(String, Option<Expr>, Expr),
	If(Expr, Block, Option<Block>, Expr),
	From(Expr, Option<Block>, Option<Block>, Expr),
	Call(String, Vec<Expr>),
	Uncall(String, Vec<Expr>),
}
named!(stmt<Statement>, alt!(
	ws!(do_parse!(
		tag!("let") >>
		m: opt!(tag!("mut")) >>
		n: id >>
		t: opt!(ws!(preceded!(tag!(":"), typ))) >>
		tag!("=") >>
		e: expr >>
		
		(Statement::Let(m.is_some(), n, t, e))
	))
	| ws!(do_parse!(
		tag!("reset") >>
		n: id >>
		tag!("=") >>
		e: expr >>
		
		(Statement::Reset(n, e))
	))
	| ws!(map!(
		separated_pair!(id, tag!("<>"), id),
		|(l, r)| Statement::Swap(l, r)
	))
	| ws!(do_parse!(
		v: id >>
		s: opt!(ws!(delimited!(
			tag!("["),
			expr,
			tag!("]")
		))) >>
		o: alt!(
			tag!("+=")
			| tag!("-=")
			| tag!("^=")
			| tag!("<<=")
			| tag!(">>=")
		) >>
		e: expr >>
		
		(match o {
			b"+=" => Statement::Add(v, s, e),
			b"-=" => Statement::Sub(v, s, e),
			b"^=" => Statement::Xor(v, s, e),
			b"<<=" => Statement::LRot(v, s, e),
			b">>=" => Statement::RRot(v, s, e),
			_ => unreachable!()
		})
	))
	| ws!(do_parse!(
		tag!("if") >>
		p: expr >>
		t: block >>
		e: opt!(ws!(preceded!(tag!("else"),  block))) >>
		tag!("assert") >>
		a: expr >>
		
		(Statement::If(p, t, e, a))
	))
	| ws!(do_parse!(
		tag!("from") >>
		a: expr >>
		d: opt!(block) >>
		l: opt!(block) >>
		tag!("until") >>
		p: expr >>
		
		(Statement::From(a, d, l, p))
	))
	| ws!(do_parse!(
		c: alt!(tag!("call") | tag!("uncall")) >>
		//r: opt!(tag!("rev")) >>
		n: id >>
		a: ws!(delimited!(
			tag!("("),
			ws!(separated_list!(tag!(","), expr)),
			tag!(")")
		)) >>
		
		(match c {
			b"call" => Statement::Call(n, a),
			b"uncall" => Statement::Uncall(n, a),
			_ => unreachable!()
		})
	))
))

named!(block<Block>, ws!(delimited!(
	tag!("{"),
	ws!(many0!(
		ws!(terminated!(stmt, tag!(";")))
	)),
	tag!("}")
)));

re_bytes_find!("^[-+]?[1-9][0-9]*")
re_bytes_find!("^0x[A-Fa-f0-9_]+")
re_bytes_find!("^0b[01_]+")



<num-type> ::= 'u16'
	| 'i16'
	| 'usize'
	| 'isize'

<literal> ::= <num>
	| '[' <value> ']'

<typelist> ::= <type> | <type> ',' <typelist>
<type> ::= <id> ['<' <typelist> '>']
	| 'bool'
	| 'u16'
	| 'i16'
	| 'usize'
	| 'isize'
	| '[' <type> [';' <num>] ']'
	| '^' <type>
	| <proc-type>

<proc-type> ::= 'proc' '(' <proc-param-type-list> ')'
<proc-param-type-list> ::= [<proc-param-type-list'>]
<proc-param-type-list'> ::= ['mut'] <type> | ['mut'] <type> ',' <proc-param-type-list'>

<param> ::= ['mut'] <id> ':' <type>

<paramlist> ::= [<paramlist'>]
<paramlist'> ::= <param>
	| <param> ',' <paramlist'>

<value> ::= <num> | <id>

<arglist> ::= [<arglist'>]
<arglist'> ::= <value>
	| <value> ',' <arglist'>

<const-bin-op> ::= '*' | '/' | '%' | <bin-op>
<bin-op> ::= '+' | '-' | '^' | '&' | '|' | '<' | '>' | '<=' | '>=' | '=' | '!=' | '<<' | '>>'
<mod-op> ::= '+=' | '-=' | '^=' | '<<=' | '>>='

<stmt> ::= 'let' ['mut'] <id> [':' <type>] '=' <expr>
	| 'reset' <id> '=' <expr>
	| 'const' <id> ':' <type> '=' <const-expr>
	| <id> ['[' <expr> ']'] <mod-op> <expr>
	| <id> '<>' <id>
	| <if-stmt>
	| <from-stmt>
	| 'call' <id> '(' <arglist> ')'
	| 'uncall' <id> '(' <arglist> ')'

<stmtlist> ::= [<stmt> ';' <stmtlist>]

<if-stmt> ::=
	'if' <expr> '{' <stmtlist> '}'
	['else' '{' <stmtlist> '}']
	'assert' <expr>

<from-stmt> ::=
	'from' <expr> ['{' <stmtlist> '}']
	'until' <expr> ['{' <stmtlist> '}']

<proc> ::= 'proc' <id> '(' <paramlist> ')' '{' <stmtlist> '}'
