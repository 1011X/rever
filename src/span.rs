//use std::fmt;
use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
	pub start: usize,
	pub end: usize,
}

impl Span {
	pub fn new(start: usize, len: usize) -> Span {
		Span {
			start: start,
			end: start + len,
		}
	}
	
	pub fn merge(&self, span: &Span) -> Span {
		debug_assert!(self.start < span.end);
		Span {
			start: self.start,
			end: span.end,
		}
	}
}

impl From<Range<usize>> for Span {
	fn from(range: Range<usize>) -> Self {
		Span {
			start: range.start,
			end: range.end,
		}
	}
}


/*
#[derive(Debug, Clone)]
pub struct Span<T: fmt::Debug + Clone>(T, Range<usize>);

impl<T: fmt::Debug + Clone> Span<T> {
	pub fn new(t: T, span: Range<usize>) -> Span<T> {
		Span(t, span)
	}
	
	pub fn merge_with<U, V>(&mut self, other: Span<U>, f: impl FnMut(T, U) -> V) -> Span<V>
	where U: fmt::Debug + Clone, V: fmt::Debug + Clone {
		debug_assert!(self.1.start < other.1.end);
		Span(f(self.0, other.0), self.1.start .. other.1.end)
	}
}
*/
