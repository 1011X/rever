use std::collections::HashMap;
use super::*;
use super::super::compile::Location;
use rel;

#[derive(Debug)]
pub struct Function {
	/// Name of the function.
	pub name: String,
	/// Arguments' setup within the function
	pub args: Vec<Arg>,
	/// Sequence of statements that make up the function.
	pub code: Vec<Statement>,
}

impl Function {
	named!(pub parse<Self>, ws!(do_parse!(
		tag!("fn") >>
		name: ident >>
		args: delimited!(
			tag!("("),
			separated_list!(tag!(","), Arg::parse),
			tag!(")")
		) >>
		code: block
		>> (Function { name, args, code })
	)));
	
	pub fn verify(&mut self) {
		for statement in &mut self.code {
			statement.verify();
		}
		/*
		let decls: Vec<&Statement> = self.code.iter()
			.filter(|&stmt| match *stmt {
				Statement::Let(true, ..) | Statement::Drop(..) => true,
				_ => false
			})
			.collect();
		
		decls.sort_by_key(|&stmt| match *stmt {
			Statement::Let(_, ref id, ..)
			| Statement::Drop(ref id, ..) => id,
			_ => unreachable!()
		});
		
		decls.dedup_by(|&s0, &s1| )
		
		for decl in decls.chunks(2)
		*/
	}
	
	pub fn compile(&self) -> Vec<rel::Op> {
		let mut body = vec![];
		// every symbol is associated with a location, and therefore a value
		let mut symbol_table = HashMap::new();
		
		// Add arguments to symbol table. Pascal convention is used.
		for (i, arg) in self.args.iter().rev().enumerate() {
			symbol_table.insert(arg.name.clone(), Location::Memory(-(i as isize)));
		}
		
		println!("Symbols: {:?}", symbol_table);
		
		// Compile body.
		for statement in &self.code {
			body.extend(statement.compile(&mut symbol_table));
		}
		
		println!("Code for {}: {:#?}", self.name, body);
		body
	}
}
