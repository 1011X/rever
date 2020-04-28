use std::path::Path;
//use std::io::prelude::*;

use crate::ast::{Item, Module, Procedure};
use crate::parse;
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
	
	match parse::parse_items(&mut tokens) {
		Ok(mut ast) => {
			// look for main function
			let main_pos = ast.iter()
				.position(|item| matches!(
					item,
					Item::Proc(p) if p.name == "main"
				));
			
			// run main procedure, if any
			if let Some(pos) = main_pos {
				let main = ast.remove(pos);
				
				// create root module
				let mut root_mod = Module::new("root", ast);
				
				root_mod.items.push(
					Item::InternProc("puts", intrinsic::puts, intrinsic::unputs)
				);
				
				if let Item::Proc(pr) = main {
					pr.call(Vec::new(), &root_mod);
				} else {
					unreachable!();
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
