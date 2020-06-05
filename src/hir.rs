/*!
A High-level Intermediate Representation (HIR) of Rever constructs.
*/

// TODO: hir should maybe not depend on interpret?
// And if it *must*, then perhaps merge them.
//use crate::interpret::{EvalResult, Scope, Value, Eval};
use crate::interpret::*;
use crate::ast;

mod expr;
mod function;
mod item;
mod literal;
mod lvalue;
mod module;
mod procedure;
mod statement;
mod term;
mod types;

pub use self::expr::Expr;
pub use self::function::Function;
pub use self::item::Item;
pub use self::literal::Literal;
pub use self::lvalue::LValue;
pub use self::module::Module;
pub use self::procedure::Procedure;
pub use self::statement::Statement;
pub use self::term::Term;
pub use self::types::Type;

type Proc = fn(Box<[Value]>) -> Box<[Value]>;
