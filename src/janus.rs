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
named!(num<i16>, reb_parse!("^[-+]?[0-9]+"));
named!(boolean<bool>, reb_parse!("^(true|false)"));
named!(st<String>, delimited!(
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
	IntArray(Expr)
}

#[derive(Debug)]
pub struct Decl {
	name: String,
	_type: Type
}

impl Decl {
	named!(parse<Decl>, alt_complete!(
		ws!(do_parse!(
			tag!("int") >>
			name: ident
			>> (Decl {name, _type: Type::Int})
		))
		| ws!(do_parse!(
			tag!("stack") >>
			name: ident
			>> (Decl {name, _type: Type::Stack})
		))
		| ws!(do_parse!(
			tag!("int") >>
			name: ident >>
			len: delimited!(tag!("["), call!(Expr::parse), tag!("]"))
			>> (Decl {name, _type: Type::IntArray(len)})
		))
	));
}

#[derive(Debug)]
pub enum Factor {
	LValue(LValue),
	Literal(Literal)
}

impl Factor {
	named!(parse<Factor>, alt_complete!(
		map!(LValue::parse, Factor::LValue)
		| map!(Literal::parse, Factor::Literal)
	));
}

#[derive(Debug)]
pub struct LValue {
	name: String,
	indices: Vec<Expr>,
}

impl LValue {
	named!(parse<LValue>, ws!(do_parse!(
		name: ident >>
		indices: many0!(delimited!(
			tag!("["),
			call!(Expr::parse),
			tag!("]")
		))
		>> (LValue {name, indices})
	)));
}

#[derive(Debug)]
pub enum Literal {
	Int(i16),
	Bool(bool),
	IntArray(Vec<i16>)
}

impl Literal {
	named!(parse<Literal>, alt_complete!(
		map!(num, Literal::Int)
		| map!(boolean, Literal::Bool)
		| ws!(map!(
			delimited!(
				tag!("{"),
				separated_list!(tag!(","), num),
				tag!("}")
			),
			Literal::IntArray
		))
	));
}

#[derive(Debug)]
pub enum Expr {
	Factor(Factor),
	
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
	named!(parse<Expr>, alt_complete!(
		call!(Expr::bitop)
		| call!(Expr::sum)
		| call!(Expr::product)
		| call!(Expr::leaf)
	));
	
	named!(leaf<Expr>, alt_complete!(
		map!(Factor::parse, Expr::Factor)
		| ws!(delimited!(
			tag!("("),
			call!(Expr::parse),
			tag!(")")
		))
	));
	
	named!(product<Expr>, alt_complete!(
		ws!(do_parse!(
			left: call!(Expr::leaf) >>
			tag!("*") >>
			right: alt_complete!(
				call!(Expr::product)
				| call!(Expr::leaf)
			)
			>> (Expr::Mul(Box::new(left), Box::new(right)))
		))
		| ws!(do_parse!(
			left: call!(Expr::leaf) >>
			tag!("/") >>
			right: alt_complete!(
				call!(Expr::product)
				| call!(Expr::leaf)
			)
			>> (Expr::Div(Box::new(left), Box::new(right)))
		))
		| ws!(do_parse!(
			left: call!(Expr::leaf) >>
			tag!("%") >>
			right: alt_complete!(
				call!(Expr::product)
				| call!(Expr::leaf)
			)
			>> (Expr::Mod(Box::new(left), Box::new(right)))
		))
	));
	
	named!(sum<Expr>, alt_complete!(
		ws!(do_parse!(
			left: alt_complete!(
				call!(Expr::product)
				| call!(Expr::leaf)
			) >>
			tag!("+") >>
			right: alt_complete!(
				call!(Expr::product)
				| call!(Expr::leaf)
			)
			>> (Expr::Add(Box::new(left), Box::new(right)))
		))
		| ws!(do_parse!(
			left: alt_complete!(
				call!(Expr::product)
				| call!(Expr::leaf)
			) >>
			tag!("-") >>
			right: alt_complete!(
				call!(Expr::product)
				| call!(Expr::leaf)
			)
			>> (Expr::Sub(Box::new(left), Box::new(right)))
		))
	));
	
	named!(bitop<Expr>, alt_complete!(
		ws!(do_parse!(
			left: alt_complete!(
				call!(Expr::sum)
				| call!(Expr::product)
				| call!(Expr::leaf)
			) >>
			tag!("&") >>
			right: alt_complete!(
				call!(Expr::sum)
				| call!(Expr::product)
				| call!(Expr::leaf)
			)
			>> (Expr::BitAnd(Box::new(left), Box::new(right)))
		))
		| ws!(do_parse!(
			left: alt_complete!(
				call!(Expr::sum)
				| call!(Expr::product)
				| call!(Expr::leaf)
			) >>
			tag!("|") >>
			right: alt_complete!(
				call!(Expr::sum)
				| call!(Expr::product)
				| call!(Expr::leaf)
			)
			>> (Expr::BitOr(Box::new(left), Box::new(right)))
		))
		| ws!(do_parse!(
			left: alt_complete!(
				call!(Expr::sum)
				| call!(Expr::product)
				| call!(Expr::leaf)
			) >>
			tag!("|") >>
			right: alt_complete!(
				call!(Expr::sum)
				| call!(Expr::product)
				| call!(Expr::leaf)
			)
			>> (Expr::BitXor(Box::new(left), Box::new(right)))
		))
	));
}

#[derive(Debug)]
pub enum Pred {
	LogAnd(Box<Pred>, Box<Pred>),
	
	LogOr(Box<Pred>, Box<Pred>),
	
	Not(Box<Pred>),
	
	Eq(Expr, Expr),
	Neq(Expr, Expr),
	Gt(Expr, Expr),
	Lt(Expr, Expr),
	Gte(Expr, Expr),
	Lte(Expr, Expr),
}

impl Pred {
	named!(parse<Pred>, alt_complete!(
		call!(Pred::or)
		| call!(Pred::and)
		| call!(Pred::not)
		| call!(Pred::cmp)
	));
	
	named!(or<Pred>, ws!(do_parse!(
		left: alt_complete!(call!(Pred::and) | call!(Pred::not) | call!(Pred::cmp)) >>
		tag!("||") >>
		right: alt_complete!(call!(Pred::and) | call!(Pred::not) | call!(Pred::cmp))
		>> (Pred::LogOr(Box::new(left), Box::new(right)))
	)));
	
	named!(and<Pred>, ws!(do_parse!(
		left: alt_complete!(call!(Pred::not) | call!(Pred::cmp)) >>
		tag!("&&") >>
		right: alt_complete!(call!(Pred::not) | call!(Pred::cmp))
		>> (Pred::LogAnd(Box::new(left), Box::new(right)))
	)));
	
	named!(not<Pred>, alt_complete!(
		ws!(do_parse!(
			tag!("!") >>
			leaf: call!(Pred::not)
			>> (Pred::Not(Box::new(leaf)))
		))
		| ws!(delimited!(
			tag!("("),
			call!(Pred::parse),
			tag!(")")
		))
	));
	
	named!(cmp<Pred>, alt_complete!(
		ws!(do_parse!(
			left: call!(Expr::parse) >>
			tag!("=") >>
			right: call!(Expr::parse)
			>> (Pred::Eq(left, right))
		))
		| ws!(do_parse!(
			left: call!(Expr::parse) >>
			tag!("!=") >>
			right: call!(Expr::parse)
			>> (Pred::Neq(left, right))
		))
		| ws!(do_parse!(
			left: call!(Expr::parse) >>
			tag!(">") >>
			right: call!(Expr::parse)
			>> (Pred::Lt(left, right))
		))
		| ws!(do_parse!(
			left: call!(Expr::parse) >>
			tag!("<") >>
			right: call!(Expr::parse)
			>> (Pred::Gt(left, right))
		))
		| ws!(do_parse!(
			left: call!(Expr::parse) >>
			tag!(">=") >>
			right: call!(Expr::parse)
			>> (Pred::Lte(left, right))
		))
		| ws!(do_parse!(
			left: call!(Expr::parse) >>
			tag!("<=") >>
			right: call!(Expr::parse)
			>> (Pred::Gte(left, right))
		))
	));
}

type Block = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
	Skip,
	Local(Decl, Expr),
	Delocal(Decl, Expr),
	Add(LValue, Expr),
	Sub(LValue, Expr),
	Xor(LValue, Expr),
	Swap(LValue, LValue),
	If(Pred, Block, Option<Block>, Pred),
	From(Pred, Option<Block>, Option<Block>, Pred),
	Call(String, Vec<Factor>),
	Uncall(String, Vec<Factor>),
	
	// built-ins
	Print(String),
	Printf(String, Vec<Factor>),
	Error(String),
}

