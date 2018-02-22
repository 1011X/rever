#![allow(unused_variables, unused_imports)]

#[macro_use] extern crate nom;
extern crate regex;

extern crate rel;

mod janus;
mod rever;

fn main() {
	let ast = rever::ast::Program::parse(br#"
	fn f(a: u16, b: bool, d: ^type A) {
	}
	
	fn main() {
		do show("finally");
	}"#);
	
	//println!("{:#?}", ast);
	//ast.verify();
	//ast.compile();
	
	let (rem, res) = janus::ast::Procedure::parse(br#"
	procedure main(int a, int b)
		local int x = 1
		a += b
		b -= x
		a ^= b
		b <=> a
		delocal int x = 1
	"#).unwrap();
	
	assert!(rem.is_empty());
	
	let res = janus::ast::Procedure::compile(&res);
	
	println!("{:#?}", res);
	println!("{:#?}", janus::compile::optimize(res));
}
