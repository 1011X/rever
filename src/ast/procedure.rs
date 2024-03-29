use super::*;

use crate::interpret::EvalResult;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir { Fore, Back }

#[derive(Debug, Clone)]
pub struct Param {
	pub name: String,
	pub constant: bool,
	pub typ: Type,
}

#[derive(Clone)]
pub enum ProcDef {
	/// Sequence of statements defining a user-provided procedure.
	User(Vec<Stmt>),
	/// Pair of irreversible functions defining an internal reversible 
	/// procedure.
	Internal {
		fore: fn(&mut [Value]) -> EvalResult<()>,
		back: fn(&mut [Value]) -> EvalResult<()>,
	},
	External,
}

use std::fmt;
impl fmt::Debug for ProcDef {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ProcDef::Internal { .. } => fmt.write_str("<internal proc>"),
			ProcDef::External => fmt.write_str("<external proc>"),
			ProcDef::User(stmts) => stmts.fmt(fmt),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Procedure {
	pub name: String,
	pub params: Vec<Param>,
	/// How and where a procedure is defined.
	pub code: ProcDef,
}

impl Parser<'_> {
	pub fn parse_proc(&mut self) -> ParseResult<Procedure> {
		self.expect(Token::Proc).ok_or("`proc`")?;
		
		let proc_name = match self.peek() {
			Some(Token::VarIdent) => self.slice().to_string(),
			_ => Err("procedure name")?,
		};
		self.next();
		
		let mut params = Vec::new();
		
		// parse parameter list
		// starting '{'
		if self.peek() == Some(&Token::LBrace) {
			self.next();
			
			// TODO add case for newline for multiline param declaration
			loop {
				match self.peek() {
					// ending '}'
					Some(Token::RBrace) => break,
					
					// parse as parameter
					Some(_) => {
						// whether parameter is `const`
						let constant = self.expect(Token::Const).is_some();
						
						// parameter name
						let param_name = match self.peek() {
							Some(Token::VarIdent) => self.slice().to_string(),
							_ => Err("parameter name in procedure declaration")?,
						};
						self.next();
						
						self.expect(Token::Colon)
							.ok_or("`:` after parameter name")?;
						
						// parameter's type
						let typ = self.parse_type()?;
						
						params.push(Param { constant, name: param_name, typ });
						
						match self.peek() {
							Some(Token::Comma) => { self.next(); }
							Some(Token::RBrace) => {}
							_ => Err("`,` or `}` in parameter list")?
						}
					}
					
					None => Err("`,` or `}` in parameter list")?,
				}
			}
			self.next();
		}
		
		self.expect(Token::Newline)
			.ok_or("newline after procedure declaration")?;
		
		self.skip_newlines();
		
		// code block section
		let mut code = Vec::new();
		loop {
			match self.peek() {
				Some(Token::Return) => break,
				Some(_) => code.push(self.parse_stmt()?),
				None => Err("a statement or `end`")?,
			}
		}
		self.next();
		
		Ok(Procedure {
			name: proc_name,
			params,
			code: ProcDef::User(code),
		})
	}
}


use crate::interpret::{Context, StackFrame};

impl Procedure {
	fn call_base(&self, items: Context, dir: Dir, args: Vec<Value>) -> EvalResult<Vec<Value>> {
		// verify number of arguments and their types
		assert_eq!(args.len(), self.params.len(),
			"wrong number of parameters before calling proc {}", self.name
		);
		for (arg, param) in args.iter().zip(&self.params) {
			assert_eq!(arg.get_type(), param.typ,
				"value with wrong type passed before calling proc {}", self.name
			);
		}
		
		// make stack frame with parameter names bound to argument values
		let mut ctx = StackFrame::new(
			items,
			self.params.iter()
				.map(|param| param.name.clone())
				.zip(args.clone())
				.collect(),
		);
		
		// execute the actual code
		match (dir, &self.code) {
			(Dir::Fore, ProcDef::User(code)) => {
				for stmt in code {
					stmt.eval(&mut ctx)?;
				}
			}
			(Dir::Back, ProcDef::User(code)) => {
				for stmt in code {
					stmt.clone().invert().eval(&mut ctx)?;
				}
			}
			(Dir::Fore, ProcDef::Internal { fore, .. }) => {
				fore(ctx.values())?;
			}
			(Dir::Back, ProcDef::Internal { back, .. }) => {
				back(ctx.values())?;
			}
			_ => todo!()
		}
		
		let args = ctx.into_inner();
		
		// verify number of arguments and their types again
		assert_eq!(args.len(), self.params.len(),
			"wrong number of parameters after calling proc {}", self.name
		);
		for (arg, param) in args.iter().zip(&self.params) {
			assert_eq!(arg.get_type(), param.typ,
				"value with wrong type received after calling proc {}", self.name
			);
		}
		
		Ok(args)
	}
	
	pub fn call(&self, items: Context, args: Vec<Value>) -> EvalResult<Vec<Value>> {
		self.call_base(items, Dir::Fore, args)
	}
	
	pub fn uncall(&self, items: Context, args: Vec<Value>) -> EvalResult<Vec<Value>> {
		self.call_base(items, Dir::Back, args)
	}
}
