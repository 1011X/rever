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
use std::io;
use std::io::prelude::*;

use crate::ast::Parse;
//use crate::interpret;

mod tokenize;
mod ast;
mod hir;
//mod compile;
mod interpret;
mod repl;

fn main() -> io::Result<()> {
	let mut args = env::args().skip(1);
	
	match args.next() {
		// start REPL
		None => {
			let stdin = io::stdin();
			let mut input = String::new();
			let mut stdout = io::stdout();
			let mut scope = repl::Scope::new();
			
			println!("Rever 0.0.1");
			println!("Type \"show x\" to display the value of x.");
			
			loop {
				print!("< ");
				stdout.flush()?;
				stdin.read_line(&mut input)?;
				
				let mut tokens = tokenize::tokenize(&input)
					.expect("Could not tokenize");
				let line = repl::ReplLine::parse(&mut tokens);
				
				if let Err(e) = line {
					eprintln!("! Invalid input: expected {}.", e);
					input.clear();
					continue;
				}
				let line = line.unwrap();
				
				if scope.eval_line(line).is_ok() {
					input.clear();
				} else {
					break;
				}
			}
		}
		
		// interpret stdin
		/*
		Some(arg) if arg == "-" => {
			let mut source = String::new();
			io::stdin().read_to_string(&mut source)?;
			
			let mut tokens = tokenize::tokenize(&source)
				.expect("Could not tokenize");
			
			match parse_file_module(&mut tokens) {
				Ok(ast) => {
					println!("AST: {:#?}", ast);
				}
				Err(e) => {
					let remaining_tokens = tokens.as_inner();
					eprintln!("Expected {}.", e);
					eprintln!("Tokens: {:#?}", remaining_tokens);
				}
			}
		}
		*/
		
		// interpret file
		Some(file) => {
			use std::fs::read_to_string as open;
			
			let source = open(file)?;
			let mut tokens = tokenize::tokenize(&source)
				.expect("Lexer error");
			
			let ast = match ast::parse_file_module(&mut tokens) {
				Ok(ast) => ast,
				Err(e) => {
					let remaining_tokens = tokens.as_inner();
					eprintln!("Expected {}.", e);
					eprintln!("Tokens: {:#?}", remaining_tokens);
					return Ok(())
				}
			};
			
			interpret::interpret_file(ast.into());
		}
	}
	
	Ok(())
}
