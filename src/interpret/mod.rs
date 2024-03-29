//use std::io::prelude::*;
use std::fmt;

use crate::ast::{self, Item, Module, Type, Procedure, Param, ProcDef};

pub use self::value::Value;
pub use self::stack::{Stack, StackFrame, Context};

mod io;
mod value;
pub mod intrinsic;
mod stack;

//pub type Scope = Vec<(String, Value)>;
pub type EvalResult<T> = Result<T, EvalError>;

#[derive(Debug)]
pub enum EvalError {
	TypeMismatch {
		expected: Type,
		got: Type,
	},
	UnknownIdent(String),
	IrreversibleState,
}

impl fmt::Display for EvalError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			EvalError::UnknownIdent(id) =>
				write!(f, "name {:?} is not defined", id),
			EvalError::TypeMismatch { expected, got } =>
				write!(f, "expected {:?}, got {:?}", expected, got),
			EvalError::IrreversibleState =>
				f.write_str("hit an irreversible state"),
		}
	}
}
/*
// Creates root module, loads intrinsics, finds `main`, and executes it.
pub fn interpret_file(items: Vec<ast::Item>) {
	// create root module
	let mut root = Module::new("root".into(), items);
		
	root.items.push(
		Item::Proc(Procedure {
			name: "show".to_string(),
			params: vec![Param {
				name: "string".to_string(),
				constant: false,
				typ: Type::String,
			}],
			code: ProcDef::Internal {
				fore: intrinsic::show,
				back: intrinsic::unshow
			},
		})
	);
	
	// find main procedure
	let main = root.items.iter()
		.find(|item| matches!(item, Item::Proc(pr) if pr.name == "main"));
	
	// TODO set up stack
	
	// run main procedure, if any
	if let Some(Item::Proc(pr)) = main {
		pr.call(Vec::new(), &root).unwrap();
	} else {
		eprintln!("No main procedure found.");
	}
}
*/
