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

macro_rules! reb_parse {
	($i:expr, $e:expr) => {
		map_res!(
			$i,
			map_res!(re_bytes_find!($e), str::from_utf8),
			str::parse
		);
	}
}

named!(ident<String>, reb_parse!("^[A-Za-z_][A-Za-z0-9_]*"));
named!(num<u16>, reb_parse!("^[-+]?[0-9]+"));

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

named!(block<Vec<Statement>>, ws!(delimited!(
	tag!("{"),
	// many0! is supressing error in stmt
	many0!(
		terminated!(Statement::parse, tag!(";"))
	),
	tag!("}")
)));

