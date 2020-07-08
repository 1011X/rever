//use std::io::prelude::*;

use crate::ast;
use crate::hir::{Item, Module};

mod io;
mod value;
mod intrinsic;

pub use self::value::Value;

#[derive(Debug, Clone)]
pub struct StackFrame {
	args: Vec<Value>,
	locals: Vec<(String, Value)>,
}

pub type Scope = Vec<(String, Value)>;
pub type Stack = Vec<StackFrame>;
pub type EvalResult = Result<Value, &'static str>;

pub trait Eval {
	fn eval(&self, scope: &Scope) -> EvalResult;
}

pub enum EvalError {
	TypeMismatch,
	
}


pub fn interpret_file(items: Vec<ast::Item>) {
	// create root module
	let mut root = Module::from(items);
		
	root.0.insert(
		String::from("puts"),
		Item::InternProc(intrinsic::puts, intrinsic::unputs)
	);
	
	// run main procedure, if any
	if let Some(main) = root.0.get("main") {
		if let Item::Proc(pr) = main {
			println!("running `proc main`...");
			pr.call(Vec::new(), &root);
		} else {
			eprintln!("found `main`, but it's not a procedure");
		}
	} else {
		eprintln!("No main procedure found.");
	}
}
