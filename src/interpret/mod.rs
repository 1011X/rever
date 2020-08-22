//use std::io::prelude::*;

use crate::ast::{self, Item, Module, Type, Procedure, Param, ProcDef};

pub use self::value::Value;
pub use self::stack::{Stack, StackFrame};

mod io;
mod value;
mod intrinsic;
mod stack;

//pub type Scope = Vec<(String, Value)>;
pub type EvalResult<T> = Result<T, EvalError>;

pub trait Eval {
	fn eval(&self, scope: &StackFrame) -> EvalResult<Value>;
}

#[derive(Debug)]
pub enum EvalError {
	TypeMismatch {
		expected: Type,
		got: Type,
	},
	UnknownIdent(String),
}


pub fn interpret_file(items: Vec<ast::Item>) {
	// create root module
	let mut root = Module::new("root".into(), items);
		
	root.items.push(
		Item::Proc(Procedure {
			name: "show".to_string(),
			params: vec![Param {
				name: "string".to_string(),
				mutable: false,
				typ: Type::String,
			}],
			code: ProcDef::Internal {
				fore: intrinsic::show,
				back: intrinsic::unshow
			},
		})
	);
	
	let main = root.items.iter()
		.find(|item| matches!(item, Item::Proc(pr) if pr.name == "main"));
	
	// TODO set up stack
	
	// run main procedure, if any
	if let Some(main) = main {
		if let Item::Proc(pr) = main {
			pr.call(Vec::new(), &root);
		} else {
			eprintln!("no `proc main` found");
		}
	} else {
		eprintln!("No main procedure found.");
	}
}
