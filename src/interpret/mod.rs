use std::path::Path;
//use std::io::prelude::*;

use crate::ast;
use crate::hir::{Item, Module, Procedure};
use crate::tokenize::tokenize;

mod io;
mod value;

pub use self::value::Value;

#[derive(Debug, Clone)]
pub struct StackFrame {
    args: Vec<Value>,
    locals: Vec<(String, Value)>,
}

pub type EvalResult = Result<Value, &'static str>;
pub type Scope = Vec<(String, Value)>;
pub type Stack = Vec<StackFrame>;


pub fn interpret_file<P: AsRef<Path>>(path: P) {
	use std::fs::read_to_string as open;
	
	let source = open(path).expect("Could not read file");
	let mut tokens = tokenize(&source)
		.expect("Lexer error")
		.into_iter()
		.peekable();
	
	match ast::parse_file_module(&mut tokens) {
		Ok(ast) => {
			// create root module
			let mut root = Module::from(ast);
				
			root.0.insert(
				String::from("puts"),
				Item::InternProc(intrinsic::puts, intrinsic::unputs)
			);
			
			// run main procedure, if any
			if let Some(main) = root.0.get("main") {
				if let Item::Proc(pr) = main {
					pr.call(Vec::new(), &root);
				} else {
					eprintln!("found `main`, but it's not a procedure");
				}
			} else {
				eprintln!("No main procedure found.");
			}
		}
		Err(e) => {
			let remaining_tokens = tokens.clone()
				.collect::<Box<[_]>>();
			eprintln!("Expected {}.", e);
			eprintln!("Tokens: {:#?}", remaining_tokens);
		}
	}
}

pub mod intrinsic {
	use super::Value;
	use std::io::prelude::*;
	
	pub fn puts(args: Box<[Value]>) -> Box<[Value]> {
		assert!(args.len() == 1);
		
		let mut rstdout = super::io::RevStdout::new();
		let string = match &args[0] {
			Value::String(s) => s.as_bytes(),
			_ => panic!("not a string")
		};
		
		rstdout.write(string);
		
		args
	}
	
	pub fn unputs(args: Box<[Value]>) -> Box<[Value]> {
		assert!(args.len() == 1);
		
		let mut rstdout = super::io::RevStdout::new();
		let string = match &args[0] {
			Value::String(s) => s.as_bytes(),
			_ => panic!("not a string")
		};
		
		rstdout.unwrite(string, string.len());
		
		args
	}
}
