//use std::fmt;
//use std::error;

use super::*;

#[derive(Debug, Clone)]
pub enum BlockExpr {
	Inline(Expr),
	
	If(Expr, Box<Self>, Box<Self>),
	
	Let(String, Type, Expr, Box<Self>),
}

#[derive(Debug, Clone)]
pub enum BlockExprErr {
	NoNlAfterCond,
	NoElse,
	NoIfOrNlAfterElse,
	NotVarName,
}

impl Parser<'_> {
	pub fn parse_block_expr(&mut self) -> ParseResult<BlockExpr> {
		let block_expr = match self.peek() {
			Some(Token::If) => {
				self.next();
				
				let test = self.parse_expr()?;
				
				self.expect(Token::Newline)
					.ok_or("newline after `if` predicate")?;
					//.ok_or(BlockExprErr::NoNlAfterCond)?;
				
				self.skip_newlines();
				
				// parse main block
				let main_expr = Box::new(self.parse_block_expr()?);
				
				self.expect(Token::Else)
					.ok_or("`else` in `if` expression")?;
					//.ok_or(BlockExprErr::NoElse)?;
				
				match self.peek() {
					Some(Token::If) => {}
					Some(Token::Newline) => {
						self.next();
						self.skip_newlines();
					}
					_ => Err("`if` or newline after `else`")?,
					//_ => Err(BlockExprErr::NoIfOrNlAfterElse)?,
				}
				
				let else_block = Box::new(self.parse_block_expr()?);
				
				BlockExpr::If(test, main_expr, else_block)
			}
			
			Some(Token::Let) => {
				self.next();
				
				let name = match self.peek() {
					Some(Token::VarIdent) => self.slice().to_string(),
					_ => Err("variable name for let binding")?,
					//_ => Err(BlockExprErr::NotVarName)?,
				};
				
				// get optional `: <type>`
				let typ = match self.peek() {
					Some(Token::Colon) => {
						self.next();
						self.parse_type()?
					}
					_ => Type::Infer,
				};
				
				// expect '='
				self.expect(Token::Eq)
					.ok_or("`=` at let binding")?;
					//.ok_or(BlockExprErr::NoEq)?;
				
				let val = self.parse_expr()?;
				
				self.expect(Token::Newline)
					.ok_or("newline at let binding")?;
					//.ok_or(BlockExprErr::NoNlAfterBind)?;
				
				self.skip_newlines();
				
				let scope = Box::new(self.parse_block_expr()?);
				
				BlockExpr::Let(name, typ, val, scope)
			}
			
			_ => BlockExpr::Inline(self.parse_expr()?)
		};
		
		// check for newline(s) but *don't consume them*.
		// final newlines are for statements, not expressions.
		if self.peek() != Some(&Token::Newline) {
			Err("newline after block expression")?;
		}
		
		Ok(block_expr)
	}
}

use crate::interpret::StackFrame;
impl BlockExpr {
	pub fn eval(&self, ctx: &mut StackFrame) -> EvalResult<Value> {
		match self {
			BlockExpr::Inline(expr) => expr.eval(ctx),
			
			BlockExpr::If(test, expr, else_expr) => {
				if test.eval(ctx)? == Value::Bool(true) {
					expr.eval(ctx)
				} else {
					else_expr.eval(ctx)
				}
			}
			
			BlockExpr::Let(name, _, val, scope) => {
				let val = val.eval(ctx)?;
				ctx.push(name.clone(), val);
				let res = scope.eval(ctx);
				ctx.pop();
				res
			}
		}
	}
}
