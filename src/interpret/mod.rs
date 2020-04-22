use std::path::Path;

use crate::ast::{Item, Module, Procedure};
use crate::parse;
use crate::tokenize::tokenize;

//mod io;
mod value;

pub use self::value::Value;

pub type EvalResult = Result<Value, &'static str>;
pub type Scope = Vec<(String, Value)>;

#[derive(Debug, Clone)]
pub struct StackFrame {
    args: Vec<Value>,
    locals: Vec<(String, Value)>,
}

pub type Stack = Vec<StackFrame>;

/*
TODO: create some sort of trait(s) or generic struct(s) that can:
+ Take an input device and store data that's given back.
  + Data that's given back is stored in a stack.
  + Data from the stack is used before data from the device.
+ Take an output device and return data that was passed to it.
  + Data that's given back is stored in a stack.
  + Data from the stack can be passed back until it's empty.
  + Should stdout be written to immediately? or on program close? or have a 
    flush procedure that can be called?

The goal is to have stdin and stdout behave as close to files as possible.
*/

/** A handle to a reversible standard input stream.

When data is sent back here, e.g. in cases where IO is being rejected */
struct RevStdin {
	stdin: std::io::Stdin,
	stack: Vec<u8>,
}

impl RevStdin {
	fn reset(&mut self) { self.stack.clear() }
	
	fn read_byte(&mut self, buf: &mut Vec<u8>) -> EvalResult {
		unimplemented!()
	}
}

pub fn interpret_file<P: AsRef<Path>>(path: P) {
	use std::fs::read_to_string as open;
	
	let source = open(path).expect("Could not read file");
	let mut tokens = tokenize(&source)
		.expect("Lexer error")
		.into_iter()
		.peekable();
	
	match parse::parse_items(&mut tokens) {
		Ok(mut ast) => {
			println!("AST successfully created.");
			
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
				let root_mod = Module::new("root", ast);
				
				if let Item::Proc(pr) = main {
					println!("Running...");
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
