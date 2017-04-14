#[macro_use]
extern crate nom;
extern crate regex;

extern crate rel_isa as rel;

//mod janus;
//mod janus_extended;
mod rever;

fn main() {
	println!("{:#?}", rever::program(b"fn main() { let a: usize = 0; }"));
}
