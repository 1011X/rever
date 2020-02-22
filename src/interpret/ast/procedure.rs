use crate::tokenize::Token;
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
	pub fn parse(tokens: &mut Tokens) -> ParseResult<Self> {
	    // keyword `proc`
	    if tokens.next() != Some(Token::Proc) {
	        return Err("`proc`");
	    }
	    
	    // get procedure name
	    let name = match tokens.next() {
	    	Some(Token::Ident(n)) => n,
			_ => return Err("procedure name")
		};
		
		// parse parameter list
		let mut params = Vec::new();
		
		// starting '('
		if tokens.peek() == Some(&Token::LParen) {
			tokens.next();
		    
		    loop {
				// TODO add case for newline for multiline param declaration?
				match tokens.peek() {
					// ending ')'
					Some(Token::RParen) => {
						tokens.next();
						break;
					}
					// try parsing as Param
					Some(_) => {
						params.push(Param::parse(tokens)?);
						
						match tokens.next() {
							Some(Token::RParen) => break,
							Some(Token::Comma) => {}
							_ => return Err("`,`")
						}
					}
					None => return Err("`,` or `)`")
				}
			}
		}
		
		// check for newline
		if tokens.next() != Some(Token::Newline) {
			return Err("newline after parameter list");
		}
	    
	    // code block section
        let mut code = Vec::new();
        
        loop {
		    match tokens.peek() {
				// ending 'end'
				Some(Token::End) => {
					tokens.next();
					break;
				}
				// statement
				Some(_) => {
					let stmt = Statement::parse(tokens)?;
					code.push(stmt);
				}
				None => return Err("a statement or `end`")
			}
		}
	    
	    Ok(Procedure { name, params, code })
	}
	
	
    // TODO perhaps the arguments should be stored in a HashMap, the local vars
    // in a vector, and then turn the vector into a hashmap and compare keys at
    // the end to verify everything is there.
    /*
    fn call_base(&self, forward: bool, args: Vec<Value>) -> Vec<Value> {
	    // verify number of arguments and their types
        assert_eq!(
            args.iter().map(|arg| arg.get_type()).collect::<Vec<_>>(),
            self.params.iter().map(|param| &param.typ).cloned().collect::<Vec<_>>()
        );
        
        // store args in scope stack
        let mut vars: Vec<(String, Value)> = self.params.iter()
            .map(|param| param.name.clone())
            .zip(args.into_iter())
            .collect();
        
	    // execute actual code
        if forward {
	        for stmt in &self.code {
	            stmt.eval(&mut vars);
	        }
        }
        else {
            for stmt in &self.code {
	            stmt.clone().invert().eval(&mut vars);
	        }
        }
        
        // verify number of arguments and their types again
        assert_eq!(
            args.iter().map(|arg| arg.get_type()).collect::<Vec<_>>(),
            self.params.iter().map(|param| &param.typ).cloned().collect::<Vec<_>>()
        );
            
	    // store arg values back in parameters
	    self.params.iter()
	        .map(|param| {
	            let var = vars.iter().rposition(|(id, _)| *id == param.name)
	                .expect("...parameter disappeared??");
	            vars.remove(var);
            })
	        .collect()
    }
    
	pub fn call(&self, args: Vec<Value>) -> Vec<Value> {
	    self.call_base(true, args)
	}
	
	pub fn uncall(&self, args: Vec<Value>) -> Vec<Value> {
	    self.call_base(false, args)
	}
	*/
	
	// add the procedure to the scope
	/*
	pub fn eval(&self, t: &mut Scope) {
	    unimplemented!()
	}
	*/
}
