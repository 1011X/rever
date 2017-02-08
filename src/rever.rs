use std::str;

named!(ident<String>, re_bytes_find!("^[A-Za-z_][A-Za-z0-9_]*"));
named!(constant<Num>, map_res!(digit, str::from_utf8));

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
		tag!("[") >>
		t: type_ >>
		tag!(";") >>
		n: num >>
		tag!("]") >>
		
		(Type::Array(t, n))
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
	| ws!(preceded!(tag!("type"), ident))
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
				
				(m, name, t)
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
enum Factor {
	Lit(Literal),
	LVal(LValue),
}

named!(factor<Factor>, alt!(
	map!(lit, Factor::Lit)
	| map!(lval, Factor::Lval)
));

#[derive(Debug)]
enum Literal {
	Num(Number),
}

named!(lit<Literal>, map!(num, Literal::Num));

#[derive(Debug)]
enum Number {
	Unknown(u16),
	U16(u16),
	I16(i16),
	Usize(u16),
	Isize(i16),
}

named!(num<Number>, alt!(
	re_bytes_find!("^[-+]?[1-9][0-9]*")
	| re_bytes_find!("^0x[A-Fa-f0-9_]+")
	| re_bytes_find!("^0b[01_]+")
));

#[derive(Debug)]
enum Deref {
	Direct,
	Indexed(Factor),
	Field(String),
}

#[derive(Debug)]
struct LValue {
	id: String,
	ops: Vec<Deref>,
}

named!(lval<LValue>, ws!(do_parse!(
	ident: id >>
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
		id: ident,
		ops: ops,
	})
)));

#[derive(Debug)]
pub enum Statement {
	Let(String, Option<Type>, Lit),
	Var(String, Option<Type>, Lit),
	Drop(String, Lit),
	
	Not(LValue),
	Neg(LValue),
	
	RotLeft(LValue, Factor),
	RotRight(LValue, Factor),
	
	CCNot(LValue, Factor, Factor),
	Xor(LValue, Vec<Factor>),
	
	Add(LValue, Vec<(bool, Factor)>),
	Sub(LValue, Vec<(bool, Factor)>),
	
	Swap(LValue, LValue),
	CSwap(Factor, LValue, LValue),
	
	If(BinExpr, Vec<Statement>, Option<Vec<Statement>>, BinExpr),
	
	From(BinExpr, Option<Vec<Statement>>, Option<Vec<Statement>>, BinExpr),
	
	Call(LValue, Vec<Factor>),
	Uncall(LValue, Vec<Factor>),
	
	//Switch(String, Vec<String, Vec<Statement>>),
	//Unsafe(Vec<Statement>),
}
/*
impl Statement {
	pub fn invert(self) -> Self {
		match self {
			Statement::Let(..)        => self,
			Statement::Var(a, b, c)   => Statement::Drop(a, b, c),
			Statement::Drop(a, b, c)  => Statement::Var(a, b, c),
			
			//Statement::Not(..)        => self,
			//Statement::Neg(..)        => self,
			
			Statement::RotLeft(a, b)  => Statement::RotRight(a, b),
			Statement::RotRight(a, b) => Statement::RotLeft(a, b),
			
			Statement::CCNot(..)      => self,
			Statement::Xor(..)        => self,
			
			Statement::Add(a, b)      => Statement::Sub(a, b),
			Statement::Sub(a, b)      => Statement::Add(a, b),
			
			Statement::Swap(..)       => self,
			Statement::CSwap(..)      => self,
			
			Statement::Call(a, b)     => Statement::Uncall(a, b),
			Statement::Uncall(a, b)   => Statement::Call(a, b),
			
			// ...
			Statement::Unsafe(..)     => self,
		}
	}
}
*/

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
		control: expression >>
		tag!("?") >>
		left: lval >>
		tag!("<>") >>
		right: lval >>
		
		(Statement::CSwap(control, left, right))
	))
	| ws!(do_parse!(
		l: lval >>
		m: alt!(
			tag!("+=") | tag!("-=") |
			tag!("^=") |
			tag!("<<=") | tag!(">>=")
		) >>
		r: factor >>
		
		(match m {
			b"+="  => Statement::Add(l, r),
			b"-="  => Statement::Sub(l, r),
			b"^="  => Statement::Xor(l, r),
			b"<<=" => Statement::LRot(l, r),
			b">>=" => Statement::RRot(l, r),
			_ => unreachable!()
		})
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
		p: expr >>
		t: block >>
		e: opt!(ws!(preceded!(tag!("else"), block))) >>
		tag!("fi") >>
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
));

named!(pub program<Vec<Item> >, complete!(
	ws!(many0!(parse_item))
));
