use super::Value;
use std::io::prelude::*;

pub fn puts(args: &mut [Value]) {
	assert!(args.len() == 1);
	
	let mut rstdout = super::io::RevStdout::new();
	let string = match &args[0] {
		Value::String(s) => s.as_bytes(),
		_ => panic!("not a string")
	};
	
	rstdout.write(string).unwrap();
}

pub fn unputs(args: &mut [Value]) {
	assert!(args.len() == 1);
	
	let mut rstdout = super::io::RevStdout::new();
	let string = match &args[0] {
		Value::String(s) => s.as_bytes(),
		_ => panic!("not a string")
	};
	
	rstdout.unwrite(string, string.len());
}
