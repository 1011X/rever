use std::str;
//use std::collections::HashMap;

macro_rules! reb_parse {
	($i:expr, $e:expr) => {
		map_res!(
			$i,
			map_res!(re_bytes_find!($e), str::from_utf8),
			str::parse
		);
	}
}

/// removes whitespace, inline comments, and block comments
named!(useless, recognize!(many1!(alt_complete!(
	preceded!(
		tag!("//"),
		take_until!("\n")
	)
	| delimited!(
		tag!("/*"),
		take_until!("*/"),
		tag!("*/")
	)
	| call!(::nom::sp)
))));

/// custom macro to remove whitespace and comments
macro_rules! sp (
  ($i:expr, $($args:tt)*) => (
    {
      sep!($i, useless, $($args)*)
    }
  )
);

/// parses an identifier
named!(ident<String>, reb_parse!("^[A-Za-z_][A-Za-z0-9_]*"));
/// parses a boolean literal
named!(boolean<bool>, reb_parse!("^(true|false)"));
/// parses a string literal
named!(st<String>, delimited!(
    tag!("\""),
    map_res!(
        escaped_transform!(is_not!("\\\""), '\\', alt_complete!(
            value!(b"\\", tag!("\\"))
            | value!(b"\"", tag!("\""))
            | value!(b"\n", tag!("n"))
            | value!(b"\t", tag!("t"))
        )),
        String::from_utf8
    ),
    tag!("\"")
));

#[derive(Debug)]
enum Type {
	Int, Stack,
	IntArray(Vec<Option<Expr>>),
}

#[derive(Debug)]
pub struct Decl {
	name: String,
	typ: Type,
}

impl Decl {
	named!(parse<Decl>, alt_complete!(
		sp!(do_parse!(
			tag!("int") >>
			name: ident >>
			dims: sp!(many0!(delimited!(
				tag!("["),
				opt!(Expr::parse),
				tag!("]")
			)))
			>> (if dims.is_empty() {
				Decl {name, typ: Type::Int}
			} else {
				Decl {name, typ: Type::IntArray(dims)}
			})
		))
		| sp!(do_parse!(
			tag!("stack") >>
			name: ident
			>> (Decl {name, typ: Type::Stack})
		))
	));
}

#[derive(Debug)]
pub enum Factor {
	LValue(LValue),
	Literal(Literal)
}

impl Factor {
	named!(parse<Factor>, alt_complete!(
		map!(LValue::parse, Factor::LValue)
		| map!(Literal::parse, Factor::Literal)
	));
}

#[derive(Debug)]
pub struct LValue {
	name: String,
	indices: Vec<Expr>,
}

impl LValue {
	named!(parse<LValue>, sp!(do_parse!(
		name: ident >>
		indices: sp!(many0!(delimited!(
			tag!("["),
			call!(Expr::parse),
			tag!("]")
		)))
		>> (LValue {name, indices})
	)));
}

#[derive(Debug)]
pub enum Literal {
	Int(i16),
	IntArray(Vec<Literal>)
}

