use std::collections::HashMap;

fn compile(prog: Vec<rever::Item>) -> Vec<rel::Op> {
	use rel::{Op, Addr, Reg};
	use rever::Item;
	
	let functions = HashMap::new();
	
	prog.into_iter()
		.map(|Item::Fn(f)| f)
		.map(|func| {
			
		})
	
	for Item::Fn(f) in prog {
		let symbol_table = f.args.iter()
			.map(|&(m, ref n, ref t)|
				(n.clone(), (m, Some(t.clone())))
			)
			.collect::<HashMap<String, (bool, Option<Type>)>>();
		
		let bytecode = f.code.iter()
			.flat_map(|stmt| match *stmt {
				Not(LValue {}) => vec![
					Op::Not()
				],
			});
	}
}
