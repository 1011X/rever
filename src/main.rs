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

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use clap::Parser;
use logos::Logos;

use std::env;
use std::io;
use std::path::PathBuf;

//use crate::ast::Parse;
//use crate::interpret;
use crate::token::Token;

mod span;
mod token;
mod ast;
//mod hir;
//mod compile;
mod interpret;
mod repl;


/// Test.
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
	/*
	/// Forces opening the REPL. If any paths are provided, they will be loaded
	/// in.
	#[clap(short, long)]
	interactive: bool,
	*/
	/// Path to a Rever file.
	file: Option<PathBuf>,
}

fn main() -> io::Result<()> {
	let args = Args::parse();
	
	repl::init()?;
	
	/*
	match args.file {
		// start REPL
		None => repl::init()?,
		
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
		Some(path) => {
			use std::fs::read_to_string as open;
			
			let source = open(path)?;
			let tokens = Token::lexer(&source);
			let mut parser = ast::Parser::new(tokens);
			
			let ast = match parser.parse_file_module() {
				Ok(ast) => ast,
				Err(e) => {
					/*
					eprintln!("Error: expected {}.", e);
					eprintln!("Remaining source:\n{}", parser.tokens.remainder());
					*/
					eprintln!(
						"rever: Parser error at line {}, column {}!",
						parser.line(),
						parser.column()
					);
					eprintln!("rever: Expected {}, got {:?}", e, parser.slice());
					eprintln!("rever: Remaining source:\n{}", parser.remainder());
					
					return Ok(())
				}
			};
			
//			println!("{:#?}", ast);
			interpret::interpret_file(ast.into());
		}
	}
	*/
	
	Ok(())
}
