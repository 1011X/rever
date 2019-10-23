use crate::tokenize::Token;
use crate::interpret::{ScopeTable, Value};
use super::*;

#[derive(Debug)]
pub struct Procedure {
	/// Name of the function.
	pub name: String,
	/// List of parameters for the procedure.
	pub params: Vec<Param>,
	/// Sequence of statements that define the procedure.
	pub code: Vec<Statement>,
}

impl Procedure {
	pub fn eval(&self, env: &mut ScopeTable, args: Vec<Value>) -> Vec<Value> {
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
	
	pub fn uneval(&self, env: &mut ScopeTable, args: Vec<Value>) -> Vec<Value> {
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
	
	pub fn parse(mut tokens: &[Token]) -> ParseResult<Self> {
	    // keyword `proc`
	    if tokens.first() != Some(&Token::Proc) {
	        return Err(format!("expected `proc`"));
	    }
	    tokens = &tokens[1..];
	    
	    let name = match tokens.first() {
	    	Some(Token::Ident(n)) => {
			 	tokens = &tokens[1..];
				n.clone()
			}
			_ => return Err(format!("expected identifier"))
		};
	    
	    // starting '('
	    if tokens.first() != Some(&Token::LParen) {
	    	return Err(format!("expected start of argument list"));
        }
    	tokens = &tokens[1..];
	    
	    // parameters
	    let mut params = Vec::new();
        
        loop {
		    match tokens.first() {
		    	// ending ')'
		    	Some(Token::RParen) => {
		    		tokens = &tokens[1..];
		    		break;
	    		}
	    		// assume param
		    	Some(_) => {
				    let (param, t) = Param::parse(tokens)?;
				    tokens = t;
				    params.push(param);
				    
				    match tokens.first() {
				    	Some(Token::RParen) => {
				    		tokens = &tokens[1..];
				    		break
			    		}
			    		Some(Token::Comma) => {}
			    		_ => return Err(format!("expected `,`"))
			    	}
			    }
			    None => return Err(format!("eof @ parameter list")),
		    }
	    }
	    
	    // code block section
	    
	    // starting '{'
	    if tokens.first() != Some(&Token::LBrace) {
	    	return Err(format!("expected `{{`"));
        }
    	tokens = &tokens[1..];
        
        // code block
        let mut code = Vec::new();
        
        loop {
        	match tokens.first() {
        		// ending '}'
        		Some(Token::RBrace) => {
        			tokens = &tokens[1..];
        			break;
    			}
    			// statement
    			Some(_) => {
    				let (stmt, tx) = Statement::parse(tokens)?;
    				tokens = tx;
    				code.push(stmt);
    			}
    			None => return Err(format!("eof @ proc definition"))
			}
        }
	    
	    Ok((Procedure { name, params, code }, tokens))
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
