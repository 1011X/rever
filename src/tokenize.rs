#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
	// keywords
	And, Do, Drop, Else, End, Fi, From, If, Mod, Not, Or, Proc, Skip, Then,
	Undo, Until, Var,
	// unused
	Fn, Let, Loop, Match, As, For, In, //Goto, ComeFrom,
	
	// brackets
	LParen, RParen, LBracket, RBracket, LBrace, RBrace,
	
	// relational
	Neq, Lt, Gt, Lte, Gte, Eq,
	// TODO: choose symbols for bit rotation in statements and expressions.
	// tentative options:  :<  >:  <:  :>  |<  >|  <|  |>
	Shl, Shr,
	Rol, Ror,
	
	// statements
	Swap, Assign, AddAssign, SubAssign,
	
	// multi-purpose
	Plus, At, Colon, Comma, Period, Semicolon, Minus,
	Star, FSlash, Bang, Caret,
	
	Newline,
	
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
4. ^ << >> shl shr <: :> rol ror
5. * / & div mod and
6. + - or
7. = != < > <= >=
*/

pub type Tokens = std::iter::Peekable<std::vec::IntoIter<Token>>;

pub fn tokenize(s: &str) -> Result<Vec<Token>, &'static str> {
	let mut tokens = Vec::with_capacity(s.len());
	let mut chars = s.chars().peekable();
	
	while let Some(c) = chars.next() {
		tokens.push(match c {
			// identifiers and keywords
			// [_A-Za-z]
			'_' | 'A'..='Z' | 'a'..='z' => {
				let mut token = String::with_capacity(5);
				token.push(c);
				
				// [_A-Za-z0-9]*
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
					"and"   => Token::And,
					"do"    => Token::Do,
					"drop"  => Token::Drop,
					"else"  => Token::Else,
					"end"   => Token::End,
					"fi"    => Token::Fi,
					"fn"    => Token::Fn,
					"from"  => Token::From,
					"if"    => Token::If,
					"let"   => Token::Let,
					"mod"   => Token::Mod,
					"not"   => Token::Not,
					"or"    => Token::Or,
					"proc"  => Token::Proc,
					"skip"  => Token::Skip,
					"then"  => Token::Then,
					"undo"  => Token::Undo,
					"until" => Token::Until,
					"var"   => Token::Var,
					
					// reserved
					"as"    => Token::As,
					"for"   => Token::For,
					"in"    => Token::In,
					"loop"  => Token::Loop,
					"match" => Token::Match,

					_ => Token::Ident(token)
				}
			}

			// handle numbers
			// [0-9]
			'0'..='9' => {
				let mut token = String::with_capacity(1);
				token.push(c);
				
				// [0-9]*
				while let Some(c @ '0'..='9') = chars.peek() {
					token.push(*c);
					chars.next();
				}
				
				token.shrink_to_fit();
				Token::Number(token)
			}
			
			// handle strings
			'"' | '“' | '«' => {
				let mut string = String::new();
				let dual = match c {
					'"' => '"',
					'“' => '”',
					'«' => '»',
					_ => unreachable!()
				};
				
				loop {
					match chars.next() {
						Some(c) if c == dual => break,
						Some('\\') => string.push(match chars.next() {
							Some('\\') => '\\',
							Some('"')  => '"',
							Some('”')  => '”',
							Some('»')  => '»',
							
							Some('n')  => '\n',
							Some('t')  => '\t',
							Some('0')  => '\0',
							
							Some(_) =>
								return Err("unknown escape character"),
							None =>
								return Err("eof @ escaped character"),
						}),
						Some(c) => string.push(c),
						None => return Err("eof @ string"),
					}
				}
				
				string.shrink_to_fit();
				Token::String(string)
			}

			'!' => match chars.peek() {
				Some('=') => {
					chars.next();
					Token::Neq
				}
				_ => Token::Bang
			}
			'<' => match chars.peek() {
				Some('>') => {
					chars.next();
					Token::Swap
				}
				Some('=') => {
					chars.next();
					Token::Lte
				}
				/*
				Some(':') => {
					chars.next();
					Token::Rol
				}
				*/
				_ => Token::Lt
			}
			'>' => match chars.peek() {
				Some('=') => {
					chars.next();
					Token::Gte
				}
				_ => Token::Gt
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
				Some('=') => {
					chars.next();
					Token::Assign
				}
				/*
				Some('>') => {
					chars.next();
					Token::Ror
				}
				*/
				_ => Token::Colon
			}
			';' => Token::Semicolon,
			'=' => Token::Eq,
			'+' => match chars.peek() {
				Some('=') => {
					chars.next();
					Token::AddAssign
				}
				_ => Token::Plus
			}
			'-' => match chars.peek() {
				Some('=') => {
					chars.next();
					Token::SubAssign
				}
				_ => Token::Minus
			}
			'^' => Token::Caret,
			
			'≠' => Token::Neq,
			'≤' => Token::Lte,
			'≥' => Token::Gte,

			// space
			' ' | '\t' => continue,
			
			// track newlines
			'\n' => Token::Newline,

			// comment
			'#' => {
				while chars.peek() != Some(&'\n') || chars.peek() == None {
					chars.next();
				}
				continue;
			}
			
			_ => return Err("unrecognized symbol")
		});
	}
	
	if tokens.last() == Some(&Token::Newline) {
		tokens.pop();
	}
	tokens.dedup_by(|a, b| *a == Token::Newline && *b == Token::Newline);
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
