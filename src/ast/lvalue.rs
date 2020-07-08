use super::*;

#[derive(Debug, Clone)]
pub enum Deref {
	Direct,
	Field(String),
	Index(Expr),
}

#[derive(Debug, Clone)]
pub struct LValue {
	pub id: String,
	pub ops: Vec<(Deref, Span)>,
}

// TODO ponder: is `var name` and `drop name` within statements part of a bigger pattern?
impl Parser {
	pub fn parse_lval(&mut self) -> ParseResult<LValue> {
		let mut ops = Vec::new();
		
		// get lval name
		let (name, start) = self.expect_ident_span()
			.ok_or("variable name in left-value expression")?;
		
		loop {
			match self.peek() {
				// '!'
				Some(Token::Bang) => {
					let (_, span) = self.next().unwrap();
					ops.push((Deref::Direct, span));
				}
				// '.'
				Some(Token::Period) => {
					self.next();
					
					match self.peek() {
						Some(Token::LParen) => {
							self.next();
							
							let (expr, span) = self.parse_expr()?;
							
							self.expect(&Token::RParen)
								.ok_or("`)` after index expression")?;
							
							ops.push((Deref::Index(expr), span));
						}
						Some(Token::Ident(_)) => {
							let (name, span) = self.expect_ident_span().unwrap();
							ops.push((Deref::Field(name), span));
						}
						_ => Err("field name or `(`")?,
					}
				}
				
				_ => break,
			}
		}
		
		let end = ops.last().map(|(_, span)| *span).unwrap_or(start);
		Ok((LValue { id: name, ops }, start.merge(&end)))
	}
}
