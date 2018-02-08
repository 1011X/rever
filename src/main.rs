#[macro_use] extern crate nom;
extern crate regex;

extern crate rel_isa as rel;

mod janus;
mod rever;

fn main() {
	let ast = rever::ast::Program::parse(br#"
	fn f(a: u16, b: bool, mut c: fn(), d: ^type A) {
		let mut a =0;
		let mut b: [i16;1] = 0;
		let c : i16 = 0;
		!a;
		-a;
		a <<= 1;
		a >>= b;
		a ^= b[0] ^c;
		a ^= b[0] & c;
		a += b[0] + c -1;
		a -= b[0] - c + 1;
		a <> b [ 0 ];
		c ? a <> b[0];
		do f( a , b [ 0 ] , 1 );
		undo f();
		if x = 0 { x += 1; } else { x += 2; } fi x = 0;
		from i = 0 {} until i = 0 {};
		drop a = 0;
		drop b = 0;
		drop c = 0;
	}
	
	fn main() {
		do show("finally");
	}"#);
	
	println!("{:#?}", ast);
	//ast.verify();
	//ast.compile();
	
	let (rem, res) = janus::ast::Procedure::parse(br#"
	procedure main(int a, int b)
		a += b
		b -= a
		a ^= b
		b <=> a
	"#).unwrap();
	
	assert!(rem.is_empty());
	
	let res = janus::ast::Procedure::compile(&res);
	
	println!("{:#?}", res);
}
