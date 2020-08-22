use std::io::{self, prelude::*};
use logos::Logos;

use crate::token::Token;
use crate::ast::{self, LValue, Expr, Item, Module, Stmt};
use crate::interpret::{Eval, EvalResult, Stack, StackFrame, Value};

pub fn init() -> io::Result<()> {
	let stdin = io::stdin();
	let mut input = String::new();
	let mut stdout = io::stdout();
	let mut continuing = false;
	
	let mut module = Module::new("repl".into(), Vec::new());
	let mut stack = Stack::new();
	let root_frame = StackFrame::new(Vec::new());
	stack.push(root_frame);
	
	println!("Rever 0.0.1");
	println!("Type \"show x\" to display the value of x.");
	
	loop {
		let prompt = if continuing { "|" } else { "<" };
		print!("{} ", prompt);
		stdout.flush()?;
		stdin.read_line(&mut input)?;
		
		// read
		let tokens = Token::lexer(&input);
		let mut parser = ast::Parser::new(tokens);
		
		let line = match parser.parse_repl_line() {
			Ok(line) => line,
			Err(ast::ParseError::Eof) => {
				continuing = true;
				continue;
			}
			Err(e) => {
				eprintln!("! Invalid input: {}.", e);
				input.clear();
				continuing = false;
				continue;
			}
		};
		
		// eval
		if line.eval(stack.last_mut().unwrap(), &mut module).is_ok() {
			input.clear();
		} else {
			return Ok(());
		}
		
		continuing = false;
	}
}

#[derive(Debug, Clone)]
pub enum ReplLine {
	Show(LValue),
	//Expr(Expr),
	
	Var(String, Expr),
	Drop(String),
	
	Item(Item),
	Stmt(Stmt),
}

impl ast::Parser<'_> {
	pub fn parse_repl_line(&mut self) -> ast::ParseResult<ReplLine> {
		Ok(match self.peek() {
			None => todo!(),
			Some(Token::Var) => {
				self.next();
				
				let name = self.expect_ident()
					.ok_or("variable name after `var`")?;
				
				self.expect(Token::Assign)
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
			/*
			Some(Token::Ident) if self.slice() == "show" => {
				self.next();
				
				let name = self.expect_ident()
					.ok_or("variable name after `show`")?;
				
				ReplLine::Show(name)
			}
			*/
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

impl ReplLine {
	fn eval(self, t: &mut StackFrame, m: &mut Module) -> EvalResult<Value> {
		match self {
			ReplLine::Show(lval) => {
				println!(": {}", t.get(&lval)?);
			}
			
			ReplLine::Var(name, expr) => {
				let val = expr.eval(t)?;
				t.push(name, val);
			}
			
			ReplLine::Drop(name) => {
				t.remove(&name)?;
			}
			// TODO return Err for item and stmt when not enough input.
			ReplLine::Item(item) => {
				m.insert(item);
			}
			ReplLine::Stmt(stmt) => {
				stmt.eval(t, m)?;
			}
		}
		Ok(Value::Nil)
	}
}

impl From<Item> for ReplLine {
	fn from(item: Item) -> Self { ReplLine::Item(item) }
}

impl From<Stmt> for ReplLine {
	fn from(stmt: Stmt) -> Self { ReplLine::Stmt(stmt) }
}

enum Error {
	SymbolNotFound,
}
