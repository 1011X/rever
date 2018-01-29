use std::collections::HashMap;
use rel;

pub type SymbolTable = HashMap<String, Location>;

/// Tracks location of a symbol or value.
#[derive(Debug, Clone, Copy)]
pub enum Location {
	/// In a CPU register
	Reg(rel::Reg),
	/// In memory at the specified offset in the stack. Variables have positive
	/// values, arguments don't.
	Memory(isize),
}

