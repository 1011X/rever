use super::*;
use super::super::interpret::{Value, SymTab};

#[derive(Debug)]
pub enum Expr {
	Factor(Factor),
	Size(String),
	Top(String),
	
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
		bitops: many0!(Expr::bitop) >>
		prods: many0!(Expr::product) >>
		sums: many0!(Expr::sum)
		
		>> (leaf.to_bitop(bitops).to_product(prods).to_sum(sums))
	)));
	
	named!(leaf<Self>, sp!(alt_complete!(
		do_parse!(
			op: alt!(tag!("size") | tag!("top")) >>
			tag!("(") >>
			id: ident >>
			tag!(")")
			>> (match op {
				b"size" => Expr::Size(id),
				b"top"  => Expr::Top(id),
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
		op: alt!(tag!("&") | tag!("|") | tag!("^")) >>
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
					b"*" => Expr::Mul(Box::new(acc), Box::new(e)),
					b"/" => Expr::Div(Box::new(acc), Box::new(e)),
					b"%" => Expr::Mod(Box::new(acc), Box::new(e)),
					_ => unreachable!()
				};
				// change curr_op for next element
				curr_op = op;
				res
			});
		
		match curr_op {
			b"*" => Expr::Mul(Box::new(expr), Box::new(self)),
			b"/" => Expr::Div(Box::new(expr), Box::new(self)),
			b"%" => Expr::Mod(Box::new(expr), Box::new(self)),
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
					b"+" => Expr::Add(Box::new(acc), Box::new(e)),
					b"-" => Expr::Sub(Box::new(acc), Box::new(e)),
					_ => unreachable!()
				};
				curr_op = op;
				res
			});
		
		match curr_op {
			b"+" => Expr::Add(Box::new(expr), Box::new(self)),
			b"-" => Expr::Sub(Box::new(expr), Box::new(self)),
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
					b"&" => Expr::BitAnd(Box::new(acc), Box::new(e)),
					b"|" => Expr::BitOr(Box::new(acc), Box::new(e)),
					b"^" => Expr::BitXor(Box::new(acc), Box::new(e)),
					_ => unreachable!()
				};
				curr_op = op;
				res
			});
		
		match curr_op {
			b"&" => Expr::BitAnd(Box::new(expr), Box::new(self)),
			b"|" => Expr::BitOr(Box::new(expr), Box::new(self)),
			b"^" => Expr::BitXor(Box::new(expr), Box::new(self)),
			_ => unreachable!()
		}
	}
	
	pub fn eval(&self, symtab: &SymTab) -> Result<Value, String> {
		match *self {
			Expr::Factor(ref fac) => {
				fac.eval(symtab)
			}
			
			Expr::Size(ref id) => {
				match symtab[id] {
					Value::Stack(ref v) => Ok(Value::Int(v.len() as i16)),
					Value::Array(ref v) => Ok(Value::Int(v.len() as i16)),
					Value::Int(_) => Err(format!("size() used on int")),
				}
			}
			
			Expr::Top(ref id) => {
				if let Value::Stack(ref s) = symtab[id] {
					Ok(Value::Int(s.len() as i16))
				} else {
					Err(format!("Expected stack type for top()"))
				}
			}
			
			Expr::Mul(ref exp0, ref exp1) => {
				let val0 = exp0.eval(symtab)?;
				let val1 = exp1.eval(symtab)?;
				
				match (val0, val1) {
					(Value::Int(i0), Value::Int(i1))
						=> Ok(Value::Int(i0 * i1)),
					
					_ => Err(format!("Can't multiply things that aren't ints"))
				}
			}
			
			Expr::Div(ref exp0, ref exp1) => {
				let val0 = exp0.eval(symtab)?;
				let val1 = exp1.eval(symtab)?;
				
				match (val0, val1) {
					(Value::Int(i0), Value::Int(i1))
						=> Ok(Value::Int(i0 / i1)),
					
					_ => Err(format!("Can't divide things that aren't ints"))
				}
			}
			
			Expr::Mod(ref exp0, ref exp1) => {
				let val0 = exp0.eval(symtab)?;
				let val1 = exp1.eval(symtab)?;
				
				match (val0, val1) {
					(Value::Int(i0), Value::Int(i1))
						=> Ok(Value::Int(i0 % i1)),
					
					_ => Err(format!("Can't mod things that aren't ints"))
				}
			}
			
			Expr::Add(ref exp0, ref exp1) => {
				let val0 = exp0.eval(symtab)?;
				let val1 = exp1.eval(symtab)?;
				
				match (val0, val1) {
					(Value::Int(i0), Value::Int(i1))
						=> Ok(Value::Int(i0 + i1)),
					
					_ => Err(format!("Can't add things that aren't ints"))
				}
			}
			
			Expr::Sub(ref exp0, ref exp1) => {
				let val0 = exp0.eval(symtab)?;
				let val1 = exp1.eval(symtab)?;
				
				match (val0, val1) {
					(Value::Int(i0), Value::Int(i1))
						=> Ok(Value::Int(i0 - i1)),
					
					_ => Err(format!("Can't subtract things that aren't ints"))
				}
			}
			
			Expr::BitXor(ref exp0, ref exp1) => {
				let val0 = exp0.eval(symtab)?;
				let val1 = exp1.eval(symtab)?;
				
				match (val0, val1) {
					(Value::Int(i0), Value::Int(i1))
						=> Ok(Value::Int(i0 ^ i1)),
					
					_ => Err(format!("Can't XOR things that aren't ints"))
				}
			}
			
			Expr::BitAnd(ref exp0, ref exp1) => {
				let val0 = exp0.eval(symtab)?;
				let val1 = exp1.eval(symtab)?;
				
				match (val0, val1) {
					(Value::Int(i0), Value::Int(i1))
						=> Ok(Value::Int(i0 & i1)),
					
					_ => Err(format!("Can't AND things that aren't ints"))
				}
			}
			
			Expr::BitOr(ref exp0, ref exp1) => {
				let val0 = exp0.eval(symtab)?;
				let val1 = exp1.eval(symtab)?;
				
				match (val0, val1) {
					(Value::Int(i0), Value::Int(i1))
						=> Ok(Value::Int(i0 | i1)),
					
					_ => Err(format!("Can't OR things that aren't ints"))
				}
			}
		}
	}
}

