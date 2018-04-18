use super::*;
use super::super::compile::SymbolTable;
use rel;

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
	
	Do(LValue, Vec<Factor>),
	Undo(LValue, Vec<Factor>),
	
	If(BinExpr, Vec<Statement>, Option<Vec<Statement>>, BinExpr),
	
	From(BinExpr, Option<Vec<Statement>>, Option<Vec<Statement>>, BinExpr),
	
	//Switch(String, Vec<String, Vec<Statement>>),
	//Unsafe(Vec<Statement>),
}

impl Statement {
	named!(pub parse<Self>, ws!(alt_complete!(
		map!(preceded!(tag!("!"), LValue::parse), Statement::Not)
		| map!(preceded!(tag!("-"), LValue::parse), Statement::Neg)
		| do_parse!( // procedure call; do/undo
			kw: alt!(tag!("do") | tag!("undo")) >>
			name: call!(LValue::parse) >>
			args: delimited!(
				tag!("("),
				separated_list!(tag!(","), Factor::parse),
				tag!(")")
			)
			>> (match kw {
				b"do" => Statement::Do(name, args),
				b"undo" => Statement::Undo(name, args),
				_ => unreachable!()
			})
		)
		| do_parse!( // let var
			tag!("let") >>
			m: opt!(tag!("mut")) >>
			name: ident >>
			ty: opt!(preceded!(tag!(":"), Type::parse)) >>
			tag!("=") >>
			l: call!(Literal::parse)
			>> (Statement::Let(m.is_some(), name, ty, l))
		)
		| do_parse!( // drop var
			tag!("drop") >>
			name: ident >>
			tag!("=") >>
			l: call!(Literal::parse)
			>> (Statement::Drop(name, l))
		)
		| do_parse!(
			tag!("if") >> p: call!(BinExpr::parse) >>
			t: block >>
			e: opt!(preceded!(tag!("else"), block)) >>
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
		| do_parse!( // rotation
			l: call!(LValue::parse) >>
			op: alt!(tag!("<<=") | tag!(">>=")) >>
			r: call!(Factor::parse)
			>> (match op {
				b"<<=" => Statement::RotLeft(l, r),
				b">>=" => Statement::RotRight(l, r),
				_ => unreachable!()
			})
		)
		| do_parse!( // xor
			l: call!(LValue::parse) >>
			tag!("^=") >>
			r: separated_nonempty_list!(
				tag!("^"),
				Factor::parse
			)
			>> (Statement::Xor(l, r))
		)
		| do_parse!( // increment/add, decrement/subtract
			l: call!(LValue::parse) >>
			op: alt!(tag!("+=") | tag!("-=")) >>
			r: call!(Factor::parse) >>
			m: many0!(alt_complete!(
				map!(preceded!(tag!("+"), Factor::parse), FlatOp::Add)
				| map!(preceded!(tag!("-"), Factor::parse), FlatOp::Sub)
			))
			
			>> ({
				let mut m = m;
				m.insert(0, FlatOp::Add(r));
				match op {
					b"+=" => Statement::Add(l, m),
					b"-=" => Statement::Sub(l, m),
					_ => unreachable!()
				}
			})
		)
	)));
	
	pub fn verify(&mut self) {
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
	
	pub fn compile(&self, st: &mut SymbolTable) -> Vec<rel::Op> {
		use rel::Op;
		
		match *self {
			Statement::Not(ref lval) => {
				lval.compile(st, |r| vec![Op::Not(r)])
			}
			Statement::Neg(ref lval) => {
				lval.compile(st, |r| vec![Op::Not(r), Op::AddImm(r, 1)])
			}
			/*
			Statement::Call(..) => {
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
				vec![Op::Nop]
			}
			*/
			_ => {
				vec![Op::Nop]
			}
		}
	}
}
