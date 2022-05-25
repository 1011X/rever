#![allow(dead_code)]

use std::io::prelude::*;
use std::sync::Mutex;

use super::{EvalResult, EvalError, Value, io::{RevStdout, RevStdin}};
use crate::ast::{Type, ProcDef};

lazy_static::lazy_static! {
	static ref STDOUT: Mutex<RevStdout> = Mutex::new(RevStdout::new(false));
	static ref STDIN: Mutex<RevStdin> = Mutex::new(RevStdin::new());
}

// Arguments: str:String, bytes:Uint
// Action: moves str to stdout, increments bytes by number of bytes written.
pub fn print(args: &mut [Value]) -> EvalResult<()> {
	match args {
		[Value::String(string), Value::U32(bytes)] => {
			let mut stdout = STDOUT.lock().unwrap();
			let bytes_read = stdout.write(string.as_bytes()).unwrap();
			*string = string.split_off(bytes_read);
			*bytes += bytes_read as u32;
			Ok(())
		}
		[Value::String(_), val] =>
			Err(EvalError::TypeMismatch {
				expected: Type::U32,
				got: val.get_type(),
			}),
		[val, _] =>
			Err(EvalError::TypeMismatch {
				expected: Type::String,
				got: val.get_type(),
			}),
		_ => panic!("wrong number of parameters: expected 2, got {}", args.len()),
	}
}

// Arguments: str:String, bytes:Uint
// Action: decrements bytes by number of bytes that will be read, and moves
//         stdout data into str.
pub fn unprint(args: &mut [Value]) -> EvalResult<()> {
	match args {
		[Value::String(string), Value::U32(bytes)] => {
			let mut stdout = STDOUT.lock().unwrap();
			let s = String::from_utf8(stdout.unwrite(*bytes as usize)).unwrap();
			*bytes -= s.len() as u32;
			*string = s + &string;
			Ok(())
		}
		[Value::String(_), val] =>
			Err(EvalError::TypeMismatch {
				expected: Type::U32,
				got: val.get_type(),
			}),
		[val, _] =>
			Err(EvalError::TypeMismatch {
				expected: Type::String,
				got: val.get_type(),
			}),
		_ => panic!("wrong number of parameters: expected 2, got not 2"),
	}
}


use crate::ast::Procedure;

pub static PRINT_PROCDEF: ProcDef = ProcDef::Internal {
	fore: print,
	back: unprint,
};





pub fn show(args: &mut [Value]) -> EvalResult<()> {
	assert!(args.len() == 1);
	
	let mut rstdout = RevStdout::new(false);
	
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
	
	let mut rstdout = RevStdout::new(false);
	
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
