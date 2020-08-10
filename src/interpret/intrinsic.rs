use super::{EvalResult, EvalError, Value};
use crate::ast::Type;
use std::io::prelude::*;

pub type InternProc = fn(&mut [Value]) -> EvalResult<()>;
pub type InternFn = fn(&[Value]) -> EvalResult<Value>;

pub fn puts(args: &mut [Value]) -> EvalResult<()> {
	assert!(args.len() == 1);
	
	let mut rstdout = super::io::RevStdout::new();
	
	if let Value::String(string) = &args[0] {
		rstdout.write(string.as_bytes()).unwrap();
		Ok(())
	} else {
		Err(EvalError::TypeMismatch {
			expected: Type::String,
			got: args[0].get_type(),
		})
	}
}

pub fn unputs(args: &mut [Value]) -> EvalResult<()> {
	assert!(args.len() == 1);
	
	let mut rstdout = super::io::RevStdout::new();
	
	if let Value::String(string) = &args[0] {
		rstdout.unwrite(string.as_bytes(), string.len());
		Ok(())
	} else {
		Err(EvalError::TypeMismatch {
			expected: Type::String,
			got: args[0].get_type(),
		})
	}
}
