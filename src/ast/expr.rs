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
	Add, Sub, Or, Xor,
	// precedence 7
	Eq, Ne, Lt, Gt, Le, Ge,
}

#[derive(Debug, Clone)]
pub enum Expr {
	// precedence 1
	Term(Term),
	Group(Box<Expr>),
	Cast(Box<Expr>, Type),
	
	// precedence 3
	Neg(Box<Expr>),
	Not(Box<Expr>),
	
	// binary op, precendeces 4-7
	BinOp(Box<Expr>, BinOp, Box<Expr>),
	
	// secret precendece 8
	If(Box<Expr>, Box<Expr>, Box<Expr>),
	
	// secret precedence 9
	Let(String, Option<Type>, Box<Expr>, Box<Expr>),
}

impl Expr {
	pub fn get_type(&self) -> Option<Type> {
		match self {
			Expr::Cast(_, t) => Some(t.clone()),
			_ => None // TODO
		}
	}
}

impl Parse for Expr {
	fn parse(tokens: &mut Tokens) -> ParseResult<Self> {
		if tokens.starts_with(&Token::If) {
			tokens.next();
			
			let test = Box::new(Expr::parse_rel(tokens)?);
			
			// check for `then`
			tokens.expect(&Token::Then)
				.ok_or("`then` after `if` predicate")?;
			
			// parse the main block
			let main_expr = Box::new(Expr::parse(tokens)?);
			
			// check for `else`
			tokens.expect(&Token::Else)
				.ok_or("`else` in `if` expression")?;
			
			// parse else section
			let else_block = Box::new(Expr::parse(tokens)?);
			
			// check for `fi`
			tokens.expect(&Token::Fi)
				.ok_or("`fi` in `if` expression")?;
			
			Ok(Expr::If(test, main_expr, else_block))
		} else if tokens.starts_with(&Token::Let) {
			tokens.next();
			
			let name = tokens.expect_ident()
				.ok_or("variable name for let binding")?;
			
			// get optional type as `: type`
			let typ =
				if tokens.starts_with(&Token::Colon) {
					tokens.next();
					Some(Type::parse(tokens)?)
				} else {
					None
				};
			
			// expect '='
			tokens.expect(&Token::Eq)
				.ok_or("`=` at let binding")?;
			
			// val is artificially limited here on purpose. it doesn't make much
			// sence to allow `let` inside a `let` binding value, for example.
			let val = Box::new(Expr::parse_rel(tokens)?);
			
			// check for newline
			tokens.expect(&Token::Newline)
				.ok_or("newline at let binding")?;
			
			let scope = Box::new(Expr::parse(tokens)?);
			
			Ok(Expr::Let(name, typ, val, scope))
		} else {
			Expr::parse_rel(tokens)
		}
	}
}

// rel  -> expr {(=|≠|<|>|≤|≥|in) expr}
// expr -> term {(+|-|or) term}
// term -> exp {(*|/|mod|and) exp}
// exp  -> atom {^ atom}
// atom -> ( expr )
//      -> expr 'as' type
//      -> factor
impl Expr {
	fn parse_rel(tokens: &mut Tokens) -> ParseResult<Self> {
		// <term>
		let first = Expr::parse_expr(tokens)?;
		let mut exprs: Vec<(BinOp, Expr)> = Vec::new();
		
		// { ('=' | '!=' | '<' | '>' | '<=' | '>=') <expr> }
		loop {
			let op = match tokens.peek() {
				Some(Token::Eq)  => BinOp::Eq,
				Some(Token::Neq) => BinOp::Ne,
				Some(Token::Lt)  => BinOp::Lt,
				Some(Token::Gt)  => BinOp::Gt,
				Some(Token::Lte) => BinOp::Le,
				Some(Token::Gte) => BinOp::Ge,
				//Some(Token::In) => BinOp::In,
			    _ => break
			};
			tokens.next();
		    
		    let expr = Expr::parse_expr(tokens)?;
		    exprs.push((op, expr));
		}
		
		if exprs.is_empty() {
		    return Ok(first);
		}
		
		Ok(exprs.into_iter().fold(first, |acc, (op, base)|
			Expr::BinOp(Box::new(acc), op, Box::new(base))
		))
	}
	
