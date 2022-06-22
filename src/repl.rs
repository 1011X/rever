use std::io::{self, prelude::*};
use logos::Logos;

use crate::token::Token;
use crate::ast::{self, LValue, Expr, Item, Module, Procedure, Param, Stmt, Type};
use crate::interpret::{EvalResult, Stack, StackFrame, Context, Value};

pub fn init() -> io::Result<()> {
	let stdin = io::stdin();
	let mut input = String::new();
	let mut stdout = io::stdout();
	let mut continuing = false;
	
	let items = Context {
		procs: vec![
			Procedure {
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
			},
		],
		funcs: vec![],
		mods: vec![],
	};
	
	let mut stack = Stack::new();
	let root_frame = StackFrame::new(items, Vec::new());
	stack.push(root_frame);
	
	//println!("Rever 0.0.1");
	
	loop {
		let prompt = if continuing { "|" } else { "<" };
		print!("{} ", prompt);
		stdout.flush()?;
		stdin.read_line(&mut input)?;
		
		//println!("{:?}", input);
		
		// read
		let mut parser = ast::Parser::new(&input);
		
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
		match line.eval(stack.last_mut().unwrap()) {
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
	
	Var(String, Type, Expr),
	Drop(String, Type, Option<Expr>),
	
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
				
				// get optional type
				let typ = match self.expect(Token::Colon) {
					Some(_) => self.parse_type()?,
					None => Type::Infer,
				};
				
				// check for (optional) assignment op
				let deinit = match self.expect(Token::Assign) {
					Some(_) => Some(self.parse_expr()?),
					None => None,
				};
				
				/*
				self.expect(Token::Newline)
					.ok_or("newline after variable declaration")?;
				*/
				ReplLine::Drop(name, typ, deinit)
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
	fn eval(self, ctx: &mut StackFrame) -> EvalResult<Value> {
		match self {
			ReplLine::Blank => Ok(Value::Nil),
			
			ReplLine::Var(name, _, expr) => {
				let val = expr.eval(ctx)?;
				ctx.push(name, val);
				Ok(Value::Nil)
			}
			
			ReplLine::Drop(name, _, opt_deinit) => {
				match opt_deinit {
					None => Ok(ctx.remove(&name)?),
					Some(expr) => {
						let deinit = expr.eval(ctx)?;
						assert_eq!(deinit, ctx.remove(&name)?);
						Ok(Value::Nil)
					}
				}
			}
			
			ReplLine::Item(item) => {
				ctx.items.insert(item);
				Ok(Value::Nil)
			}
			ReplLine::Stmt(stmt) => {
				stmt.eval(ctx)?;
				Ok(Value::Nil)
			}
			ReplLine::Expr(expr) => {
				expr.eval(ctx)
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
