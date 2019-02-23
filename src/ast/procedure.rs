use crate::ast::*;

#[derive(Debug)]
pub struct Procedure {
	/// Name of the function.
	pub name: String,
	/// Parameters' setup within the function
	pub params: Vec<Param>,
	/// Sequence of statements that make up the function.
	pub code: Vec<Statement>,
}

impl Procedure {
	pub fn eval(&self, args: Vec<Value>, env: &mut ScopeTable) -> Vec<Value> {
	    // verify number of arguments and their types
        assert_eq!(
            args.iter().map(|arg| arg.get_type()).collect::<Vec<_>>(),
            self.params.iter().map(|param| &param.typ).cloned().collect::<Vec<_>>()
        );
        
        // store args in scope table
        let kv_pairs = self.params.iter()
            .map(|param| &param.name)
            .zip(args.into_iter());
        
	    for (name, value) in kv_pairs {
	        env.locals.insert(name.clone(), value);
	    }
	    
	    // execute actual code
	    for stmt in &self.code {
	        stmt.eval(env);
	    }
	    
	    // store arg values back in parameters
	    self.params.iter()
	        .map(|param| env.locals
	            .remove(&param.name)
	            .expect("...parameter disappeared??"))
	        .collect()
	}
	
	pub fn uneval(&self, args: Vec<Value>, env: &mut ScopeTable) -> Vec<Value> {
	    // verify number of arguments and their types
        assert_eq!(
            args.iter().map(|arg| arg.get_type()).collect::<Vec<_>>(),
            self.params.iter().map(|param| &param.typ).cloned().collect::<Vec<_>>()
        );
        
        // store args in scope table
        let kv_pairs = self.params.iter()
            .map(|param| &param.name)
            .zip(args.into_iter());
        
	    for (name, value) in kv_pairs {
	        env.locals.insert(name.clone(), value);
	    }
	    
	    // execute actual code
	    for stmt in &self.code {
	        stmt.clone().invert().eval(env);
	    }
	    
	    // store arg values back in parameters
	    self.params.iter()
	        .map(|param| env.locals
	            .remove(&param.name)
	            .expect("...parameter disappeared??"))
	        .collect()
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
	    
	    // parameters
	    let mut params = Vec::new();
	    
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
            
            let (param, sx) = Param::parse(s)?;
            params.push(param);
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
	    
	    Ok((Procedure { name: name.to_string(), params, code }, s))
	}
    /*
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
