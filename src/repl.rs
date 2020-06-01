use std::collections::HashMap;

use crate::tokenize::{Token, Tokens};
use crate::ast::{self, Expr, Item, Statement};
use crate::hir;
use crate::interpret;


pub struct Scope {
	vars:  Vec<(String, interpret::Value)>,
	items: HashMap<String, hir::Item>,
}

impl Scope {
	pub fn new() -> Self {
		Self {
			vars: Vec::new(),
			items: HashMap::new(),
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
	
	pub fn eval_line(&mut self, line: ReplLine) -> Result<Option<interpret::Value>, ()> {
		match line {
			ReplLine::Skip => Ok(None),
			ReplLine::Show(var) => {
				Ok(self.get(&var).cloned())
			}
			ReplLine::Var(name, expr) => {
				let expr: hir::Expr = expr.clone().into();
				let val = expr.eval(&mut self.vars).unwrap();
				self.vars.push((name.clone(), val));
				Ok(None)
			}
			ReplLine::Drop(name) => {
				let val = self.vars.iter()
					.enumerate()
					.rfind(|(_, (k,_))| *k == name)
					.map(|(i,_)| i);
				
				match val {
					None => Err(()),
					Some(i) => Ok(Some(self.vars.remove(i).1)),
				}
			}
			ReplLine::Item(item) => {
				self.items.insert(item.get_name().to_string(), item.into());
				Ok(None)
			}
			ReplLine::Stmt(stmt) => {
				let stmt: hir::Statement = stmt.into();
				let module = hir::Module(self.items.clone());
				
				stmt.eval(&mut self.vars, &module).unwrap();
				Ok(None)
			}
		}
	}
}

pub enum ReplLine {
	Skip,
	Show(String),
	Var(String, Expr),
	Drop(String),
	
	Item(Item),
	Stmt(Statement),
}

impl ast::Parse for ReplLine {
	fn parse(tokens: &mut Tokens) -> ast::ParseResult<Self> {
		match tokens.peek() {
			None => Ok(ReplLine::Skip),
			Some(Token::Var) => {
				tokens.next();
				
				let name = match tokens.next() {
					Some(Token::Ident(n)) => n,
					_ => return Err("variable name after `var`")
				};
				
				tokens.expect(&Token::Assign)
					.ok_or("`:=` after variable name")?;
				
				let init = Expr::parse(tokens)?;
				
				Ok(ReplLine::Var(name, init))
			}
			Some(Token::Drop) => {
				tokens.next();
				
				let name = match tokens.next() {
					Some(Token::Ident(n)) => n,
					_ => return Err("variable name after `drop`")
				};
				
				Ok(ReplLine::Drop(name))
			}
			Some(Token::Ident(id)) if id == "show" => {
				tokens.next();
				
				let name = match tokens.next() {
					Some(Token::Ident(name)) => name,
					_ => return Err("variable name after `show`"),
				};
				
				Ok(ReplLine::Show(name))
			}
			Some(tok) => match tok {
				Token::Fn | Token::Proc | Token::Mod =>
					Ok(ReplLine::Item(Item::parse(tokens)?)),
				
				_ => Ok(ReplLine::Stmt(Statement::parse(tokens)?)),
			}
		}
	}
}

enum Error {
	SymbolNotFound,
}
