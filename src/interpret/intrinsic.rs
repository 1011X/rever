use super::{EvalResult, EvalError, Value};
use crate::ast::Type;
use std::io::prelude::*;

pub fn show(args: &mut [Value]) -> EvalResult<()> {
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

pub fn unshow(args: &mut [Value]) -> EvalResult<()> {
	assert!(args.len() == 1);
	
	let mut rstdout = super::io::RevStdout::new();
	
	if let Value::String(string) = &args[0] {
		let extracted_data = rstdout.unwrite(string.len());
		assert_eq!(string.as_bytes(), extracted_data.as_slice());
		Ok(())
	} else {
		Err(EvalError::TypeMismatch {
			expected: Type::String,
			got: args[0].get_type(),
		})
	}
}

// Arguments: str:String, bytes:Uint
// Action: moves str to stdout, increments bytes by number of bytes written.
pub fn print(args: &mut [Value]) -> EvalResult<()> {
	let mut rstdout = super::io::RevStdout::new();
	
	match args {
		[Value::String(string), Value::Uint(bytes)] => {
			*bytes += rstdout.write(string.as_bytes()).unwrap() as u64;
		}
		[Value::String(_), val] |
		[val, _] =>
			return Err(EvalError::TypeMismatch {
				expected: Type::String,
				got: val.get_type(),
			}),
		_ => panic!("wrong number of parameters: expected 2, got not 2"),
	}
	
	Ok(())
}

// Arguments: str:String, bytes:Uint
// Action: decrements bytes by number of bytes that will be read, and moves
//         stdout data into str.
pub fn unprint(args: &mut [Value]) -> EvalResult<()> {
	let mut rstdout = super::io::RevStdout::new();
	
	match args {
		[Value::String(string), Value::Uint(len)] => {
			let s = String::from_utf8(rstdout.unwrite(*len as usize)).unwrap();
			*len -= s.len() as u64;
		}
		[Value::String(_), val] |
		[val, _] =>
			return Err(EvalError::TypeMismatch {
				expected: Type::String,
				got: val.get_type(),
			}),
		_ => panic!("wrong number of parameters: expected 2, got not 2"),
	}
	
	Ok(())
}
