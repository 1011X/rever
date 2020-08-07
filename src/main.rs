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
		None => repl::repl()?,
		
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
