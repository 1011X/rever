use std::env;
use std::fs::read_to_string as open;
use std::io;
use std::io::prelude::*;

use rever::tokenize::tokenize;
use rever::interpret;

fn main() {
	match env::args().skip(1).next() {
		// start REPL
		None => {
			print!(">>> ");
			unimplemented!();
		}
		
		Some(ref arg) if arg == "-" => {
			let mut source = String::new();
			io::stdin().read_to_string(&mut source).expect("File error");
			
			let tokens = tokenize(&source).expect("Could not tokenize");
			
			println!("{:#?}", tokens);
		}
		
		// interpret file
		Some(file) => {
			let source = open(file).expect("Could not read file");
			
			println!("Source:\n{:?}\n", source);
			
			let tokens = tokenize(&source).expect("Could not tokenize");
			
			println!("Tokens: {:#?}", tokens);
			
			let ast = interpret::parse_module(&tokens).expect("Syntax error");
			
			println!("AST: {:#?}", ast);
		}
	} 
}
