use std::collections::HashMap;

fn compile(prog: Vec<rever::Item>) -> Vec<rel::Op> {
	use rel::{Op, Addr, Reg};
	use rever::*;
	
	let functions = HashMap::new();
	
	for Item::Fn(f) in prog {
		// add parameters to symbol table
		let sym_table = f.args.iter()
			.map(|&(m, ref id, ref t)|
				(id.clone(), (m, Some(t.clone())))
			)
			.collect::<HashMap<String, (bool, Option<Type>)>>();
		
		let bytecode = f.code.iter()
			.flat_map(|stmt| match *stmt {
				Let(m, ref id, ref t, _) => {
					sym_table.insert(id.clone(), (m, t.clone()));
					vec![]
				}
				
				Not(LValue {}) => vec![
					Op::Not()
				],
			});
		
		functions.insert(f.name.clone(), bytecode);
	}
}
