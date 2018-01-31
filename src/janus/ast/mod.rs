use std::str;

/// custom macro to remove whitespace and comments
macro_rules! sp (
  ($i:expr, $($args:tt)*) => (
    {
      sep!($i, useless, $($args)*)
    }
  )
);

/// parse based on given regexp
macro_rules! reb_parse {
	($i:expr, $e:expr) => {
		map_res!(
			$i,
			map_res!(re_bytes_find!($e), str::from_utf8),
			str::parse
		);
	}
}

pub mod decl;
pub mod expr;
pub mod factor;
pub mod item;
pub mod literal;
pub mod lvalue;
pub mod pred;
pub mod procedure;
pub mod program;
pub mod statement;

pub use self::decl::Decl;
pub use self::expr::Expr;
pub use self::factor::Factor;
pub use self::item::Item;
pub use self::literal::Literal;
pub use self::lvalue::LValue;
pub use self::pred::Pred;
pub use self::procedure::Procedure;
pub use self::program::Program;
pub use self::statement::Statement;


/// removes whitespace, inline comments, and block comments
named!(pub useless, recognize!(many1!(alt_complete!(
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

/// parses an identifier
named!(pub ident<String>, reb_parse!("^[A-Za-z_][A-Za-z0-9_]*"));

/// parses a boolean literal
named!(pub boolean<bool>, reb_parse!("^(true|false)"));

/// parses a string literal
named!(pub st<String>, delimited!(
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

