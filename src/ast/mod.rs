/*!
AST representation of Rever.
*/

/*
TODO: what does a complete program even look like?

List of state given to program:
* return code
* cli args
* env vars
* heap/memory store

"Devices" to handle:
* filesystem
* stdio

*/

mod arg;
mod expr;
mod factor;
//mod function;
mod procedure;
mod item;
mod literal;
mod lvalue;
mod program;
mod statement;
mod types;

pub use self::arg::Arg;
pub use self::expr::Expr;
pub use self::factor::Factor;
//pub use self::function::Function;
pub use self::procedure::Procedure;
pub use self::item::Item;
pub use self::literal::Literal;
pub use self::lvalue::LValue;
pub use self::program::Program;
pub use self::statement::Statement;
pub use self::types::Type;

use std::str;
use std::collections::HashMap;

pub type ParseResult<'a, T> = Result<(T, &'a str), String>;

macro_rules! has {
	($i:ident, $t:expr) => {
		if $i.starts_with($t) {
			return Err("invalid character".to_string());
		}
		
		$i = &$i[$t.len()..];
	}
}

// ident ::= [_A-Za-z][_A-Za-z0-9]*
pub fn ident(i: &str) -> ParseResult<&str> {
	let mut idx = 0;
	
	if i.is_empty() {
		return Err("reached eof".to_string());
	}
	
	// [A-Za-z_]
	if !i.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_') {
		return Err("doesn't start with valid character".to_string());
	}
	idx += 1;
	
	// [A-Za-z0-9_]*
	while i.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_') {
		idx += 1;
	}
	
	Ok((&i[..idx], &i[idx..]))
}

/*
named!(ch<char>, delimited!(
    tag!("'"),
    alt!(
        value!('\\', tag!(r"\\"))
        | value!('\'', tag!(r"\'"))
        | value!('\n', tag!(r"\n"))
        | value!('\t', tag!(r"\t"))
        | call!(::nom::anychar)
    ),
    tag!("'")
));
*/
pub fn ch(mut i: &str) -> ParseResult<char> {
	// '
	has!(i, "'");
	
	let c =
		// escape character
		if i.starts_with('\\') {
			i = &i[1..];
			
			match &i[..1] {
				"\\" => '\\',
				"'"  => '\'',
				"\n" => '\n',
				"\t" => '\t',
				_ => return Err("unrecognized escaped character".to_string())
			}
		}
		// anything else
		else if i.starts_with('\'') {
			return Err("single quote needs to be escaped".to_string());
		}
		else {
			i.chars().nth(0)
			.ok_or(Err("invalid character".to_string()))?
		}
	;
	
	// '
	has!(i, "'");
	
	Ok((c, i))
}
/*
named!(st<String>, delimited!(
    tag!("\""),
    map_res!(
        escaped_transform!(is_not!("\\\""), '\\', alt!(
            value!(b"\\", tag!("\\"))
            | value!(b"\"", tag!("\""))
            | value!(b"\n", tag!("n"))
            | value!(b"\t", tag!("t"))
        )),
        String::from_utf8
    ),
    tag!("\"")
));

pub fn st(mut i: &str) -> ParseResult<String> {
	let mut s = String::new();
	
	// "
	has!(i, "\"");
	
	
	
	// "
	has!(i, "\"");
}

named!(block<Vec<Statement>>, ws!(delimited!(
	tag!("{"),
	// many0! is supressing error in stmt
	many0!(
		terminated!(Statement::parse, tag!(";"))
	),
	tag!("}")
)));
*/


pub struct ScopeTable {
    procedures: HashMap<String, Procedure>,
    //functions: HashMap<String, Function>,
    locals: HashMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Bool(bool),
    Int(i32),
}

impl From<Literal> for Value {
    fn from(l: Literal) -> Self {
        match l {
            Literal::Bool(b) => Value::Bool(b),
            Literal::Num(n) => Value::Int(n),
        }
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        if b { Value::Bool(true) }
        else { Value::Bool(false) }
    }
}

