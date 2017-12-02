use std::str;
use std::collections::HashMap;
use rel;

type SymbolTable = HashMap<String, Location>;

/// Tracks location of a symbol or value.
#[derive(Debug, Clone, Copy)]
enum Location {
	/// In a CPU register
	Reg(rel::Reg),
	/// In memory at the specified offset in the stack. Arguments have non-
	/// positive values, variables have positive values.
	Memory(isize),
}

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
named!(num<u16>, reb_parse!("^[-+]?[0-9]+"));
named!(ch<char>, delimited!(
    tag!("'"),
    alt!(
        value!('\\', tag!(r"\\"))
        | value!('\'', tag!(r"\'"))
        | value!('\n', tag!(r"\n"))
        | value!('\t', tag!(r"\t"))
        | call!(::nom::anychar)
    ),
    tag!("'")
));
named!(st<String>, delimited!(
    tag!("\""),
    map_res!(
        escaped_transform!(is_not!("\\\""), '\\', alt!(
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

impl BinExpr {
	named!(parse<BinExpr>, alt!(
		do_parse!( // a = b
			l: call!(Factor::parse) >> tag!("=") >> r: call!(Factor::parse)
			>> (BinExpr::Eq(l, r))
		)
		| do_parse!( // a != b, a ≠ b
			l: call!(Factor::parse) >> alt!(tag!("!=") | tag!("≠")) >> r: call!(Factor::parse)
			>> (BinExpr::Neq(l, r))
		)
		| do_parse!( // a < b
			l: call!(Factor::parse) >> tag!("<") >> r: call!(Factor::parse)
			>> (BinExpr::Lt(l, r))
		)
		// Should I really have `<=` and `>=`? They look so much like arrows.
		| do_parse!( // a <= b, a ≤ b
			l: call!(Factor::parse) >> alt!(tag!("<=") | tag!("≤")) >> r: call!(Factor::parse)
			>> (BinExpr::Lte(l, r))
		)
		| do_parse!( // a > b
			l: call!(Factor::parse) >> tag!(">") >> r: call!(Factor::parse)
			>> (BinExpr::Gt(l, r))
		)
		| do_parse!( // a >= b, a ≥ b
			l: call!(Factor::parse) >> alt!(tag!(">=") | tag!("≥")) >> r: call!(Factor::parse)
			>> (BinExpr::Gte(l, r))
		)
		| do_parse!( // a & b, a and b
			l: call!(Factor::parse) >> alt!(tag!("&") | tag!("and")) >> r: call!(Factor::parse)
			>> (BinExpr::And(l, r))
		)
		| do_parse!( // a | b, a or b
			l: call!(Factor::parse) >> alt!(tag!("|") | tag!("or")) >> r: call!(Factor::parse)
			>> (BinExpr::Or(l, r))
		)
		| do_parse!( // a ^ b, a xor b
			l: call!(Factor::parse) >> alt!(tag!("^") | tag!("xor")) >> r: call!(Factor::parse)
			>> (BinExpr::Xor(l, r))
		)
	));
}


#[derive(Debug, Clone)]
pub enum Type {
	Unit,
	Bool,
	U16, I16, Usize, Isize,
    Char,
	Pointer(Box<Type>),
	Array(Box<Type>, usize),
	Fn(Vec<Type>),
	Composite(String),
}

impl Type {
	named!(parse<Type>, alt!(
		value!(Type::Unit, tag!("unit"))
		| value!(Type::Bool, tag!("bool"))
		| value!(Type::U16, tag!("u16"))
		| value!(Type::I16, tag!("i16"))
		| value!(Type::Usize, tag!("usize"))
		| value!(Type::Isize, tag!("isize"))
		| value!(Type::Char, tag!("char"))
		| map!(ws!(preceded!(tag!("^"), Type::parse)), |t| Type::Pointer(Box::new(t)))
		| ws!(do_parse!(
			tag!("[") >> t: call!(Type::parse) >> tag!(";") >> n: num >> tag!("]")
			>> (Type::Array(Box::new(t), n as usize))
		))
		| map!(ws!(preceded!(
			tag!("fn"), 
			delimited!(
				tag!("("),
				separated_list!(tag!(","), Type::parse),
				tag!(")")
			)
		)), Type::Fn)
		| map!(ws!(preceded!(tag!("type"), ident)), Type::Composite)
	));
}


#[derive(Debug)]
pub struct Arg {
	name: String,
	mutable: bool,
	typ: Type,
}

impl Arg {
	named!(parse<Arg>, ws!(do_parse!(
		m: opt!(tag!("mut")) >>
		name: ident >>
		tag!(":") >>
		typ: call!(Type::parse)
		>> (Arg { name, mutable: m.is_some(), typ })
	)));
}


#[derive(Debug)]
pub struct Function {
	/// Name of the function.
	pub name: String,
	/// Arguments' setup within the function
	pub args: Vec<Arg>,
	/// Sequence of statements that make up the function.
	pub code: Vec<Statement>,
}

impl Function {
	named!(parse<Function>, ws!(do_parse!(
		tag!("fn") >> name: ident >>
		args: delimited!(
			tag!("("),
			separated_list!(tag!(","), Arg::parse),
			tag!(")")
		) >>
		code: block
		>> (Function { name, args, code })
	)));
	
	fn verify(&mut self) {
		for statement in &mut self.code {
			statement.verify();
		}
		/*
		let decls: Vec<&Statement> = self.code.iter()
			.filter(|&stmt| match *stmt {
				Statement::Let(true, ..) | Statement::Drop(..) => true,
				_ => false
			})
			.collect();
		
		decls.sort_by_key(|&stmt| match *stmt {
			Statement::Let(_, ref id, ..)
			| Statement::Drop(ref id, ..) => id,
			_ => unreachable!()
		});
		
		decls.dedup_by(|&s0, &s1| )
		
		for decl in decls.chunks(2)
		*/
	}
	
	fn compile(&self) -> Vec<rel::Op> {
		let mut body = vec![];
		// every symbol is associated with a location, and therefore a value
		let mut symbol_table = HashMap::new();
		
		// Add arguments to symbol table. Pascal convention is used.
		for (i, arg) in self.args.iter().rev().enumerate() {
			symbol_table.insert(arg.name.clone(), Location::Memory(-(i as isize)));
		}
		
		println!("Symbols: {:?}", symbol_table);
		
		// Compile body.
		for statement in &self.code {
			body.extend(statement.compile(&mut symbol_table));
		}
		
		println!("Code for {}: {:#?}", self.name, body);
		body
	}
}


#[derive(Debug)]
pub enum Item {
	//Static(bool, String, Type, ConstExpr),
	//Mod(Vec<Item>),
	Fn(Function),
}

impl Item {
	named!(parse<Item>, map!(Function::parse, Item::Fn));
}


#[derive(Debug)]
pub enum Factor {
	Lit(Literal),
	LVal(LValue),
}

impl Factor {
	named!(parse<Factor>, alt!(
		map!(Literal::parse, Factor::Lit)
		| map!(LValue::parse, Factor::LVal)
	));
	
	fn compile(&self, st: &mut SymbolTable) -> Vec<rel::Op> {
		use rel::Op;
		vec![Op::Nop]
	}
}


#[derive(Debug)]
pub enum Literal {
	Num(u16),
	Bool(bool),
	Char(char),
    Str(String),
}

impl Literal {
	named!(parse<Literal>, alt!(
		map!(num, Literal::Num)
		| value!(Literal::Bool(true), tag!("true"))
		| value!(Literal::Bool(false), tag!("false"))
		| map!(ch, Literal::Char)
		| map!(st, Literal::Str)
	));
}


#[derive(Debug)]
pub enum Deref {
	Direct,
	Indexed(Factor),
	Field(String),
}

#[derive(Debug)]
pub struct LValue {
	pub id: String,
	pub ops: Vec<Deref>,
}

impl LValue {
	named!(parse<LValue>, ws!(do_parse!(
		id: ident >>
		ops: many0!(alt!(
			value!(Deref::Direct, tag!("*"))
			| ws!(delimited!(
				tag!("["),
				map!(Factor::parse, Deref::Indexed),
				tag!("]")
			))
			| ws!(preceded!(tag!("."), map!(ident, Deref::Field)))
		))
		>> (LValue { id, ops })
	)));
	
	fn compile<F>(&self, st: &mut SymbolTable, f: F) -> Vec<rel::Op>
	where F: Fn(rel::Reg) -> Vec<rel::Op> {
		// need to know: symbol's value
		match st[&self.id] {
			Location::Reg(r) => f(r),
			
			Location::Memory(offset) => {
				use rel::{Reg, Op};
				
				// TODO: generalize to any register, not just r0
				let mut code = vec![];
				let loc = st.get_mut(&self.id).unwrap();
				
				// copy sp to r0
				*loc = Location::Reg(Reg::R0);
				code.push(Op::CNot(Reg::R0, Reg::SP));
				
				// store offset in immediate instruction(s)
				// if it's small enough, we only have to use 1 instruction.
				// perhaps some of this isn't necessary? maybe leave it for the
				// optimizer later on
				match offset {
					0 => {}
					1...255 => code.extend(vec![
						Op::Immediate(Reg::R1, offset as u8),
						Op::CAdd(Reg::R0, Reg::R1)
					]),
					_ => code.extend(vec![
						Op::Immediate(Reg::R1, (offset >> 8) as u8),
						Op::RotLeftImm(Reg::R1, 8),
						Op::Immediate(Reg::R1, offset as u8),
						Op::CAdd(Reg::R0, Reg::R1)
					])
				}
				
				code.push(Op::Exchange(Reg::R2, Reg::R0));
				code.extend(f(Reg::R2));
				// Assuming value is still at the same register...
				code.push(Op::Exchange(Reg::R2, Reg::R0));
				
				// store offset in immediate instruction(s)
				match offset {
					0 => {}
					1...255 => code.extend(vec![
						Op::CSub(Reg::R0, Reg::R1),
						Op::Immediate(Reg::R1, offset as u8)
					]),
					_ => code.extend(vec![
						Op::CSub(Reg::R0, Reg::R1),
						Op::Immediate(Reg::R1, offset as u8),
						Op::RotRightImm(Reg::R1, 8),
						Op::Immediate(Reg::R1, (offset >> 8) as u8)
					])
				}
				
				code.push(Op::CNot(Reg::R0, Reg::SP));
				*loc = Location::Memory(offset);
				
				code
			}
		}
	}
}


#[derive(Debug)]
pub enum FlatOp {
	Add(Factor),
	Sub(Factor),
}


#[derive(Debug)]
pub enum Statement {
	Let(bool, String, Option<Type>, Literal),
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
	
	Call(LValue, Vec<Factor>),
	Uncall(LValue, Vec<Factor>),
	
	If(BinExpr, Vec<Statement>, Option<Vec<Statement>>, BinExpr),
	
	From(BinExpr, Option<Vec<Statement>>, Option<Vec<Statement>>, BinExpr),
	
	//Switch(String, Vec<String, Vec<Statement>>),
	//Unsafe(Vec<Statement>),
}

impl Statement {
	named!(parse<Statement>, alt!(
		map!(preceded!(tag!("!"), LValue::parse), Statement::Not)
		| map!(preceded!(tag!("-"), LValue::parse), Statement::Neg)
		| do_parse!( // swap
			left: call!(LValue::parse) >>
			tag!("<>") >>
			right: call!(LValue::parse)
			>> (Statement::Swap(left, right))
		)
		| do_parse!( // ccnot, toffoli
			dest: call!(LValue::parse) >>
			tag!("^=") >>
			lctrl: call!(Factor::parse) >>
			tag!("&") >>
			rctrl: call!(Factor::parse)
			>> (Statement::CCNot(dest, lctrl, rctrl))
		)
		| do_parse!( // cswap, fredkin
			ctrl: call!(Factor::parse) >>
			tag!("?") >>
			left: call!(LValue::parse) >>
			tag!("<>") >>
			right: call!(LValue::parse)
			>> (Statement::CSwap(ctrl, left, right))
		)
		| ws!(do_parse!( // left rotate
			l: call!(LValue::parse) >>
			m: tag!("<<=") >>
			r: call!(Factor::parse)
			>> (Statement::RotLeft(l, r))
		))
		| ws!(do_parse!( // right rotate
			l: call!(LValue::parse) >>
			m: tag!(">>=") >>
			r: call!(Factor::parse)
			>> (Statement::RotRight(l, r))
		))
		| ws!(do_parse!( // xor
			l: call!(LValue::parse) >>
			tag!("^=") >>
			r: ws!(separated_nonempty_list!(
				tag!("^"),
				Factor::parse
			))
			>> (Statement::Xor(l, r))
		))
		| do_parse!( // increment, add
			l: call!(LValue::parse) >>
			tag!("+=") >>
			r: call!(Factor::parse) >>
			m: many0!(alt!(
				map!(preceded!(tag!("+"), Factor::parse), FlatOp::Add)
				| map!(preceded!(tag!("-"), Factor::parse), FlatOp::Sub)
			))
			
			>> (Statement::Add(l, {
				let mut m = m;
				m.insert(0, FlatOp::Add(r));
				m
			}))
		)
		| do_parse!( // decrement, subtract
			l: call!(LValue::parse) >>
			tag!("-=") >>
			r: call!(Factor::parse) >>
			m: many0!(alt!(
				map!(preceded!(tag!("+"), Factor::parse), FlatOp::Add)
				| map!(preceded!(tag!("-"), Factor::parse), FlatOp::Sub)
			))
			
			>> (Statement::Sub(l, {
				let mut m = m;
				m.insert(0, FlatOp::Add(r));
				m
			}))
		)
		/*
		| ws!(do_parse!(
			tag!("unsafe") >>
			b: block >>
			
			(Statement::Unsafe(b.into_iter()
				.filter_map(|s| s)
				.collect()
			))
		))
		*/
		| do_parse!( // call
			tag!("do") >>
			name: call!(LValue::parse) >>
			args: delimited!(
				tag!("("),
				ws!(separated_list!(tag!(","), Factor::parse)),
				tag!(")")
			)
			>> (Statement::Call(name, args))
		)
		| do_parse!( // uncall
			tag!("undo") >>
			name: call!(LValue::parse) >>
			args: delimited!(
				tag!("("),
				ws!(separated_list!(tag!(","), Factor::parse)),
				tag!(")")
			)
			>> (Statement::Uncall(name, args))
		)
		| ws!(do_parse!( // declare var
			tag!("let") >>
			m: opt!(tag!("mut")) >>
			name: ident >>
			ty: opt!(ws!(preceded!(tag!(":"), Type::parse))) >>
			tag!("=") >>
			l: call!(Literal::parse)
			>> (Statement::Let(m.is_some(), name, ty, l))
		))
		| ws!(do_parse!(
			tag!("drop") >>
			name: ident >>
			tag!("=") >>
			l: call!(Literal::parse)
			>> (Statement::Drop(name, l))
		))
		| do_parse!(
			tag!("if") >> p: call!(BinExpr::parse) >>
			t: block >>
			e: opt!(ws!(preceded!(tag!("else"), block))) >>
			tag!("fi") >> a: call!(BinExpr::parse)
			>> (Statement::If(p, t, e, a))
		)
		| do_parse!(
			tag!("from") >> a: call!(BinExpr::parse) >>
			d: opt!(block) >>
			tag!("until") >> p: call!(BinExpr::parse) >>
			l: opt!(block)
			>> (Statement::From(a, d, l, p))
		)
	));
	
	fn verify(&mut self) {
		match *self {
			Statement::Let(_, _, ref typ, ref lit) => {
				if let Some(ref typ) = *typ {
					match (typ, lit) {
						(&Type::Bool, &Literal::Bool(_))
						| (&Type::Char, &Literal::Char(_))
						| (&Type::U16, &Literal::Num(_))
						| (&Type::I16, &Literal::Num(_))
						| (&Type::Usize, &Literal::Num(_))
						| (&Type::Isize, &Literal::Num(_)) => {}
						_ => panic!("literal must match type given")
					}
				}
				/*
				// The following code is best left to a type check pass
				else {
					match *lit {
						Literal::Bool(_) => {typ.get_or_insert(Type::Bool);}
						Literal::Char(_) => {typ.get_or_insert(Type::Char);}
						_ => unimplemented!()
					}
				}
				*/
			}
			
			Statement::RotLeft(ref lval, ref factor)
			| Statement::RotRight(ref lval, ref factor) => {
				if let Factor::LVal(ref fac) = *factor {
					assert!(lval.id != fac.id, "value can't modify itself");
				}
			}
			
			Statement::CCNot(ref lval, ref c0, ref c1) => {
				if let Factor::LVal(ref fac) = *c0 {
					assert!(lval.id != fac.id, "value can't modify itself");
				}
				if let Factor::LVal(ref fac) = *c1 {
					assert!(lval.id != fac.id, "value can't modify itself");
				}
			}
			
			Statement::Xor(ref lval, ref facs) => {
				for fac in facs {
					if let Factor::LVal(ref fac) = *fac {
						assert!(lval.id != fac.id, "value can't modify itself");
					}
				}
			}
			
			Statement::Add(ref lval, ref flops)
			| Statement::Sub(ref lval, ref flops) => {
				for flop in flops {
					match *flop {
						FlatOp::Add(Factor::LVal(ref op)) =>
							assert!(lval.id != op.id, "value can't modify itself"),
						FlatOp::Sub(Factor::LVal(ref op)) =>
							assert!(lval.id != op.id, "value can't modify itself"),
						_ => {}
					}
				}
			}
			
			Statement::CSwap(ref fac, ref lv0, ref lv1) => {
				if let Factor::LVal(ref fac) = *fac {
					assert!(fac.id != lv0.id, "value can't modify itself");
					assert!(fac.id != lv1.id, "value can't modify itself");
				}
			}
			
			Statement::From(_, ref fbody, ref rbody, _) =>
				assert!(fbody.is_some() || rbody.is_some(), "Must provide at least 1 body."),
			
			_ => {}
		}
	}
	
	fn compile(&self, st: &mut SymbolTable) -> Vec<rel::Op> {
		use rel::Op;
		
		match *self {
			Statement::Let(..) => {
				vec![Op::Nop]
			}
			Statement::Drop(..) => {
				vec![Op::Nop]
			}
			
			Statement::Not(ref lval) => {
				lval.compile(st, |r| vec![Op::Not(r)])
			}
			Statement::Neg(ref lval) => {
				lval.compile(st, |r| vec![Op::Not(r), Op::Increment(r)])
			}
			
			Statement::RotLeft(..) => {
				/*
				lval.compile(st, |r| vec![])
				*/
				vec![Op::Nop]
			}
			Statement::RotRight(..) => {
				vec![Op::Nop]
			}
			
			Statement::CCNot(..) => {
				vec![Op::Nop]
			}
			Statement::Xor(..) => {
				vec![Op::Nop]
			}
			
			Statement::Add(..) => {
				vec![Op::Nop]
			}
			Statement::Sub(..) => {
				vec![Op::Nop]
			}
			
			Statement::Swap(..) => {
				vec![Op::Nop]
			}
			Statement::CSwap(..) => {
				vec![Op::Nop]
			}
			
			Statement::Call(..) => {
				/*
				let code = vec![];
				
				for fac in args {
					code.extend(fac.compile(st));
				}
				
				code.extend(lval.compile(st, |r| vec![
					Op::SwapPc(r),
					Op::Immediate(Reg::R1, (offset >> 8) as u8),
					Op::RotLeftImm(Reg::R1, 8),
					Op::Immediate(Reg::R1, offset as u8),
					Op::CSub(r, )
				]));
				code
				*/
				vec![Op::Nop]
			}
			Statement::Uncall(..) => {
				vec![Op::Nop]
			}
			
			Statement::If(..) => {
				vec![Op::Nop]
			}
			
			Statement::From(..) => {
				vec![Op::Nop]
			}
		}
	}
}


named!(block<Vec<Statement>>, ws!(delimited!(
	tag!("{"),
	// many0! is supressing error in stmt
	many0!(
		terminated!(Statement::parse, tag!(";"))
	),
	tag!("}")
)));

#[derive(Debug)]
pub struct Program {
	items: Vec<Item>,
}

impl Program {
	named!(pub parse<Program>, complete!(do_parse!(
		items: many0!(Item::parse)
		>> (Program { items })
	)));
	
	pub fn verify(&mut self) {
		for &mut Item::Fn(ref mut f) in &mut self.items {
			f.verify();
		}
	}
	
	pub fn compile(&self) -> Vec<rel::Op> {
		self.items.iter()
			.map(|&Item::Fn(ref f)| f.compile())
			.collect::<Vec<_>>()
			.concat()
	}
}