impl Literal {
	named!(parse<Literal>, alt_complete!(
		map!(reb_parse!("^[-+]?[0-9]+"), Literal::Int)
		| map!(
			sp!(delimited!(
				tag!("{"),
				separated_list!(tag!(","), Literal::parse),
				tag!("}")
			)),
			Literal::IntArray
		)
	));
	/*
	fn to_value(&self) -> Value {
		match *self {
			Literal::Int(i) => Value::Int(i),
			Literal::IntArray(ref vals) => Value::IntArray(vals.clone()),
		}
	}
	*/
}

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
	// expr -> leaf {product} {sum} {bitop}
	named!(parse<Expr>, sp!(do_parse!(
		leaf: call!(Expr::leaf) >>
		prods: sp!(many0!(Expr::product)) >>
		sums: sp!(many0!(Expr::sum)) >>
		bitops: sp!(many0!(Expr::bitop))
		
		>> (leaf.to_product(prods).to_sum(sums).to_bitop(bitops))
	)));
	
	// leaf -> ( expr )
	//      -> factor
	named!(leaf<Expr>, alt_complete!(
		sp!(do_parse!(
			tag!("size") >> tag!("(") >>
			lval: call!(LValue::parse) >>
			tag!(")")
			>> (Expr::Size(lval))
		))
		| sp!(do_parse!(
			tag!("top") >> tag!("(") >>
			lval: call!(LValue::parse) >>
			tag!(")")
			>> (Expr::Top(lval))
		))
		| sp!(delimited!(
			tag!("("),
			call!(Expr::parse),
			tag!(")")
		))
		| map!(Factor::parse, Expr::Factor)
	));
	
	// product -> * leaf {product}
	//         -> / leaf {product}
	//         -> % leaf {product}
	named!(product<(&[u8], Expr)>, sp!(do_parse!(
		op: alt!(tag!("*") | tag!("/") | tag!("%")) >>
		leaf: call!(Expr::leaf)
		>> (op, leaf)
	)));
	
	// sum -> + leaf {product} {sum}
	//     -> - leaf {product} {sum}
	named!(sum<(&[u8], Expr)>, sp!(do_parse!(
		op: alt!(tag!("+") | tag!("-")) >>
		leaf: call!(Expr::leaf) >>
		prods: sp!(many0!(Expr::product))
		
		>> (op, leaf.to_product(prods))
	)));
	
	// bitop -> & leaf {product} {sum} {bitop}
	//       -> | leaf {product} {sum} {bitop}
	named!(bitop<(&[u8], Expr)>, sp!(do_parse!(
		op: alt!(tag!("&") | tag!("|")) >>
		leaf: call!(Expr::leaf) >>
		prods: sp!(many0!(Expr::product)) >>
		sums: sp!(many0!(Expr::sum))
		
		>> (op, leaf.to_product(prods).to_sum(sums))
	)));
	
	fn to_product(self, mut prods: Vec<(&[u8], Expr)>) -> Expr {
		if prods.is_empty() {
			return self;
		}
		
		let (mut curr_op, last) = prods.pop().unwrap();
		let expr = prods.into_iter()
			.rev()
			.fold(last, |acc, (op, e)| {
				let res = match curr_op {
					b"*" => Expr::Mul(Box::new(e), Box::new(acc)),
					b"/" => Expr::Div(Box::new(e), Box::new(acc)),
					b"%" => Expr::Mod(Box::new(e), Box::new(acc)),
					_ => unreachable!()
				};
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
	fn eval(&self, globs: HashMap<String, Value>) -> Result<Value, String> {
		match *self {
			Expr::Size(ref lval) => 
		}
	}
	*/
}

#[derive(Debug)]
pub enum Pred {
	Bool(bool),
	Empty(LValue),
	
	Not(Box<Pred>),
	And(Vec<Pred>),
	Or(Vec<Pred>),
	
	Eq(Expr, Expr),
	Neq(Expr, Expr),
	Gt(Expr, Expr),
	Lt(Expr, Expr),
	Gte(Expr, Expr),
	Lte(Expr, Expr),
}

impl Pred {
	// pred -> leaf {and} {or}
	named!(parse<Pred>, sp!(do_parse!(
		leaf: call!(Pred::leaf) >>
		ands: sp!(many0!(Pred::and)) >>
		ors: sp!(many0!(Pred::or))
		>> ({
			let leaf = if ands.is_empty() { leaf }
			else {
				let mut ands = ands;
				ands.insert(0, leaf);
				Pred::And(ands)
			};
	
			if ors.is_empty() { leaf }
			else {
				let mut ors = ors;
				ors.insert(0, leaf);
				Pred::Or(ors)
			}
		})
	)));
	
	// leaf -> ! not
	//      -> boolean
	//      -> ( pred )
	//      -> empty ( lvalue )
	//      -> expr cmp expr
	// cmp -> =
	//     -> !=
	//     -> >=
	//     -> <=
	//     -> >
	//     -> <
	named!(leaf<Pred>, alt_complete!(
		map!(sp!(preceded!(tag!("!"), Pred::not)), |x| Pred::Not(Box::new(x)))
		| map!(boolean, Pred::Bool)
		| sp!(delimited!( // ( pred )
			tag!("("),
			call!(Pred::parse),
			tag!(")")
		))
		| sp!(do_parse!( // empty(x)
			tag!("empty") >> tag!("(") >>
			lval: call!(LValue::parse) >>
			tag!(")")
			>> (Pred::Empty(lval))
		))
		| sp!(do_parse!( // cmp
			left: call!(Expr::parse) >>
			cmp: alt!(
				tag!("=") | tag!("!=") | tag!(">=") | tag!("<=") | tag!(">") | tag!("<")
			) >>
			right: call!(Expr::parse)
			>> (match cmp {
				b"=" => Pred::Eq(left, right),
				b"!=" => Pred::Neq(left, right),
				b">=" => Pred::Gte(left, right),
				b"<=" => Pred::Lte(left, right),
				b">" => Pred::Gt(left, right),
				b"<" => Pred::Lt(left, right),
				_ => unreachable!()
			})
		))
	));
	
	// not -> ! not
	//     -> boolean
	//     -> ( pred )
	//     -> empty ( lvalue )
	named!(not<Pred>, alt_complete!(
		map!(sp!(preceded!(tag!("!"), Pred::not)), |x| Pred::Not(Box::new(x)))
		| map!(boolean, Pred::Bool)
		| sp!(delimited!( // ( pred )
			tag!("("),
			call!(Pred::parse),
			tag!(")")
		))
		| sp!(do_parse!( // empty(x)
			tag!("empty") >> tag!("(") >>
			lval: call!(LValue::parse) >>
			tag!(")")
			>> (Pred::Empty(lval))
		))
	));
	
	// or -> || leaf {and}
	named!(or<Pred>, sp!(do_parse!(
		tag!("||") >>
		leaf: call!(Pred::leaf) >>
		ands: sp!(many0!(Pred::and))
		>> (if ands.is_empty() {
			leaf
		} else {
			let mut ands = ands;
			ands.insert(0, leaf);
			Pred::And(ands)
		})
	)));
	
	// and -> && leaf
	named!(and<Pred>, sp!(preceded!(tag!("&&"), Pred::left)));
}

type Block = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
	Skip,
	Local(Decl, Expr),
	Delocal(Decl, Expr),
	Add(LValue, Expr),
	Sub(LValue, Expr),
	Xor(LValue, Expr),
	Swap(LValue, LValue),
	If(Pred, Block, Option<Block>, Pred),
	From(Pred, Option<Block>, Option<Block>, Pred),
	Call(String, Vec<Factor>),
	Uncall(String, Vec<Factor>),
	
	// built-ins
	Print(String),
	Printf(String, Vec<Factor>),
	Error(String),
	Show(LValue),
	Pop(LValue, LValue),
	Push(LValue, LValue),
}

impl Statement {
	named!(parse<Statement>, alt_complete!(
		value!(Statement::Skip, tag!("skip"))
		| sp!(do_parse!(
			tag!("local") >>
			decl: call!(Decl::parse) >>
			tag!("=") >>
			val: call!(Expr::parse)
			>> (Statement::Local(decl, val))
		))
		| sp!(do_parse!(
			tag!("delocal") >>
			decl: call!(Decl::parse) >>
			tag!("=") >>
			val: call!(Expr::parse)
			>> (Statement::Delocal(decl, val))
		))
		| sp!(do_parse!(
			left: call!(LValue::parse) >>
			tag!("<=>") >>
			right: call!(LValue::parse)
			>> (Statement::Swap(left, right))
		))
		| sp!(do_parse!(
			left: call!(LValue::parse) >>
			tag!("+=") >>
			expr: call!(Expr::parse)
			>> (Statement::Add(left, expr))
		))
		| sp!(do_parse!(
			left: call!(LValue::parse) >>
			tag!("-=") >>
			expr: call!(Expr::parse)
			>> (Statement::Sub(left, expr))
		))
		| sp!(do_parse!(
			left: call!(LValue::parse) >>
			tag!("^=") >>
			expr: call!(Expr::parse)
			>> (Statement::Xor(left, expr))
		))
		| sp!(do_parse!(
			tag!("from") >>
			assert: call!(Pred::parse) >>
			forward: opt!(sp!(preceded!(
				tag!("do"),
				many1!(Statement::parse)
			))) >>
			backward: opt!(sp!(preceded!(
				tag!("loop"),
				many1!(Statement::parse)
			))) >>
			tag!("until") >>
			pred: call!(Pred::parse)
			
			>> (Statement::From(assert, forward, backward, pred))
		))
		| sp!(do_parse!(
			tag!("if") >>
			pred: call!(Pred::parse) >>
			pass: sp!(preceded!(
				tag!("then"),
				many1!(Statement::parse)
			)) >>
			fail: opt!(sp!(preceded!(
				tag!("else"),
				many1!(Statement::parse)
			))) >>
			tag!("fi") >>
			assert: call!(Pred::parse)
			
			>> (Statement::If(pred, pass, fail, assert))
		))
		| sp!(do_parse!(
			tag!("call") >>
			func: ident >>
			args: delimited!(
				tag!("("),
				separated_list!(tag!(","), Factor::parse),
				tag!(")")
			)
			>> (Statement::Call(func, args))
		))
		| sp!(do_parse!(
			tag!("uncall") >>
			func: ident >>
			tag!("(") >>
			args: separated_list!(tag!(","), Factor::parse) >>
			tag!(")")
			>> (Statement::Uncall(func, args))
		))
		// built-ins
		| sp!(do_parse!(
			tag!("print") >>
			tag!("(") >>
			string: st >>
			tag!(")")
			>> (Statement::Print(string))
		))
		| sp!(do_parse!(
			tag!("printf") >>
			tag!("(") >>
			string: st >>
			vargs: many0!(sp!(preceded!(
				tag!(","),
				Factor::parse
			))) >>
			tag!(")")
			>> (Statement::Printf(string, vargs))
		))
		| sp!(do_parse!(
			tag!("error") >>
			tag!("(") >>
			string: st >>
			tag!(")")
			>> (Statement::Error(string))
		))
		| sp!(do_parse!(
			tag!("show") >>
			tag!("(") >>
			lval: call!(LValue::parse) >>
			tag!(")")
			>> (Statement::Show(lval))
		))
		| sp!(do_parse!(
			tag!("pop") >>
			tag!("(") >>
			into: call!(LValue::parse) >>
			tag!(",") >>
			from: call!(LValue::parse) >>
			tag!(")")
			>> (Statement::Pop(into, from))
		))
		| sp!(do_parse!(
			tag!("push") >>
			tag!("(") >>
			from: call!(LValue::parse) >>
			tag!(",") >>
			into: call!(LValue::parse) >>
			tag!(")")
			>> (Statement::Push(from, into))
		))
	));
}

#[derive(Debug)]
struct Procedure {
	name: String,
	args: Vec<Decl>,
	body: Vec<Statement>
}

impl Procedure {
	named!(parse<Procedure>, sp!(do_parse!(
		tag!("procedure") >>
		name: ident >>
		args: delimited!(
			tag!("("),
			separated_list!(tag!(","), Decl::parse),
			tag!(")")
		) >>
		body: many1!(Statement::parse)
		
		>> (Procedure {name, args, body})
	)));
}

#[derive(Debug)]
enum Item {
	Global(Decl, Option<Expr>),
	Proc(Procedure),
}

impl Item {
	named!(parse<Item>, sp!(alt!(
		map!(Procedure::parse, Item::Proc)
		| sp!(do_parse!(
			decl: call!(Decl::parse) >>
			val: opt!(sp!(preceded!(tag!("="), Expr::parse)))
			>> (Item::Global(decl, val))
		))
	)));
}

/*
enum Value {
	Int(i16),
	Stack(Vec<i16>),
	Array(Vec<Value>),
}
*/

#[derive(Debug)]
pub struct Program {
	items: Vec<Item>
}

impl Program {
	named!(pub parse<Program>, do_parse!(
		items: many1!(Item::parse)
		>> (Program {items})
	));
	/*
	fn run(&self) -> Result<(), String> {
		let mut globs = HashMap::new();
		let mut main = None;
		
		for item in &self.items {
			// populate globs
			if let Item::Global(decl, init) = *item {
				
				let val = match decl.typ {
					Type::Stack => Value::Stack(vec![]),
					Type::Int => {
						init.map(|x| x.eval(&globs).to_value())
						.unwrap_or(Value::Int(0))
					}
					Type::IntArray(ref dims) => {
						if let Some(init) = init {
							
						} else {
							if dims.iter().all(|x| x.is_some()) {
								for dim in dims.iter().rev() {
									let v = vec![];
									
								}
							} else {
								return Err("All array lengths must be specified.".to_string());
							}
						}
					}
				}
				
				match (decl.typ, val) {
					(Type::Int, Value::Int(_)) => {}
					
					(Type::IntArray(ref expr), Value::IntArray(ref vals))
					if expr.eval(&globs).to_int().unwrap() >= vals.len() => {}
					
					_ => return Err("Type and assigned value don't match.".to_string())
				}
				
				globs.insert(decl.name.clone(), val);
			}
			// find main function
			else if let Item::Proc(ref pr) = *item {
				if pr.name == "main" {
					if main.is_none() {
						main = Some(pr);
						break;
					} else {
						return Err("There is more than one main procedure.".to_string());
					}
				}
			}
		}
		
		if main.is_none() {
			return Err("No main function found.".to_string());
		}
	}
	*/
}
