#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
	// keywords
	And, Do, Drop, Else, End, Fi, From, If, Let, Mod, Not, Or, Proc, Skip, Then,
	Undo, Until, Var,
	// reserved
	Alias, As, Fn, For, In, Loop, Match, Tag, //Goto, ComeFrom,
	
	// brackets
	LParen, RParen, LBracket, RBracket, LBrace, RBrace,
	
	// relational
	Neq, Lt, Gt, Lte, Gte, Eq,
	Rol, Ror,
	
	// statements
	Swap, Assign, AddAssign, SubAssign,
	
	// multi-purpose
	Plus, Colon, Comma, Period, Semicolon, Minus,
	Star, FSlash, Bang, Caret, Range, Scope, Hash,
	RightArrow, QMark,
	
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenError {
	Eof,
	InvalidEscChar,
	UnknownChar,
}

pub type Tokens = TokenStream;

pub fn tokenize(s: &str) -> Result<TokenStream, TokenError> {
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
					"as"    => Token::As,
					"do"    => Token::Do,
					"drop"  => Token::Drop,
					"else"  => Token::Else,
					"end"   => Token::End,
					"fi"    => Token::Fi,
					"fn"    => Token::Fn,
					"from"  => Token::From,
					"if"    => Token::If,
					"let"   => Token::Let,
					"loop"  => Token::Loop,
					"mod"   => Token::Mod,
					"not"   => Token::Not,
					"or"    => Token::Or,
					"proc"  => Token::Proc,
					"skip"  => Token::Skip,
					"undo"  => Token::Undo,
					"until" => Token::Until,
					"var"   => Token::Var,
					
					// reserved
					"alias" => Token::Alias,
					"for"   => Token::For,
					"in"    => Token::In,
					"match" => Token::Match,
					"tag"   => Token::Tag,
					"then"  => Token::Then,
					
					// identifier
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
			'"' | '“' | '«' | '»' => {
				let mut string = String::new();
				let dual = match c {
					'"' => '"',
					'“' => '”',
					'»' => '«',
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
							Some('«')  => '«',
							
							Some('n')  => '\n',
							Some('t')  => '\t',
							Some('r')  => '\r',
							Some('0')  => '\0',
							
							Some(_) =>
								return Err(TokenError::InvalidEscChar),
							None =>
								return Err(TokenError::Eof),
						}),
						Some(c) => string.push(c),
						None => return Err(TokenError::Eof),
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
			'.' => match chars.peek() {
				Some('.') => {
					chars.next();
					Token::Range
				}
				_ => Token::Period
			}
			':' => match chars.peek() {
				Some('=') => {
					chars.next();
					Token::Assign
				}
				Some('>') => {
					chars.next();
					Token::Ror
				}
				Some('<') => {
					chars.next();
					Token::Rol
				}
				Some(':') => {
					chars.next();
					Token::Scope
				}
				/*
				Some('-') => {
					chars.next();
					unimplemented!()
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
				Some('>') => {
					chars.next();
					Token::RightArrow
				}
				_ => Token::Minus
			}
			'*' => Token::Star,
			'/' => Token::FSlash,
			'^' => Token::Caret,
			'#' => Token::Hash,
			'?' => Token::QMark,
			
			// unicode options
			'≠' => Token::Neq,
			'≤' => Token::Lte,
			'≥' => Token::Gte,
			'→' => Token::RightArrow,
			'↔' => Token::Swap,

			// space
			' ' | '\t' | '\r' => continue,
			
			// track newlines
			'\n' => Token::Newline,

			// comment
			'~' => match chars.peek() {
				None => continue,
				Some('[') => {
					chars.next();
					loop {
						if let Some(']') = chars.next() {
							if let Some('~') = chars.peek() {
								chars.next();
								break;
							}
						}
					}
					continue;
				}
				Some('{') => {
					chars.next();
					loop {
						if let Some('}') = chars.next() {
							if let Some('~') = chars.peek() {
								chars.next();
								break;
							}
						}
					}
					continue;
				}
				Some(_) => {
					chars.next();
					while chars.peek() != Some(&'\n') || chars.peek() == None {
						chars.next();
					}
					continue;
				}
			}
			
			_ => return Err(TokenError::UnknownChar)
		});
	}
	
	while tokens.last() == Some(&Token::Newline) {
		tokens.pop();
	}
	
	while tokens.first() == Some(&Token::Newline) {
		tokens.remove(0);
	}
	
	tokens.dedup_by(|a, b| *a == Token::Newline && *b == Token::Newline);
	tokens.shrink_to_fit();
	Ok(TokenStream::new(tokens))
}


#[derive(Clone, Debug)]
pub struct TokenStream {
	stream: Vec<Token>,
}

impl TokenStream {
	fn new(tokens: Vec<Token>) -> Self {
		TokenStream {
			stream: tokens,
		}
	}
	
	pub fn as_inner(self) -> Box<[Token]> {
		self.stream.into_boxed_slice()
	}
	
	pub fn peek(&mut self) -> Option<&Token> {
		self.stream.first()
	}
	
	pub fn next(&mut self) -> Option<Token> {
		if self.stream.is_empty() {
			None
		} else {
			Some(self.stream.remove(0))
		}
	}
	
	pub fn expect(&mut self, tok: &Token) -> Option<Token> {
		if self.peek() == Some(tok) {
			self.next()
		} else {
			None
		}
	}
	
	pub fn len(&self) -> usize {
		self.stream.len()
	}
	
	pub fn is_empty(&self) -> bool {
		self.stream.is_empty()
	}
}

