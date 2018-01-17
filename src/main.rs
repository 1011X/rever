#[macro_use]
extern crate nom;
extern crate regex;

extern crate rel_isa as rel;

mod janus;
//mod rever;

fn main() {
	/*
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
	
	ast.verify();
	ast.compile();
	*/
	
	let res = janus::Program::parse(br#"
	/* Various stack operations */

	/* Move n stack elements from one stack to another */
	procedure move_stack(stack src, stack dst, int n)
		local int m = 0
		from m = 0 loop
			local int x = 0
			pop(x, src)
			push(x, dst)
			delocal int x = 0
			m += 1
		until m = n
		delocal int m = n

	/* Reverse the elements of a stack */
	procedure reverse(stack s)
		if !empty(s) then
			local int x = 0
			local int n_move = size(s) - 1

			pop(x, s)
			call reverse(s)
			// Place x at the bottom of the stack
			// by moving all elements to a temporary stack
			local stack ss = nil
			call move_stack(s, ss, n_move)
			push(x, s)
			call move_stack(ss, s, n_move)
			delocal stack ss = nil

			delocal int n_move = size(s) - 1
			delocal int x = 0
		fi !empty(s)

	stack s
	procedure main()
		push(1, s)
		push(2, s)
		push(3, s)
		push(4, s)
		push(5, s)

		show(s)
		call reverse(s)
	"#);
	
	println!("{:#?}", res);
}