impl Statement {
	named!(parse<Statement>, alt_complete!(
		value!(Statement::Skip, tag!("skip"))
		| ws!(do_parse!(
			tag!("local") >>
			decl: call!(Decl::parse) >>
			tag!("=") >>
			val: call!(Expr::parse)
			>> (Statement::Local(decl, val))
		))
		| ws!(do_parse!(
			tag!("delocal") >>
			decl: call!(Decl::parse) >>
			tag!("=") >>
			val: call!(Expr::parse)
			>> (Statement::Delocal(decl, val))
		))
		| ws!(do_parse!(
			left: call!(LValue::parse) >>
			tag!("<=>") >>
			right: call!(LValue::parse)
			>> (Statement::Swap(left, right))
		))
		| ws!(do_parse!(
			left: call!(LValue::parse) >>
			tag!("+=") >>
			expr: call!(Expr::parse)
			>> (Statement::Add(left, expr))
		))
		| ws!(do_parse!(
			left: call!(LValue::parse) >>
			tag!("-=") >>
			expr: call!(Expr::parse)
			>> (Statement::Sub(left, expr))
		))
		| ws!(do_parse!(
			left: call!(LValue::parse) >>
			tag!("^=") >>
			expr: call!(Expr::parse)
			>> (Statement::Xor(left, expr))
		))
		| ws!(do_parse!(
			tag!("from") >>
			assert: call!(Pred::parse) >>
			forward: opt!(ws!(preceded!(
				tag!("do"),
				many1!(Statement::parse)
			))) >>
			backward: opt!(ws!(preceded!(
				tag!("loop"),
				many1!(Statement::parse)
			))) >>
			tag!("until") >>
			pred: call!(Pred::parse)
			
			>> (Statement::From(assert, forward, backward, pred))
		))
		| ws!(do_parse!(
			tag!("if") >>
			pred: call!(Pred::parse) >>
			pass: ws!(preceded!(
				tag!("then"),
				many1!(Statement::parse)
			)) >>
			fail: opt!(ws!(preceded!(
				tag!("else"),
				many1!(Statement::parse)
			))) >>
			tag!("fi") >>
			assert: call!(Pred::parse)
			
			>> (Statement::If(pred, pass, fail, assert))
		))
		| ws!(do_parse!(
			tag!("call") >>
			func: ident >>
			args: delimited!(
				tag!("("),
				separated_list!(tag!(","), Factor::parse),
				tag!(")")
			)
			>> (Statement::Call(func, args))
		))
		| ws!(do_parse!(
			tag!("uncall") >>
			func: ident >>
			tag!("(") >>
			args: separated_list!(tag!(","), Factor::parse) >>
			tag!(")")
			>> (Statement::Uncall(func, args))
		))
		// built-ins
		| ws!(do_parse!(
			tag!("print") >>
			tag!("(") >>
			string: st >>
			tag!(")")
			>> (Statement::Print(string))
		))
		| ws!(do_parse!(
			tag!("printf") >>
			tag!("(") >>
			string: st >>
			vargs: many0!(ws!(preceded!(
				tag!(","),
				Factor::parse
			))) >>
			tag!(")")
			>> (Statement::Printf(string, vargs))
		))
		| ws!(do_parse!(
			tag!("error") >>
			tag!("(") >>
			string: st >>
			tag!(")")
			>> (Statement::Error(string))
		))
	));
}

#[derive(Debug)]
struct Procedure {
	name: String,
	args: Vec<Decl>,
	body: Vec<Statement>
}

impl Procedure {
	named!(parse<Procedure>, ws!(do_parse!(
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
pub struct Program {
	funcs: Vec<Procedure>
}

impl Program {
	named!(pub parse<Program>, do_parse!(
		funcs: many1!(Procedure::parse)
		>> (Program {funcs})
	));
}