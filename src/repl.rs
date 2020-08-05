//use std::collections::HashMap;

use crate::tokenize::Token;
use crate::ast::{self, Expr, Item, Module, Statement};
//use crate::hir;
use crate::interpret::{self, Eval};


pub struct Scope {
	vars:  Vec<(String, interpret::Value)>,
	items: Vec<Item>,
}

impl Scope {
	pub fn new() -> Self {
		Self {
			vars: Vec::new(),
			items: Vec::new(),
		}
	}
	
	pub fn push(&mut self, name: String, val: interpret::Value) {
		self.vars.push((name, val));
	}
	
	pub fn pop(&mut self, name: String, val: interpret::Value) {
		assert_eq!(self.vars.pop(), Some((name, val)));
	}
	
	pub fn get(&self, name: &str) -> Option<&interpret::Value> {
		let mut val = None;
		for (k, v) in self.vars.iter().rev() {
			if k == name {
				val = Some(v);
				break;
			}
		}
		val
	}
	
	pub fn eval_line(&mut self, line: ReplLine) -> Result<(), ()> {
		match line {
			ReplLine::Show(var) => {
				if let Some(val) = self.get(&var) {
					println!(": {:?}", val);
				}
			}
			
			ReplLine::Var(name, expr) => {
				let val = expr.eval(&mut self.vars).unwrap();
				self.vars.push((name.clone(), val));
			}
			
			ReplLine::Drop(name) => {
				let val = self.vars.iter()
					.enumerate()
					.rfind(|(_, (k,_))| *k == name)
					.map(|(i,_)| i);
				
				match val {
					Some(i) => println!(": {:?}", self.vars.remove(i).1),
					None => return Err(()),
				}
			}
			// TODO return Err for item and stmt when not enough input.
			ReplLine::Item(item) => {
				self.items.push(item);
			}
			
			ReplLine::Stmt(stmt) => {
				let module = Module::new("repl".into(), self.items.clone());
				stmt.eval(&mut self.vars, &module).unwrap();
			}
		}
		Ok(())
	}
}

#[derive(Debug, Clone)]
pub enum ReplLine {
	Show(String),
	Var(String, Expr),
	Drop(String),
	
	Item(Item),
	Stmt(Statement),
}

impl ast::Parser<'_> {
	pub fn parse_repl_line(&mut self) -> ast::ParseResult<ReplLine> {
		Ok(match self.peek() {
			None => todo!(),
			Some(Token::Var) => {
				self.next();
				
				let name = self.expect_ident()
					.ok_or("variable name after `var`")?;
				
				self.expect(&Token::Assign)
					.ok_or("`:=` after variable name")?;
				
				let init = self.parse_expr()?;
				
				ReplLine::Var(name, init)
			}
			Some(Token::Drop) => {
				self.next();
				
				let name = self.expect_ident()
					.ok_or("variable name after `drop`")?;
				
				ReplLine::Drop(name)
			}
			Some(Token::Ident(id)) if id == "show" => {
				self.expect_ident();
				
				let name = self.expect_ident()
					.ok_or("variable name after `show`")?;
				
				ReplLine::Show(name)
			}
			
			Some(Token::Fn)
			| Some(Token::Proc)
			| Some(Token::Mod) => {
				self.parse_item()?.into()
			}
				
			Some(_) => {
				self.parse_stmt()?.into()
			}
		})
	}
}

impl From<Item> for ReplLine {
	fn from(item: Item) -> Self { ReplLine::Item(item) }
}

impl From<Statement> for ReplLine {
	fn from(stmt: Statement) -> Self { ReplLine::Stmt(stmt) }
}

enum Error {
	SymbolNotFound,
}
