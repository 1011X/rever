/*!
Expressions in Rever have 5 levels of precendence. From strongest to weakest:
1. Parentheses
2. Function calls
3. Unary operators: not - (maybe: ! ~)
4. Exponential operators: ^ << >> shl shr rol ror
5. Multiplicative operators: * / mod as and
6. Additive operators: + - or xor
7. Relational operators: = != ≠ < > <= ≤ >= ≥ in

Ideas:
+ Chained relations, a la Python?
+ In `if` statements, conjunctions is `,` and disjunction is `;` (from Prolog).
  + No short-circuiting; like Pascal.
  + Short-circuiting can be achieved using `and` and `or`.

TODO:
+ Add precedences 2, 
*/

use super::*;

#[derive(Debug, Clone)]
pub enum BinOp {
	// precedence 4
	Exp,
	// precedence 5
	Mul, Div, Mod, And,
	// precedence 6
	Add, Sub, Or, //Xor,
	// precedence 7
	Eq, Ne, Lt, Gt, Le, Ge,
}

#[derive(Debug, Clone)]
pub enum Expr {
	// precedence 1
	Term(Term),
	Cast(Box<(Expr, Span)>, (Type, Span)),
	
	// precedence 3
	Neg(Box<(Expr, Span)>),
	Not(Box<(Expr, Span)>),
	
	// binary op, precendeces 4-7
	BinOp(Box<(Expr, Span)>, BinOp, Box<(Expr, Span)>),
	
	// secret precendece 8
	If(Box<(Expr, Span)>, Box<(Expr, Span)>, Box<(Expr, Span)>),
	
	// secret precedence 9
	Let(String, Option<(Type, Span)>, Box<(Expr, Span)>, Box<(Expr, Span)>),
}

impl Expr {
	pub fn get_type(&self) -> Option<Type> {
		match self {
			Expr::Cast(_, t) => Some(t.0.clone()),
			_ => None // TODO
		}
	}
}

impl Parser {
	pub fn parse_expr(&mut self) -> ParseResult<Expr> {
		match self.peek() {
			Some(Token::If) => {
				let (_, start) = self.next().unwrap();
				let test = Box::new(self.parse_expr()?);
				
				// check for `then`
				self.expect(&Token::Then)
					.ok_or("`then` after `if` predicate")?;
				
				// parse the main block
				let main_expr = Box::new(self.parse_expr()?);
				
				// check for `else`
				self.expect(&Token::Else)
					.ok_or("`else` in `if` expression")?;
				
				let else_block = Box::new(self.parse_expr()?);
				
				let (_, end) = self.expect(&Token::Fi)
					.ok_or("`fi` in `if` expression")?;
				
				let span = start.merge(&end);
				
				Ok((Expr::If(test, main_expr, else_block), span))
			}
			Some(Token::Let) => {
				let (_, start) = self.next().unwrap();
				
				let name = self.expect_ident()
					.ok_or("variable name for let binding")?;
				
				// get optional `: <type>`
				let typ = match self.expect(&Token::Colon) {
					Some(_) => Some(self.parse_type()?),
					None => None,
				};
				
				// expect '='
				self.expect(&Token::Eq)
					.ok_or("`=` at let binding")?;
				
				// val is artificially limited here on purpose. it doesn't make much
				// sense to allow `let` inside a `let` binding value, for example.
				let val = Box::new(self.parse_expr_rel()?);
				
				self.expect(&Token::Newline)
					.ok_or("newline at let binding")?;
				
				let scope = self.parse_expr()?;
				let span = start.merge(&scope.1);
				
				Ok((Expr::Let(name, typ, val, Box::new(scope)), span))
			}
			Some(_) =>
				self.parse_expr_rel(),
			None =>
				todo!()
		}
	}
	
	// rel  -> expr {(=|≠|<|>|≤|≥|in) expr}
	// expr -> term {(+|-|or) term}
	// term -> exp {(*|/|mod|and) exp}
	// exp  -> atom {^ atom}
	// atom -> ( expr )
	//      -> expr 'as' type
	//      -> factor
	pub fn parse_expr_rel(&mut self) -> ParseResult<Expr> {
		// <term>
		let first = self.parse_expr_add()?;
		let mut exprs: Vec<(BinOp, (Expr, Span))> = Vec::new();
		
		// { ('=' | '!=' | '<' | '>' | '<=' | '>=') <expr> }
		loop {
			let op = match self.peek() {
				Some(Token::Eq)  => BinOp::Eq,
				Some(Token::Neq) => BinOp::Ne,
				Some(Token::Lt)  => BinOp::Lt,
				Some(Token::Gt)  => BinOp::Gt,
				Some(Token::Lte) => BinOp::Le,
				Some(Token::Gte) => BinOp::Ge,
				//Some(Token::In) => BinOp::In,
				_ => break
			};
			self.next();
			
			let expr = self.parse_expr_add()?;
			exprs.push((op, expr));
		}
		
		if exprs.is_empty() {
			return Ok(first);
		}
		
		let (expr, span) = exprs.into_iter()
			.fold(first, |acc, (op, base)| {
				let span = acc.1.merge(&base.1);
				(Expr::BinOp(Box::new(acc), op, Box::new(base)), span)
			});
		
		Ok((expr, span))
	}
	
