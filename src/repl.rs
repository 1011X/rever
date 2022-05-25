use std::io::{self, prelude::*};
use logos::Logos;

use crate::token::Token;
use crate::ast::{self, LValue, Expr, Item, Module, Procedure, Param, Stmt, Type};
use crate::interpret::{Eval, EvalResult, Stack, StackFrame, Value};

pub fn init() -> io::Result<()> {
	let stdin = io::stdin();
	let mut input = String::new();
	let mut stdout = io::stdout();
	let mut continuing = false;
	
	let mut module = Module::new("repl".into(), Vec::new());
	module.insert(Item::Proc(Procedure {
		name: "print".into(),
		params: vec![
			Param {
				name: "msg".into(),
				constant: false,
				typ: Type::String,
			},
			Param {
				name: "bytes_read".into(),
				constant: false,
				typ: Type::U32,
			},
		],
		code: crate::interpret::intrinsic::PRINT_PROCDEF.clone(),
	}));
	
	let mut stack = Stack::new();
	let root_frame = StackFrame::new(Vec::new());
	stack.push(root_frame);
	
	//println!("Rever 0.0.1");
	
	loop {
		let prompt = if continuing { "|" } else { "<" };
		print!("{} ", prompt);
		stdout.flush()?;
		stdin.read_line(&mut input)?;
		
		//println!("{:?}", input);
		
		// read
		let tokens = Token::lexer(&input);
		let mut parser = ast::Parser::new(tokens);
		
		let line = match parser.parse_repl_line() {
			Ok(line) => line,
			Err(_) if parser.peek() == None => {
				continuing = true;
				continue;
			}
			Err(e) => {
				eprintln!("! Invalid input: {}, got {:?}.", e, parser.peek());
				eprintln!("! Remaining input: {:?}", parser.remainder());
				input.clear();
				continuing = false;
				continue;
			}
		};
		
		// eval
		match line.eval(stack.last_mut().unwrap(), &mut module) {
			Ok(Value::Nil) => {}
			Ok(value) => {
				println!("{}", value);
			}
			Err(e) => {
				eprintln!("! Error: {}.", e);
			}
		}
		
		input.clear();
		continuing = false;
	}
}

#[derive(Debug, Clone)]
pub enum ReplLine {
	/// For blank or empty input, or input with only comments
	Blank,
	
	//Show(LValue),
	
	Var(String, Type, Expr),
	Drop(String),
	
	Item(Item),
	Stmt(Stmt),
	Expr(Expr),
}

impl ast::Parser<'_> {
	pub fn parse_repl_line(&mut self) -> ast::ParseResult<ReplLine> {
		Ok(match self.peek() {
			None => todo!(),
			
			Some(Token::Newline) => ReplLine::Blank,
			/*
			Some(Token::Ident) if self.slice() == "show" => {
				self.next();
				
				let name = match self.peek() {
					Some(Token::VarIdent) => self.slice().to_string(),
					_ => Err("variable name after `show`")?,
				};
				
				ReplLine::Show(LValue { id: name, ops: Vec::new() })
			}
			*/
			Some(Token::Fn | Token::Proc | Token::Mod) =>
				self.parse_item()?.into(),
			
			Some(Token::Var) => {
				self.next();
				
				// get name
				let name = match self.peek() {
					Some(Token::VarIdent) => self.slice().to_string(),
					_ => Err("name in variable declaration")?,
				};
				self.next();
				
				// get optional type
				let typ = match self.expect(Token::Colon) {
					Some(_) => self.parse_type()?,
					None => Type::Infer,
				};
				
				// check for assignment op
				self.expect(Token::Assign)
					.ok_or("`:=` in variable declaration")?;
				
				// get initialization expression
				let init = self.parse_expr()?;
				/*
				self.expect(Token::Newline)
					.ok_or("newline after variable declaration")?;
				*/
				ReplLine::Var(name, typ, init)
			}
			
			Some(Token::Drop) => {
				self.next();
				
				// get name
				let name = match self.peek() {
					Some(Token::VarIdent) => self.slice().to_string(),
					_ => Err("name in variable declaration")?,
				};
				self.next();
				/*
				self.expect(Token::Newline)
					.ok_or("newline after variable declaration")?;
				*/
				ReplLine::Drop(name)
			}
				
			Some(_) => {
				let mut checkpoint = self.clone();
				match self.parse_stmt() {
					Ok(stmt) => stmt.into(),
					Err(_) => {
						let expr = checkpoint.parse_expr()?.into();
						self.expect(Token::Newline);
						expr
					}
				}
			}
		})
	}
}

impl ReplLine {
	fn eval(self, t: &mut StackFrame, m: &mut Module) -> EvalResult<Value> {
		match self {
			ReplLine::Blank => Ok(Value::Nil),
			/*
			ReplLine::Show(lval) => {
				println!(": {}", t.get(&lval)?);
				Ok(Value::Nil)
			}
			*/
			ReplLine::Var(name, _, expr) => {
				let val = expr.eval(t)?;
				t.push(name, val);
				Ok(Value::Nil)
			}
			
			ReplLine::Drop(name) => {
				Ok(t.remove(&name)?)
			}
			
			// TODO return Err for item and stmt when not enough input.
			ReplLine::Item(item) => {
				m.insert(item);
				Ok(Value::Nil)
			}
			ReplLine::Stmt(stmt) => {
				stmt.eval(t, m)?;
				Ok(Value::Nil)
			}
			ReplLine::Expr(expr) => {
				expr.eval(t)
			}
		}
	}
}

impl From<Item> for ReplLine {
	fn from(item: Item) -> Self { ReplLine::Item(item) }
}

impl From<Stmt> for ReplLine {
	fn from(stmt: Stmt) -> Self { ReplLine::Stmt(stmt) }
}

impl From<Expr> for ReplLine {
	fn from(expr: Expr) -> Self { ReplLine::Expr(expr) }
}

enum Error {
	SymbolNotFound,
}
