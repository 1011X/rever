#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // keywords
    Do, Drop, Else, End, Fi, Fn, From, If, Let, Loop, Mod, Proc, Then, Undo,
    Until, Var,
    // unused
    Match, As, For, In, //Goto, ComeFrom,
    
    // brackets
    LParen, RParen, LBracket, RBracket, LBrace, RBrace,
    
    // single purpose
    Swap,
    // relational
    Neq, Lt, Gt, Lte, Gte,
    // rotation
    Rol, Ror,
    
    // multi-purpose
    Add, At, Colon, Comma, Eq, Period, Semicolon, Sub,
    Star, FSlash,
    Bang, Assign, Caret,
    
    Newline,
    
    Comment(String),
    
    // ident and number
    Ident(String),
    Number(String),
    //Char(String),
    String(String),
}

/*
order of ops:
1. parens
2. function call
3. - ~ @ ! not
4. ** << >> shl shr
5. * / & div mod and
6. + - | ^ or
7. = != < > <= >=
*/


pub fn tokenize(s: &str) -> Result<Vec<Token>, &'static str> {
    let mut tokens = Vec::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    
    while let Some(c) = chars.next() {
        tokens.push(match c {
            // identifiers and keywords
            // [_A-Za-z]
            '_' | 'A'..='Z' | 'a'..='z' => {
                let mut token = String::with_capacity(1);
                token.push(c);
                
                // [_A-Za-z0-9]
                while let Some(&c) = chars.peek() {
                    if c == '_' || c.is_ascii_alphanumeric() {
                        token.push(chars.next().unwrap());
                    }
                    else {
                    	token.shrink_to_fit();
                        break;
                    }
                }
                
                match token.as_str() {
                    // keywords
                    "proc"  => Token::Proc,
                    "do"    => Token::Do,
                    "undo"  => Token::Undo,
                    "from"  => Token::From,
                    "until" => Token::Until,
                    "loop"  => Token::Loop,
                    "if"    => Token::If,
                    "then"  => Token::Then,
                    "else"  => Token::Else,
                    "fi"    => Token::Fi,
                    "let"   => Token::Let,
                    "var"   => Token::Var,
                    "drop"  => Token::Drop,
                    "mod"   => Token::Mod,
                    
                    // reserved
                    "fn"    => Token::Fn,
                    //"return" => Token::Return,
                    "match" => Token::Match,
                    "as"    => Token::As,
                    "for"   => Token::For,
                    "in"    => Token::In,

                    _ => Token::Ident(token)
                }
            }

            // handle numbers
            // [0-9]
            '0'..='9' => {
                let mut token = String::with_capacity(1);
                token.push(c);
				
                // [0-9]*
                while let Some('0'..='9') = chars.peek() {
                    token.push(chars.next().unwrap());
                }
                
                token.shrink_to_fit();
                Token::Number(token)
            }
            
            // handle strings
            '"' => {
            	let mut string = String::new();
            	
            	loop {
            		match chars.next() {
            			Some('"') => break,
            			Some('\\') => string.push(match chars.next() {
            				Some('\\') => '\\',
            				Some('"')  => '"',
            				Some('n')  => '\n',
            				Some('t')  => '\t',
            				Some('0')  => '\0',
            				
	            			Some(_) | None =>
	            				return Err("eof @ escaped character")
            			}),
            			Some(c) => string.push(c),
            			None => return Err("eof @ string"),
            		}
        		}
        		
        		string.shrink_to_fit();
        		Token::String(string)
            }

            '!' => match chars.peek() {
                Some('=') => {chars.next(); Token::Neq}
                _ => Token::Bang
            }
            '<' => match chars.peek() {
                Some('>') => {chars.next(); Token::Swap}
                //Some('=') => {chars.next(); Token::Lte}
                _ => Token::Lt
            }
            '>' => match chars.peek() {
                Some('=') => {chars.next(); Token::Gte}
                //_ => Token::Gt
                _ => return Err("eof @ `>=`")
            }

            '(' => Token::LParen,
            ')' => Token::RParen,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ',' => Token::Comma,
            '.' => Token::Period,
            ':' => match chars.peek() {
                Some('=') => {chars.next(); Token::Assign}
                //_ => Token::Gt
                _ => Token::Colon
            }
            ';' => Token::Semicolon,
            '=' => Token::Eq,
            '+' => Token::Add,
            '-' => Token::Sub,
            '^' => Token::Caret,
            
            '≠' => Token::Neq,
            //'≤' => Token::Lte,
            '≥' => Token::Gte,

            // space
            '\n' => Token::Newline,
            ' ' | '\t' => continue,

            // comment
            '/' => match chars.peek() {
                Some('/') => {
                    chars.next();
                    let mut comment = String::new();
                    
                    while let Some(c) = chars.next() {
                        if c == '\n' { break }
                        else { comment.push(c) }
                    }
                    
                    comment.shrink_to_fit();
                    Token::Comment(comment)
                }
                _ => return Err("//")
            }
            
            _ => return Err("unrecognized symbol")
        });
    }
    
    tokens.shrink_to_fit();
    Ok(tokens)
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn keywords() {
		assert_eq!(tokenize("do").unwrap(), vec![Token::Do]);
		assert_eq!(tokenize("  do  \t").unwrap(), vec![Token::Do]);
		assert_eq!(tokenize("does").unwrap(), vec![
		    Token::Ident(String::from("does"))
	    ]);
	}
}
