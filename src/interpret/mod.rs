mod value;

//use std::fs::File;

pub use self::value::Value;

pub type EvalResult = Result<Value, &'static str>;

// TODO: add a scope for items

pub type Scope = Vec<(String, Value)>;

#[derive(Clone)]
pub struct StackFrame {
    args: Vec<Value>,
    locals: Vec<(String, Value)>,
}

pub type Stack = Vec<StackFrame>;

/*
// TODO: ensure reversibility of files and streams
struct ReverIo<T: Read + Write> {
	fn copy(&mut self, buf: &mut [u8]) -> io::Result<usize>;
	fn move(&mut self, buf: &mut [u8]) -> io::Result<usize>;
}
*/
