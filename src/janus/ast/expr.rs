use super::*;
use super::super::interpret::{Value, SymTab};

#[derive(Debug, PartialEq, Eq)]
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
	// expr -> leaf {product} {sum} {bitop}
	named!(pub parse<Self>, sp!(do_parse!(
		leaf: call!(Expr::leaf) >>
		prods: many0!(Expr::product) >>
		sums: many0!(Expr::sum) >>
		bitops: many0!(Expr::bitop)
		
		>> (leaf.to_product(prods).to_sum(sums).to_bitop(bitops))
	)));
	
	// leaf -> "size" ( ident )
	//      -> "top" ( ident )
	//      -> ( expr )
	//      -> factor
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
	
	// product -> * leaf
	//         -> / leaf
	//         -> % leaf
	named!(product<(&[u8], Expr)>, sp!(do_parse!(
		op: alt!(tag!("*") | tag!("/") | tag!("%")) >>
		leaf: call!(Expr::leaf)
		>> (op, leaf)
	)));
	
	// sum -> + leaf {product}
	//     -> - leaf {product}
	named!(sum<(&[u8], Expr)>, sp!(do_parse!(
		op: alt!(tag!("+") | tag!("-")) >>
		leaf: call!(Expr::leaf) >>
		prods: many0!(Expr::product)
		
		>> (op, leaf.to_product(prods))
	)));
	
	// bitop -> & leaf {product} {sum}
	//       -> | leaf {product} {sum}
	//       -> ^ leaf {product} {sum}
	named!(bitop<(&[u8], Expr)>, sp!(do_parse!(
		op: alt!(tag!("&") | tag!("|") | tag!("^")) >>
		leaf: call!(Expr::leaf) >>
		prods: many0!(Expr::product) >>
		sums: many0!(Expr::sum)
		
		>> (op, leaf.to_product(prods).to_sum(sums))
	)));
	
	
	fn to_product(self, prods: Vec<(&[u8], Expr)>) -> Expr {
		// use self as accumulator and build a left-to-right Expr tree
		prods.into_iter()
		.fold(self, |acc, (op, e)| match op {
			b"*" => Expr::Mul(Box::new(acc), Box::new(e)),
			b"/" => Expr::Div(Box::new(acc), Box::new(e)),
			b"%" => Expr::Mod(Box::new(acc), Box::new(e)),
			_ => unreachable!()
		})
	}
	
	fn to_sum(self, sums: Vec<(&[u8], Expr)>) -> Expr {
		sums.into_iter()
		.fold(self, |acc, (op, e)| match op {
			b"+" => Expr::Add(Box::new(acc), Box::new(e)),
			b"-" => Expr::Sub(Box::new(acc), Box::new(e)),
			_ => unreachable!()
		})
	}
	
	fn to_bitop(self, bitops: Vec<(&[u8], Expr)>) -> Expr {
		bitops.into_iter()
		.fold(self, |acc, (op, e)| match op {
			b"&" => Expr::BitAnd(Box::new(acc), Box::new(e)),
			b"|" => Expr::BitOr(Box::new(acc), Box::new(e)),
			b"^" => Expr::BitXor(Box::new(acc), Box::new(e)),
			_ => unreachable!()
		})
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


#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn parse_builtins() {
		assert_eq!(
			Expr::parse(b"size(a)").unwrap().1,
			Expr::Size(String::from("a")),
			"size() not parsed"
		);
		assert_eq!(
			Expr::parse(b"top(a)").unwrap().1,
			Expr::Top(String::from("a")),
			"top() not parsed"
		);
	}
	
	#[test]
	fn parse_simple() {
		assert_eq!(
			Expr::parse(b"(a)").unwrap().1,
			Expr::Factor(Factor::LValue(LValue {
				name: String::from("a"),
				indices: vec![],
			})),
			"addition not parsed"
		);
		assert_eq!(
			Expr::parse(b"1 + 2").unwrap().1,
			Expr::Add(
				Box::new(Expr::Factor(Factor::Literal(Literal::Int(1)))),
				Box::new(Expr::Factor(Factor::Literal(Literal::Int(2)))),
			),
			"addition not parsed"
		);
		assert_eq!(
			Expr::parse(b"2 - 1").unwrap().1,
			Expr::Sub(
				Box::new(Expr::Factor(Factor::Literal(Literal::Int(2)))),
				Box::new(Expr::Factor(Factor::Literal(Literal::Int(1)))),
			),
			"subtraction not parsed"
		);
		assert_eq!(
			Expr::parse(b"2 * 1").unwrap().1,
			Expr::Mul(
				Box::new(Expr::Factor(Factor::Literal(Literal::Int(2)))),
				Box::new(Expr::Factor(Factor::Literal(Literal::Int(1)))),
			),
			"multiplication not parsed"
		);
		assert_eq!(
			Expr::parse(b"2 / 1").unwrap().1,
			Expr::Div(
				Box::new(Expr::Factor(Factor::Literal(Literal::Int(2)))),
				Box::new(Expr::Factor(Factor::Literal(Literal::Int(1)))),
			),
			"division not parsed"
		);
		assert_eq!(
			Expr::parse(b"2 % 1").unwrap().1,
			Expr::Mod(
				Box::new(Expr::Factor(Factor::Literal(Literal::Int(2)))),
				Box::new(Expr::Factor(Factor::Literal(Literal::Int(1)))),
			),
			"modulus not parsed"
		);
		assert_eq!(
			Expr::parse(b"2 & 1").unwrap().1,
			Expr::BitAnd(
				Box::new(Expr::Factor(Factor::Literal(Literal::Int(2)))),
				Box::new(Expr::Factor(Factor::Literal(Literal::Int(1)))),
			),
			"bitwise AND not parsed"
		);
		assert_eq!(
			Expr::parse(b"2 | 1").unwrap().1,
			Expr::BitOr(
				Box::new(Expr::Factor(Factor::Literal(Literal::Int(2)))),
				Box::new(Expr::Factor(Factor::Literal(Literal::Int(1)))),
			),
			"bitwise OR not parsed"
		);
		assert_eq!(
			Expr::parse(b"2 ^ 1").unwrap().1,
			Expr::BitXor(
				Box::new(Expr::Factor(Factor::Literal(Literal::Int(2)))),
				Box::new(Expr::Factor(Factor::Literal(Literal::Int(1)))),
			),
			"bitwise XOR not parsed"
		);
	}
	
	#[test]
	fn parse_complex() {
		assert_eq!(
			Expr::parse(b"2 & 3 * 10"),
			Expr::parse(b"2 & (3 * 10)"),
			"bitops grouped before products"
		);
		assert_eq!(
			Expr::parse(b"2 | 3 + 10"),
			Expr::parse(b"2 | (3 + 10)"),
			"bitops grouped before sums"
		);
		assert_eq!(
			Expr::parse(b"2 + 3 * 10"),
			Expr::parse(b"2 + (3 * 10)"),
			"sums grouped before products"
		);
		assert_eq!(
			Expr::parse(b"1 & 2 | 4 ^ 8"),
			Expr::parse(b"((1 & 2) | 4) ^ 8"),
			"ops grouped rtl instead of ltr"
		);
		// wildcard
		assert_eq!(
			Expr::parse(b"2 * 4 + 1 & 128 - 64 / 32 | 16 % 8 ^ 256"),
			Expr::parse(b"((((2 * 4) + 1) & (128 - (64 / 32))) | (16 % 8)) ^ 256"),
			"this heinous expression can't be parsed"
		);
	}
}
