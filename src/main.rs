/*
Stuff to consider adding:
+ Annotations?
  + Could be used like hashtags where an items gets "tagged".
  + `#final fn f(): ...`
+ Use `.` for "alternatives"
  + E.g. module paths, tagged unions, etc.
+ Should "objects" have a prototype like in Javascript?
+ Should "objects" be more like a set of procedures/functions that data structs
  implement?
+ Should everything just be "tuples" and "enums" that implement "interfaces"?
+ Should i just go to sleep already???

TODO:
+ Evaluation
+ stdlib & prelude

*/

#![allow(unused_variables)]
#![allow(dead_code)]

use std::env;

use crate::ast::parse_file_module;

pub mod ast;
//pub mod compile;
pub mod hir;
pub mod interpret;
pub mod tokenize;

fn main() {
	let mut args = env::args().skip(1);
	let subcmd = args.next();
	
	match subcmd {
		// start REPL
		None => {
			print!(">>> ");
			unimplemented!();
			
			
		}
		
		// interpret stdin
		/*
		Some(arg) if arg == "-" => {
			let mut source = String::new();
			io::stdin().read_to_string(&mut source).expect("File error");
			
			let tokens = tokenize(&source).expect("Could not tokenize");
			
			println!("{:#?}", tokens);
		}
		*/
		
		// interpret file
		Some(file) => {
			use std::fs::read_to_string as open;
			
			let path = args.next().expect("Must provide a path.");
			
			let source = open(path).expect("Could not read file");
			
			let mut tokens = tokenize::tokenize(&source)
				.expect("Could not tokenize")
				.into_iter()
				.peekable();
			
			match parse_file_module(&mut tokens) {
				Ok(ast) => {
					println!("AST: {:#?}", ast);
				}
				Err(e) => {
					let remaining_tokens = tokens.clone()
						.collect::<Box<[_]>>();
					eprintln!("Expected {}.", e);
					eprintln!("Tokens: {:#?}", remaining_tokens);
				}
			}
		}
	} 
}
