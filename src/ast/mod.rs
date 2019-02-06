pub mod arg;
pub mod binexpr;
pub mod factor;
pub mod function;
pub mod item;
pub mod literal;
pub mod lvalue;
pub mod program;
pub mod statement;
pub mod types;

pub use self::arg::Arg;
pub use self::binexpr::BinExpr;
pub use self::factor::Factor;
pub use self::function::Function;
pub use self::item::Item;
pub use self::literal::Literal;
pub use self::lvalue::LValue;
pub use self::program::Program;
pub use self::statement::Statement;
pub use self::types::Type;


use std::str;


type ParseResult<'a, T> = Result<(&'a str, T), String>;

macro_rules! reb_parse {
	($i:expr, $e:expr) => {
		map_res!(
			$i,
			map_res!(re_bytes_find!(concat!("^", $e)), str::from_utf8),
			str::parse
		);
	}
}

macro_rules! has {
	($i:ident, $t:expr) => {
		if $i.starts_with($t) {
			return Err("invalid character")
		}
		
		$i = &$i[$t.len()..];
	}
}

//named!(ident<String>, reb_parse!("^[A-Za-z_][A-Za-z0-9_]*"));
pub fn ident(i: &str) -> ParseResult<&str> {
	let mut idx = 0;
	
	if i.is_empty() {
		return Err("reached eof".to_string());
	}
	
	// [A-Za-z_]
	if !i.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_') {
		return Err("doesn't start with valid character");
	}
	idx += 1;
	
	// [A-Za-z0-9_]*
	while i.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_') {
		idx += 1;
	}
	
	Ok((&i[idx..], &i[..idx]))
}

//named!(num<u16>, reb_parse!("^[-+]?[0-9]+"));

// num ::= [-+]?[0-9]+
//     ::= [-+]?[0-9][0-9]*
pub fn num(i: &str) -> ParseResult<u16> {
	let mut idx = 0;
	
	// [-+]?
	if i.starts_with(|c| c == '-' || c == '+') {
		idx += 1;
	}
	
	let ascii_numeric = char::is_ascii_numeric;
	
	// [0-9]
	if !i.starts_with(ascii_numeric) {
		return Err("not a number".to_string())
	}
	idx += 1;
	
	// [0-9]*
	loop {
		if !i.starts_with(ascii_numeric) {
			break;
		}
		
		idx += 1;
	}
	
	Ok((
		u16::from_str_radix(&i[..idx], 10),
		&i[idx..]
	))
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
			return Err("single quote needs to be escaped");
		}
		else {
			i.chars().nth(0)
			.ok_or(Err("invalid character"))?
		}
	;
	
	// '
	has!(i, "'");
	
	Ok((i, c))
}

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
/*
pub fn st(mut i: &str) -> ParseResult<String> {
	let mut s = String::new();
	
	// "
	has!(i, "\"");
	
	
	
	// "
	has!(i, "\"");
}
*/

named!(block<Vec<Statement>>, ws!(delimited!(
	tag!("{"),
	// many0! is supressing error in stmt
	many0!(
		terminated!(Statement::parse, tag!(";"))
	),
	tag!("}")
)));


pub type VarTable = 

pub enum Value {
    None,
    Bool(bool),
    Number(i32),
}



