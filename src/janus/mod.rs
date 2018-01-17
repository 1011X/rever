#[macro_use] pub mod parse;
pub mod interpret;

pub mod expr;
pub mod program;
pub mod statement;

//pub use parse::Program;
pub use self::program::Program;
pub use self::statement::Statement;
pub use self::expr::Expr;
