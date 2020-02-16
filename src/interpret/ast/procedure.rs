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
	
	pub fn parse(mut tokens: &[Token]) -> ParseResult<Self> {
	    // keyword `proc`
	    if tokens.first() != Some(&Token::Proc) {
	        return Err(format!("expected `proc` {:?}", tokens));
	    }
	    tokens = &tokens[1..];
	    
	    // get procedure name
	    let name = match tokens.first() {
	    	Some(Token::Ident(n)) => {
			 	tokens = &tokens[1..];
				n.clone()
			}
			_ => return Err(format!("expected identifier @ proc name"))
		};
		
		// parse parameter list
		let mut params = Vec::new();
		
		// starting '('
		if tokens.first() == Some(&Token::LParen) {
			tokens = &tokens[1..];
		    
		    loop {
				// TODO add case for newline for multiline param declaration?
				match tokens.first() {
					// ending ')'
					Some(Token::RParen) => {
						tokens = &tokens[1..];
						break;
					}
					// try parsing Param
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
		}
		
		// check for newline
		if tokens.first() != Some(&Token::Newline) {
			return Err(format!("expected newline after parameter list"));
		}
		tokens = &tokens[1..];
	    
	    // code block section
        let mut code = Vec::new();
        
        loop {
        	match tokens.first() {
        		// ending 'end'
        		Some(Token::End) => {
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
