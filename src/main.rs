/*
Stuff to consider adding:
+ Annotations?
  + Could be used like hashtags where an items gets "tagged".
  + `#final fn f(): ...`
+ Should "objects" be more like a set of procedures/functions that data structs
  implement?

TODO:
+ Evaluation
+ stdlib & prelude

*/

#![allow(unused_variables)]
#![allow(dead_code)]

use std::env;
use std::io;
use std::io::prelude::*;
use logos::Logos;

//use crate::ast::Parse;
//use crate::interpret;
use crate::tokenize::Token;

mod span;
mod tokenize;
mod ast;
//mod hir;
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
				
				let tokens = Token::lexer(&input);
				let mut parser = ast::Parser::new(tokens);
				let line = parser.parse_repl_line();
				
				if let Err(e) = line {
					eprintln!("! Error: expected {}.", e);
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
			
			let mut tokens = Token::lexer(&source);
			
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
			let tokens = Token::lexer(&source);
			let mut parser = ast::Parser::new(tokens);
			
			let ast = match parser.parse_file_module() {
				Ok(ast) => ast,
				Err(e) => {
					//let remaining_tokens = tokens.as_slice();
					eprintln!("Error: expected {}.", e);
					eprintln!("Remaining source:\n{}", parser.tokens.remainder());
					return Ok(())
				}
			};
			
			interpret::interpret_file(ast.into());
		}
	}
	
	Ok(())
}