	pub fn parse_expr_add(&mut self) -> ParseResult<Expr> {
		// <term>
		let first = self.parse_expr_mul()?;
		let mut terms: Vec<(BinOp, (Expr, Span))> = Vec::new();
		
		// { ('+' | '-' | 'or' | ':') <term> }
		loop {
			let op = match self.peek() {
				Some(Token::Plus)  => BinOp::Add,
				Some(Token::Minus) => BinOp::Sub,
				Some(Token::Or)    => BinOp::Or,
				//Some(Token::Colon) => BinOp::Xor,
				_ => break
			};
			self.next();
			
			let term = self.parse_expr_mul()?;
			terms.push((op, term));
		}
		
		if terms.is_empty() {
			return Ok(first);
		}
		
		let (expr, span) = terms.into_iter()
			.fold(first, |acc, (op, base)| {
				let span = acc.1.merge(&base.1);
				(Expr::BinOp(Box::new(acc), op, Box::new(base)), span)
			});
		
		Ok((expr, span))
	}
	
	pub fn parse_expr_mul(&mut self) -> ParseResult<Expr> {
		// <fact>
		let first = self.parse_expr_exp()?;
		let mut facts: Vec<(BinOp, (Expr, Span))> = Vec::new();
		
		// { ('*' | '/' | 'mod' | 'and') <fact> }
		loop {
			let op = match self.peek() {
				Some(Token::Star)   => BinOp::Mul,
				Some(Token::FSlash) => BinOp::Div,
				Some(Token::Mod)    => BinOp::Mod,
				Some(Token::And)    => BinOp::And,
				_ => break
			};
			self.next();
			
			let fact = self.parse_expr_exp()?;
			facts.push((op, fact));
		}
		
		if facts.is_empty() {
			return Ok(first);
		}
		
		let (expr, span) = facts.into_iter()
			.fold(first, |acc, (op, base)| {
				let span = acc.1.merge(&base.1);
				(Expr::BinOp(Box::new(acc), op, Box::new(base)), span)
			});
		Ok((expr, span))
	}
	
	pub fn parse_expr_exp(&mut self) -> ParseResult<Expr> {
		// <exp>
		let first = self.parse_expr_atom()?;
		let mut exps = Vec::new();
		
		// { ('^') <exp> }
		loop {
			let op = match self.peek() {
				Some(Token::Caret) => {}
				_ => break
			};
			self.next();
			
			let exp = self.parse_expr_atom()?;
			exps.push(exp);
		}
		
		if exps.is_empty() {
			return Ok(first);
		}
		
		let last = exps.pop().unwrap();
		let res = exps.into_iter().rfold(last, |acc, base| {
			let span = base.1.merge(&acc.1);
			(Expr::BinOp(Box::new(base), BinOp::Exp, Box::new(acc)), span)
		});
		
		let span = first.1.merge(&res.1);
		Ok((Expr::BinOp(Box::new(first), BinOp::Exp, Box::new(res)), span))
	}
	
	pub fn parse_expr_atom(&mut self) -> ParseResult<Expr> {
		// check if there's an open parenthesis
		let expr =
			if self.expect(&Token::LParen).is_some() {
				let (expr, span) = self.parse_expr()?;
				
				// make sure there's a closing parenthesis
				self.expect(&Token::RParen)
					.ok_or("`)` after subexpression")?;
				
				(expr, span)
			} else {
				// otherwise, treat it as a Term.
				let (term, span) = self.parse_term()?;
				(term.into(), span)
			};
		
		Ok(if let Some((_, start)) = self.expect(&Token::As) {
			let typ = self.parse_type()?;
			let span = start.merge(&typ.1);
			(Expr::Cast(Box::new(expr), typ), span)
		} else {
			expr
		})
	}
}

impl From<Term> for Expr {
	fn from(f: Term) -> Self { Expr::Term(f) }
}
