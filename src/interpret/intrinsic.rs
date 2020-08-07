use super::{EvalResult, EvalError, Value};
use std::io::prelude::*;

pub type InternProc = fn(&mut [Value]) -> Result<(), EvalError>;
pub type InternFn = fn(&[Value]) -> EvalResult;

pub fn puts(args: &mut [Value]) -> Result<(), EvalError> {
	assert!(args.len() == 1);
	
	let mut rstdout = super::io::RevStdout::new();
	let string = match &args[0] {
		Value::String(s) => s.as_bytes(),
		_ => return Err(EvalError::TypeMismatch {
			expected: Type::String,
			got: &args[0].get_type(),
		})
	};
	
	rstdout.write(string).unwrap();
	Ok(())
}

pub fn unputs(args: &mut [Value]) {
	assert!(args.len() == 1);
	
	let mut rstdout = super::io::RevStdout::new();
	let string = match &args[0] {
		Value::String(s) => s.as_bytes(),
		_ => return Err(EvalError::TypeMismatch {
			expected: Type::String,
			got: &args[0].get_type(),
		})
	};
	
	rstdout.unwrite(string, string.len());
	Ok(())
}
