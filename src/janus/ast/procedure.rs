use super::*;

#[derive(Debug)]
pub struct Procedure {
	name: String,
	args: Vec<Decl>,
	body: Vec<Statement>
}

impl Procedure {
	named!(pub parse<Procedure>, sp!(do_parse!(
		tag!("procedure") >>
		name: ident >>
		args: delimited!(
			tag!("("),
			separated_list!(tag!(","), Decl::parse),
			tag!(")")
		) >>
		body: many1!(Statement::parse)
		
		>> (Procedure {name, args, body})
	)));
}
