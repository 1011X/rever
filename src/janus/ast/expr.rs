use super::*;

#[derive(Debug)]
pub enum Expr {
	Factor(Factor),
	Size(LValue),
	Top(LValue),
	
	Mul(Box<Expr>, Box<Expr>),
	Div(Box<Expr>, Box<Expr>),
	Mod(Box<Expr>, Box<Expr>),
	
	Add(Box<Expr>, Box<Expr>),
	Sub(Box<Expr>, Box<Expr>),
	
	BitXor(Box<Expr>, Box<Expr>),
	BitAnd(Box<Expr>, Box<Expr>),
	BitOr(Box<Expr>, Box<Expr>),
}

impl Expr {
	named!(pub parse<Self>, sp!(do_parse!(
		leaf: call!(Expr::leaf) >>
		prods: many0!(Expr::product) >>
		sums: many0!(Expr::sum) >>
		bitops: many0!(Expr::bitop)
		
		>> (leaf.to_product(prods).to_sum(sums).to_bitop(bitops))
	)));
	
	named!(leaf<Self>, sp!(alt_complete!(
		do_parse!(
			op: alt!(tag!("size") | tag!("top")) >>
			tag!("(") >>
			lval: call!(LValue::parse) >>
			tag!(")")
			>> (match op {
				b"size" => Expr::Size(lval),
				b"top"  => Expr::Top(lval),
				_ => unreachable!()
			})
		)
		| delimited!(
			tag!("("),
			call!(Expr::parse),
			tag!(")")
		)
		| map!(Factor::parse, Expr::Factor)
	)));
	
	named!(product<(&[u8], Expr)>, sp!(do_parse!(
		op: alt!(tag!("*") | tag!("/") | tag!("%")) >>
		leaf: call!(Expr::leaf)
		>> (op, leaf)
	)));
	
	named!(sum<(&[u8], Expr)>, sp!(do_parse!(
		op: alt!(tag!("+") | tag!("-")) >>
		leaf: call!(Expr::leaf) >>
		prods: many0!(Expr::product)
		
		>> (op, leaf.to_product(prods))
	)));
	
	named!(bitop<(&[u8], Expr)>, sp!(do_parse!(
		op: alt!(tag!("&") | tag!("|")) >>
		leaf: call!(Expr::leaf) >>
		prods: many0!(Expr::product) >>
		sums: many0!(Expr::sum)
		
		>> (op, leaf.to_product(prods).to_sum(sums))
	)));
	
	fn to_product(self, mut prods: Vec<(&[u8], Expr)>) -> Expr {
		// no extra operations are done, so just return self
		if prods.is_empty() {
			return self;
		}
		
		// at least one operation; extract the last one.
		let (mut curr_op, last) = prods.pop().unwrap();
		// reverse, then fold using last element to build a
		// right-spanning AST
		let expr = prods.into_iter()
			.rev()
			.fold(last, |acc, (op, e)| {
				let res = match curr_op {
					b"*" => Expr::Mul(Box::new(e), Box::new(acc)),
					b"/" => Expr::Div(Box::new(e), Box::new(acc)),
					b"%" => Expr::Mod(Box::new(e), Box::new(acc)),
					_ => unreachable!()
				};
				// change curr_op for next element
				curr_op = op;
				res
			});
		
		match curr_op {
			b"*" => Expr::Mul(Box::new(self), Box::new(expr)),
			b"/" => Expr::Div(Box::new(self), Box::new(expr)),
			b"%" => Expr::Mod(Box::new(self), Box::new(expr)),
			_ => unreachable!()
		}
	}
	
	fn to_sum(self, mut sums: Vec<(&[u8], Expr)>) -> Expr {
		// same as `.to_product()`
		if sums.is_empty() {
			return self;
		}
		
		let (mut curr_op, last) = sums.pop().unwrap();
		let expr = sums.into_iter()
			.rev()
			.fold(last, |acc, (op, e)| {
				let res = match curr_op {
					b"+" => Expr::Add(Box::new(e), Box::new(acc)),
					b"-" => Expr::Sub(Box::new(e), Box::new(acc)),
					_ => unreachable!()
				};
				curr_op = op;
				res
			});
		
		match curr_op {
			b"+" => Expr::Add(Box::new(self), Box::new(expr)),
			b"-" => Expr::Sub(Box::new(self), Box::new(expr)),
			_ => unreachable!()
		}
	}
	
	fn to_bitop(self, mut bitops: Vec<(&[u8], Expr)>) -> Expr {
		// same as `.to_product()` and `.to_sum()`
		if bitops.is_empty() {
			return self;
		}
		
		let (mut curr_op, last) = bitops.pop().unwrap();
		let expr = bitops.into_iter()
			.rev()
			.fold(last, |acc, (op, e)| {
				let res = match curr_op {
					b"&" => Expr::BitAnd(Box::new(e), Box::new(acc)),
					b"|" => Expr::BitOr(Box::new(e), Box::new(acc)),
					b"^" => Expr::BitXor(Box::new(e), Box::new(acc)),
					_ => unreachable!()
				};
				curr_op = op;
				res
			});
		
		match curr_op {
			b"&" => Expr::BitAnd(Box::new(self), Box::new(expr)),
			b"|" => Expr::BitOr(Box::new(self), Box::new(expr)),
			b"^" => Expr::BitXor(Box::new(self), Box::new(expr)),
			_ => unreachable!()
		}
	}
	/*
	pub fn eval(&self, symtab: &SymTab) -> Result<i16, String> {
		match *self {
			Expr::Factor(ref fac) => {
				if let fac.eval(symtab)
			}
			
			Expr::Size(ref lval) => {
				if let Value::Array(ref v) = symtab[&lval.name] {
					Ok(v.len() as i16)
				} else {
					Err(format!("`{}` is not an array", lval.name))
				}
			}
			
			//Expr::Top(ref lval) => 
			_ => unimplemented!()
		}
		
		
	Factor(Factor),
	Size(LValue),
	Top(LValue),
	
	Mul(Box<Expr>, Box<Expr>),
	Div(Box<Expr>, Box<Expr>),
	Mod(Box<Expr>, Box<Expr>),
	
	Add(Box<Expr>, Box<Expr>),
	Sub(Box<Expr>, Box<Expr>),
	
	BitXor(Box<Expr>, Box<Expr>),
	BitAnd(Box<Expr>, Box<Expr>),
	BitOr(Box<Expr>, Box<Expr>),
	}
	*/
}

