use super::*;

#[derive(Debug, Clone)]
pub struct Param {
	pub name: String,
	pub mutable: bool,
	pub typ: (Type, Span),
}

#[derive(Debug, Clone)]
pub struct Procedure {
	/// Name of the procedure.
	pub name: String,
	/// List of parameters for the procedure.
	pub params: Vec<(Param, Span)>,
	/// Sequence of statements that define the procedure.
	pub code: Vec<(Statement, Span)>,
}

impl Parser {
	pub fn parse_proc(&mut self) -> ParseResult<Procedure> {
		let (_, start) = self.expect(&Token::Proc)
			.ok_or("`proc`")?;
		
		let proc_name = self.expect_ident()
			.ok_or("procedure name")?;
		
		let mut params = Vec::new();
		
		// parse parameter list
		// starting '('
		if self.expect(&Token::LParen).is_some() {
			loop {
				// TODO add case for newline for multiline param declaration?
				match self.peek() {
					// ending ')'
					Some(Token::RParen) => break,
					
					// parse as parameter
					Some(_) => {
						let var = self.expect(&Token::Var)
							.map(|(_, span)| span);
						
						let (param_name, param_start) = self.expect_ident_span()
							.ok_or("parameter name in procedure declaration")?;
						
						let (mutable, start) = match var {
							Some(span) => (true,  span),
							None       => (false, param_start),
						};
						
						self.expect(&Token::Colon)
							.ok_or("`:` after parameter name")?;
						
						// get type
						let typ = self.parse_type()?;
						
						for (Param { name, .. }, _) in &params {
							if *name == param_name {
								eprintln!(
									"Some parameter names in `proc {}` overlap: {:?}",
									proc_name, name
								);
								Err("parameter names to be unique")?;
							}
						}
						
						let span = start.merge(&typ.1);
						params.push((
							Param { mutable, name: param_name, typ },
							span
						));
						
						match self.peek() {
							Some(Token::Comma) => { self.next(); }
							Some(Token::RParen) => {}
							_ => Err("`,` or `)` in parameter list")?
						}
					}
					
					None => Err("`,` or `)` in parameter list")?,
				}
			}
			self.next();
		}
		
		self.expect(&Token::Newline)
			.ok_or("newline after parameter list")?;
		
		// code block section
		let mut code = Vec::new();
		loop {
			match self.peek() {
				Some(Token::End) => break,
				Some(_) => code.push(self.parse_stmt()?),
				None => Err("a statement or `end`")?,
			}
		};
		let (_, end) = self.next().unwrap();
		
		self.expect(&Token::Newline);
		
		Ok((Procedure { name: proc_name, params, code }, start.merge(&end)))
	}
}
