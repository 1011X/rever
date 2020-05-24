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
use crate::interpret::{EvalResult, Scope, Value};
use crate::ast;

mod expr;
mod item;
mod literal;
mod lvalue;
mod module;
mod procedure;
mod statement;
mod term;
mod types;

pub use self::expr::Expr;
pub use self::item::Item;
pub use self::literal::Literal;
pub use self::lvalue::LValue;
pub use self::module::Module;
pub use self::procedure::Procedure;
pub use self::statement::Statement;
pub use self::term::Term;
pub use self::types::Type;

type Proc = fn(Box<[Value]>) -> Box<[Value]>;
