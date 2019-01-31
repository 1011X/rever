#![allow(dead_code)]
pub mod ast;
pub mod compile;

use rel::Op;
use super::reverse::Reverse;


impl Reverse for Op {
	fn reverse(self) -> Self {
		self.invert()
	}
}

impl<T: Reverse + Clone> Reverse for Vec<T> {
	fn reverse(mut self) -> Self {
		for value in &mut self {
			*value = value.clone().reverse();
		}
		(&mut *self).reverse();
		self
	}
}
