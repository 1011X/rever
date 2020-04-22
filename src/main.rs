/*
Stuff to consider adding:
+ Annotations?
  + Could be used like hashtags where an items gets "tagged".
  + `#final fn f(): ...`
+ Use `.` for "alternatives"
  + E.g. module paths, tagged unions, etc.
+ Use indexing, like `a[0]`, for "composites".
  + E.g. struct fields, array elements, etc.
+ Should "objects" have a prototype like in Javascript?
+ Should "objects" be more like a set of procedures/functions that data structs
  implement?
+ Should everything just be "tuples" and "enums" that implement "interfaces"?
+ Should i just go to sleep already???

TODO:
+ Evaluation
+ stdlib & prelude
+ more examples from Janus

*/

#![allow(unused_variables)]
#![allow(dead_code)]

use std::env;

pub mod ast;
//pub mod compile;
pub mod interpret;
pub mod parse;
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
		
		Some(cmd) if cmd == "do" => {
			unimplemented!()
		}
		
		Some(cmd) if cmd == "ast" => {
			use std::fs::read_to_string as open;
			
			let path = args.next().expect("Must provide a path.");
			
			let source = open(path).expect("Could not read file");
			let mut tokens = tokenize::tokenize(&source)
				.expect("Could not tokenize")
				.into_iter()
				.peekable();
			
			match parse::parse_items(&mut tokens) {
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
			interpret::interpret_file(file);
		}
	} 
}
