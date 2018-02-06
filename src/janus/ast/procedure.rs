use super::*;
use rel;

#[derive(Debug)]
pub struct Procedure {
	pub name: String,
	args: Vec<Decl>,
	body: Vec<Statement>
}

impl Procedure {
	named!(pub parse<Self>, sp!(do_parse!(
		tag!("procedure") >>
		name: ident >>
		tag!("(") >>
		args: separated_list!(tag!(","), Decl::parse) >>
		tag!(")") >>
		body: many1!(Statement::parse)
		
		>> (Procedure {name, args, body})
	)));
	
	pub fn compile(&self) -> Vec<rel::Op> {
		unimplemented!();
	}
	
	pub fn verify(&self) {
		unimplemented!();
	}
}
