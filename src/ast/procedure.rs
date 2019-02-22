use crate::ast::*;

#[derive(Debug)]
pub struct Procedure {
	/// Name of the function.
	pub name: String,
	/// Arguments' setup within the function
	pub args: Vec<Arg>,
	/// Sequence of statements that make up the function.
	pub code: Vec<Statement>,
}

impl Procedure {
	pub fn eval(&self, args: Vec<Value>, env: &mut ScopeTable) {
	    // verify arguments
	    debug_assert!(
	        args.len() == self.args.len(),
	        "called with incorrect number of arguments"
        );
	    
	    for (i, arg) in self.args.iter().enumerate() {
	        env.locals.insert(arg.name.clone(), args[i]);
	    }
	    
	    // execute actual code
	    for stmt in &self.code {
	        stmt.eval(env);
	    }
	    
	    
	}
	
	pub fn uneval(&self, args: Vec<Value>, env: &mut ScopeTable) {
	    for (i, arg) in self.args.iter().enumerate() {
	        env.locals.insert(arg.name.clone(), args[i]);
	    }
	    
	    for stmt in &self.code {
	        stmt.clone().invert().eval(env);
	    }
	}
	
	pub fn parse(mut s: &str) -> ParseResult<Self> {
	    // starting keyword `proc`
	    if !(s.starts_with("proc")
	    && s[4..].starts_with(|c: char| !c.is_ascii_alphanumeric())) {
	        return Err("expected `proc` at start of procedure".to_string());
	    }
	    s = s[4..].trim_start();
	    
	    // name
	    let (name, sx) = ident(s)?;
	    s = sx.trim_start();
	    
	    // arguments
	    let mut args = Vec::new();
	    
	    if !s.starts_with('(') {
	        return Err("expected start of argument list".to_string());
        }
        s = &s[1..];
        
        loop {
            s = s.trim_start();
            
            if s.starts_with(')') {
                s = &s[1..];
                break;
            }
            
            let (arg, sx) = Arg::parse(s)?;
            args.push(arg);
            s = sx.trim_start();
            
            if s.starts_with(',') {
                s = &s[1..];
            }
        }
        s = s.trim_start();
        
        // code block
        let mut code = Vec::new();
        
        if !s.starts_with('{') {
            return Err("expected start of procedure definition".to_string());
        }
        s = &s[1..];
        
        loop {
            s = s.trim_start();
            
            if s.starts_with('}') {
                s = &s[1..];
                break;
            }
            
            let (stmt, sx) = Statement::parse(s)?;
            code.push(stmt);
            s = sx.trim_start();
        }
	    
	    Ok((Procedure { name: name.to_string(), args, code }, s))
	}
    /*
	named!(pub parse<Self>, ws!(do_parse!(
		tag!("proc") >>
		name: ident >>
		args: delimited!(
			tag!("("),
			separated_list!(tag!(","), Arg::parse),
			tag!(")")
		) >>
		code: block
		>> (Procedure { name, args, code })
	)));
	
	pub fn verify(&mut self) {
		for statement in &mut self.code {
			statement.verify();
		}
		
		let decls: Vec<&Statement> = self.code.iter()
			.filter(|&stmt| match *stmt {
				Statement::Let(true, ..) | Statement::Drop(..) => true,
				_ => false
			})
			.collect();
		
		decls.sort_by_key(|&stmt| match *stmt {
			Statement::Let(_, ref id, ..)
			| Statement::Drop(ref id, ..) => id,
			_ => unreachable!()
		});
		
		//decls.dedup_by(|&s0, &s1| )
		
		//for decl in decls.chunks(2)
	}
	
	pub fn compile(&self) -> Vec<rel::Op> {
		let mut body = Vec::new();
		// every symbol is associated with a location, and therefore a value
		let mut symbol_table = SymbolTable::new();
		
		// Add arguments to symbol table. C convention is used.
		for (i, arg) in self.args.iter().enumerate() {
			symbol_table.hashmap.insert(
				arg.name.clone(),
				Location::Memory(-(i as isize))
			);
		}
		
		println!("Symbols: {:?}", symbol_table.hashmap);
		
		// Compile body.
		for statement in &self.code {
			let s = statement.compile(&mut symbol_table);
			println!("{:?}", s);
			body.extend(s);
		}
		
		println!("Code for {}: {:#?}", self.name, body);
		body
	}
	*/
}