	pub fn parse_expr(tokens: &mut Tokens) -> ParseResult<Self> {
		// <term>
		let first = Expr::parse_term(tokens)?;
		let mut terms: Vec<(BinOp, Expr)> = Vec::new();
		
		// { ('+' | '-' | 'or' | ':') <term> }
		loop {
			let op = match tokens.peek() {
				Some(Token::Plus)  => BinOp::Add,
				Some(Token::Minus) => BinOp::Sub,
				Some(Token::Or)    => BinOp::Or,
				Some(Token::Colon) => BinOp::Xor,
			    _ => break
			};
			tokens.next();
		    
		    let term = Expr::parse_term(tokens)?;
		    terms.push((op, term));
		}
		
		if terms.is_empty() {
		    return Ok(first);
		}
		
		Ok(terms.into_iter().fold(first, |acc, (op, base)|
			Expr::BinOp(Box::new(acc), op, Box::new(base))
		))
	}
	
	fn parse_term(tokens: &mut Tokens) -> ParseResult<Self> {
		// <fact>
		let first = Expr::parse_exp(tokens)?;
		let mut facts: Vec<(BinOp, Expr)> = Vec::new();
		
		// { ('*' | '/' | 'mod' | 'and') <fact> }
		loop {
			let op = match tokens.peek() {
				Some(Token::Star)   => BinOp::Mul,
				Some(Token::FSlash) => BinOp::Div,
				Some(Token::Mod)    => BinOp::Mod,
				Some(Token::And)    => BinOp::And,
			    _ => break
			};
			tokens.next();
		    
		    let fact = Expr::parse_exp(tokens)?;
		    facts.push((op, fact));
		}
		
		if facts.is_empty() {
		    return Ok(first);
		}
		
		Ok(facts.into_iter().fold(first, |acc, (op, base)|
			Expr::BinOp(Box::new(acc), op, Box::new(base))
		))
	}
	
	fn parse_exp(tokens: &mut Tokens) -> ParseResult<Self> {
		// <exp>
		let first = Expr::parse_atom(tokens)?;
		let mut exps = Vec::new();
		
		// { ('^') <exp> }
		loop {
			let op = match tokens.peek() {
				Some(Token::Caret) => {}
			    _ => break
			};
			tokens.next();
		    
		    let exp = Expr::parse_atom(tokens)?;
		    exps.push(exp);
		}
		
		if exps.is_empty() {
		    return Ok(first);
		}
		
		let last = exps.pop().unwrap();
		let res = exps.into_iter().rfold(last, |acc, base|
			Expr::BinOp(Box::new(base), BinOp::Exp, Box::new(acc))
		);
		
		Ok(Expr::BinOp(Box::new(first), BinOp::Exp, Box::new(res)))
	}
	
	fn parse_atom(tokens: &mut Tokens) -> ParseResult<Self> {
		// check if there's an open parenthesis
		let expr =
			if tokens.peek() == Some(&Token::LParen) {
				tokens.next();
				
				let expr = Expr::parse(tokens)?;
				
				// make sure there's a closing parenthesis
				tokens.expect(&Token::RParen)
					.ok_or("`)` after subexpression")?;
				
				Expr::Group(Box::new(expr))
			} else {
				// otherwise, treat it as a Term.
				Expr::Term(Term::parse(tokens)?)
			};
		
		if tokens.peek() == Some(&Token::As) {
			Ok(Expr::Cast(Box::new(expr), Type::parse(tokens)?))
		} else {
			Ok(expr)
		}
	}
}

impl From<Term> for Expr {
	fn from(f: Term) -> Self { Expr::Term(f) }
}
