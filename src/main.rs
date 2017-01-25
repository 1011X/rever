#[macro_use]
extern crate nom;

//mod janus;
//mod janus_extended;
mod rever;

fn main() {
	println!("{:?}", janus::parse_program(b"
	procedure main
		a : b
	"));
}
