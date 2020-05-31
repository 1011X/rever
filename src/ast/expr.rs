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
	Not(Box<Expr>),
	
	// binary op, precendeces 4-7
	BinOp(BinOp, Box<Expr>, Box<Expr>),
	
	// secret precendece 8
	If(Box<Expr>, Box<Expr>, Box<Expr>),
	
	// secret precedence 9
	Let(String, Option<Type>, Box<Expr>, Box<Expr>),
}

impl Parse for Expr {
	fn parse(tokens: &mut Tokens) -> ParseResult<Self> {
		if tokens.peek() == Some(&Token::If) {
			tokens.next();
			
			let test = Box::new(Expr::parse_rel(tokens)?);
			
			// ensure there's a newline afterwards
			tokens.expect(&Token::Then)
				.ok_or("`then` after `if` predicate")?;
			
			// parse the main block
			let main_expr = Box::new(Expr::parse(tokens)?);
			
			// check for `else`
			tokens.expect(&Token::Else)
				.ok_or("`else` in `if` expression")?;
			
			// parse else section
			let else_block = Box::new(Expr::parse(tokens)?);
			
			Ok(Expr::If(test, main_expr, else_block))
		} else if tokens.peek() == Some(&Token::Let) {
			tokens.next();
			
			let name = match tokens.next() {
				Some(Token::Ident(name)) => name,
				_ => return Err("name for let-binding")
			};
			
			// get optional type as `: type`
			let typ = if tokens.peek() == Some(&Token::Colon) {
				tokens.next();
				Some(Type::parse(tokens)?)
			} else {
				None
			};
			
			// expect '='
			tokens.expect(&Token::Eq)
				.ok_or("`=` after let-binding name")?;
			
			// val is artificially limited here on purpose. it doesn't make much
			// sence to allow `let` inside a `let` binding value, for example.
			let val = Expr::parse_rel(tokens)?;
			
			// check for newline
			tokens.expect(&Token::Newline)
				.ok_or("`in` after `let` binding")?;
			
			let scope = Expr::parse(tokens)?;
			
			Ok(Expr::Let(name, typ, Box::new(val), Box::new(scope)))
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
			Expr::BinOp(op, Box::new(acc), Box::new(base))
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
			Expr::BinOp(op, Box::new(acc), Box::new(base))
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
			Expr::BinOp(op, Box::new(acc), Box::new(base))
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
			Expr::BinOp(BinOp::Exp, Box::new(base), Box::new(acc))
		);
		
		Ok(Expr::BinOp(BinOp::Exp, Box::new(first), Box::new(res)))
	}
	
	fn parse_atom(tokens: &mut Tokens) -> ParseResult<Self> {
		// check if there's an open parenthesis
		if tokens.peek() == Some(&Token::LParen) {
			tokens.next();
			
			let expr = Expr::parse(tokens)?;
			
			// make sure there's a closing parenthesis
			if tokens.next() != Some(Token::RParen) {
				return Err("`)` after subexpression");
			}
			
			Ok(Expr::Group(Box::new(expr)))
		} else {
			// otherwise, treat it as a Term.
			Ok(Expr::Term(Term::parse(tokens)?))
		}
	}
	
	pub fn get_type(&self) -> Option<Type> {
		match self {
			Expr::Cast(_, t) => Some(t.clone()),
			_ => unimplemented!()
		}
	}
}

impl From<Term> for Expr {
	fn from(f: Term) -> Self { Expr::Term(f) }
}
