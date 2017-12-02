#[macro_use]
extern crate nom;
extern crate regex;

extern crate rel_isa as rel;

//mod janus;
mod janus_extended;
mod rever;

// TODO: fix below example.
// for some reason `a ^= 1 ^ 2` can't be parsed
fn main() {
	
	let (_, ast) = rever::Program::parse(br#"
	fn f(a: u16 , b: bool , mut c : fn ( ) , d : ^ type A ) {
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
	
	fn main ( args : ^^char ) {
		do print("finally");
	}"#).unwrap();
	
	println!("{:#?}", ast);
	
	/*
	ast.verify();
	ast.compile();
	
	let res = janus_extended::Program::parse(br#"
	procedure fib(int x1, int x2, int n)
		if n = 0 then
			x1 += 1
			x2 += 1
		else
			n -= 1
			call fib(x1, x2, n)
			x1 += x2
			x1 <=> x2
		fi x1 = x2

	procedure main()
		local int x1 = 0
		local int x2 = 0
		local int n = 4
		call fib(x1, x2, n)
		delocal int n = 0
		delocal int x2 = 8
		delocal int x1 = 5
	"#);
	
	println!("{:#?}", res);
	*/
}