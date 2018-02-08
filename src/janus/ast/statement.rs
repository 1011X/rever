//use std::collections::HashMap;
//use super::interpret::{SymTab, Value};
use super::*;
use super::super::compile::State;
use rel;

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
	Error(String),
	Printf(String, Vec<Factor>),
	Show(LValue),
	Pop(Factor, LValue),
	Push(Factor, LValue),
}

impl Statement {
	named!(pub parse<Self>, sp!(alt_complete!(
		value!(Statement::Skip, tag!("skip"))
		| do_parse!(
			op: alt!(tag!("local") | tag!("delocal")) >>
			decl: call!(Decl::parse) >>
			tag!("=") >>
			val: call!(Expr::parse)
			>> (match op {
				b"local" => Statement::Local(decl, val),
				b"delocal" => Statement::Delocal(decl, val),
				_ => unreachable!()
			})
		)
		| do_parse!(
			left: call!(LValue::parse) >>
			tag!("<=>") >>
			right: call!(LValue::parse)
			>> (Statement::Swap(left, right))
		)
		| do_parse!(
			left: call!(LValue::parse) >>
			modop: alt!(tag!("+=") | tag!("-=") | tag!("^=")) >>
			expr: call!(Expr::parse)
			>> (match modop {
				b"+=" => Statement::Add(left, expr),
				b"-=" => Statement::Sub(left, expr),
				b"^=" => Statement::Xor(left, expr),
				_ => unreachable!()
			})
		)
		| do_parse!(
			tag!("from") >>
			assert: call!(Pred::parse) >>
			forward: opt!(preceded!(
				tag!("do"),
				many1!(Statement::parse)
			)) >>
			backward: opt!(preceded!(
				tag!("loop"),
				many1!(Statement::parse)
			)) >>
			tag!("until") >>
			pred: call!(Pred::parse)
			
			>> (Statement::From(assert, forward, backward, pred))
		)
		| do_parse!(
			tag!("if") >>
			pred: call!(Pred::parse) >>
			tag!("then") >>
			pass: many1!(Statement::parse) >>
			fail: opt!(preceded!(
				tag!("else"),
				many1!(Statement::parse)
			)) >>
			tag!("fi") >>
			assert: call!(Pred::parse)
			
			>> (Statement::If(pred, pass, fail, assert))
		)
		| do_parse!(
			op: alt!(tag!("call") | tag!("uncall")) >>
			func: ident >>
			tag!("(") >>
			args: separated_list!(tag!(","), Factor::parse) >>
			tag!(")")
			>> (match op {
				b"call"   => Statement::Call(func, args),
				b"uncall" => Statement::Uncall(func, args),
				_ => unreachable!()
			})
		)
		// built-ins
		| do_parse!(
			func: alt!(tag!("print") | tag!("error")) >>
			tag!("(") >>
			string: st >>
			tag!(")")
			>> (match func {
				b"print" => Statement::Print(string),
				b"error" => Statement::Error(string),
				_ => unreachable!()
			})
		)
		| do_parse!(
			tag!("printf") >>
			tag!("(") >>
			string: st >>
			vargs: many0!(preceded!(
				tag!(","),
				Factor::parse
			)) >>
			tag!(")")
			>> (Statement::Printf(string, vargs))
		)
		| do_parse!(
			tag!("show") >>
			tag!("(") >>
			lval: call!(LValue::parse) >>
			tag!(")")
			>> (Statement::Show(lval))
		)
		| do_parse!(
			op: alt!(tag!("pop") | tag!("push")) >>
			tag!("(") >>
			val: call!(Factor::parse) >>
			tag!(",") >>
			stack: call!(LValue::parse) >>
			tag!(")")
			>> (match op {
				b"pop"  => Statement::Pop(val, stack),
				b"push" => Statement::Push(val, stack),
				_ => unreachable!()
			})  
		)
	)));
	
	pub fn compile(&self, state: &mut State, code: &mut Vec<rel::Op>) {
		use rel::Op;
		use self::Statement::*;
		match *self {
			Skip | Print(..) | Printf(..) | Show(..) | Error(..)
				=> {},
			
			Add(ref lval, ref expr) => {
				let reg_lval = lval.compile(state, code);
				let reg_expr = expr.compile(state, code);
				
				code.push(Op::Add(reg_lval, reg_expr));
			}
			_ => unimplemented!()
		}
	}
	/*
	pub fn eval(&self, globs: &mut SymTab) {
		use self::Statement::*;
		match *self {
			Skip => (),
			Local(ref decl, ref expr) => {
				globs.insert(decl.name.clone(), expr.eval(globs).unwrap());
			}
			Delocal(ref decl, ref expr) => {
				let value = globs.remove(&decl.name).unwrap();
				assert_eq!(value, expr.eval(globs).unwrap());
			}
			Add(ref lval, ref expr) => {
				//let var = lval.eval();
				let entry = globs.get_mut(&lval.name).unwrap();
				match *entry {
					Value::Int(ref mut num) => {
						if let Value::Int(n) = expr.eval(globs).unwrap() {
							*num += n;
						}
					}
					_ => unimplemented!()
					//Value::IntArray(ref vec) =>
				}
			}
			_ => unimplemented!()
		}
	}
	*/
}
