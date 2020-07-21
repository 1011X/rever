use crate::span::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
	// keywords
	And, As, Do, Drop, Else, End, Fi, From, If, Let, Loop, Mod, Not, Or, Proc,
	Skip, Then, Undo, Until, Var,
	// reserved
	Alias, Fn, For, In, Match, Tag, //Goto, ComeFrom,
	
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
	String(String),
	Char(char),
	
	Other(char),
}

impl Token {
	pub fn at(self, start: usize, len: usize) -> (Token, Span) {
		(self, Span::new(start, len))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenError {
	Eof,
	InvalidEscChar(usize, char),
	InvalidChar(usize, char),
}

pub type Tokens = TokenStream;

pub fn tokenize(s: &str) -> Result<TokenStream, TokenError> {
	let mut tokens = Vec::with_capacity(s.len() / 2);
	let mut chars = s.chars().enumerate().peekable();
	
	while let Some((i, c)) = chars.next() {
		tokens.push(match c {
			// identifiers and keywords
			// [_A-Za-z]
			'_' | 'A'..='Z' | 'a'..='z' => {
				let mut token = String::with_capacity(5);
				
				token.push(c);
				
				// [_A-Za-z0-9]*
				while let Some(&(_, c)) = chars.peek() {
					if c == '_' || c.is_ascii_alphanumeric() {
						chars.next();
						token.push(c);
					} else {
						token.shrink_to_fit();
						break;
					}
				}
				
				let len = token.len();
				let token = match token.as_str() {
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
				};
				
				token.at(i, len)
			}

			// handle numbers
			// [0-9]
			'0'..='9' => {
				let mut token = String::with_capacity(1);
				token.push(c);
				
				// [0-9]*
				while let Some((_, c @ '0'..='9')) = chars.peek() {
					token.push(*c);
					chars.next();
				}
				
				token.shrink_to_fit();
				let len = token.len();
				Token::Number(token).at(i, len)
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
					match chars.next().map(|t| t.1) {
						Some(c) if c == dual => break,
						Some('\\') => string.push(match chars.next().map(|t| t.1) {
							Some('\\') => '\\',
							Some('"')  => '"',
							Some('”')  => '”',
							Some('»')  => '»',
							Some('«')  => '«',
							
							Some('n')  => '\n',
							Some('t')  => '\t',
							Some('r')  => '\r',
							Some('0')  => '\0',
							
							Some(c) =>
								return Err(TokenError::InvalidEscChar(i, c)),
							None =>
								return Err(TokenError::Eof),
						}),
						Some(c) => string.push(c),
						None => return Err(TokenError::Eof),
					}
				}
				
				string.shrink_to_fit();
				let len = string.len();
				Token::String(string).at(i, len)
			}
			
			'\'' => {
				let (t, len) = match chars.next().map(|t| t.1) {
					Some('\\') => (Token::Char(match chars.next().map(|t| t.1) {
						Some('\\') => '\\',
						Some('\'') => '\'',
						Some('n') => '\n',
						Some('t') => '\t',
						Some('r') => '\r',
						Some('0') => '\0',
						Some(c) => return Err(TokenError::InvalidEscChar(i, c)),
						None => return Err(TokenError::Eof),
					}), 4),
					Some(c) => (Token::Char(c), 3),
					None => return Err(TokenError::Eof),
				};
				
				match chars.next().map(|t| t.1) {
					Some('\'') => {}
					Some(c) => return Err(TokenError::InvalidChar(i, c)),
					None => return Err(TokenError::Eof),
				}
				
				t.at(i, len)
			}

			'!' => match chars.peek().map(|t| t.1) {
				Some('=') => {
					chars.next();
					Token::Neq.at(i, 2)
				}
				_ => Token::Bang.at(i, 1),
			}
			'<' => match chars.peek().map(|t| t.1) {
				Some('>') => {
					chars.next();
					Token::Swap.at(i, 2)
				}
				Some('=') => {
					chars.next();
					Token::Lte.at(i, 2)
				}
				_ => Token::Lt.at(i, 1),
			}
			'>' => match chars.peek().map(|t| t.1) {
				Some('=') => {
					chars.next();
					Token::Gte.at(i, 2)
				}
				_ => Token::Gt.at(i, 1),
			}

			'(' => Token::LParen.at(i, 1),
			')' => Token::RParen.at(i, 1),
			'[' => Token::LBracket.at(i, 1),
			']' => Token::RBracket.at(i, 1),
			'{' => Token::LBrace.at(i, 1),
			'}' => Token::RBrace.at(i, 1),
			',' => Token::Comma.at(i, 1),
			'.' => match chars.peek().map(|t| t.1) {
				Some('.') => {
					chars.next();
					Token::Range.at(i, 2)
				}
				_ => Token::Period.at(i, 1),
			}
			':' => match chars.peek().map(|t| t.1) {
				Some('=') => {
					chars.next();
					Token::Assign.at(i, 2)
				}
				Some('>') => {
					chars.next();
					Token::Ror.at(i, 2)
				}
				Some('<') => {
					chars.next();
					Token::Rol.at(i, 2)
				}
				Some(':') => {
					chars.next();
					Token::Scope.at(i, 2)
				}
				/*
				Some('-') => {
					chars.next();
					unimplemented!()
				}
				*/
				_ => Token::Colon.at(i, 1),
			}
			';' => Token::Semicolon.at(i, 1),
			'=' => Token::Eq.at(i, 1),
			'+' => match chars.peek().map(|t| t.1) {
				Some('=') => {
					chars.next();
					Token::AddAssign.at(i, 2)
				}
				_ => Token::Plus.at(i, 1),
			}
			'-' => match chars.peek().map(|t| t.1) {
				Some('=') => {
					chars.next();
					Token::SubAssign.at(i, 2)
				}
				Some('>') => {
					chars.next();
					Token::RightArrow.at(i, 2)
				}
				_ => Token::Minus.at(i, 1),
			}
			'*' => Token::Star.at(i, 1),
			'/' => Token::FSlash.at(i, 1),
			'^' => Token::Caret.at(i, 1),
			'#' => Token::Hash.at(i, 1),
			'?' => Token::QMark.at(i, 1),
			
			// unicode options
			'≠' => Token::Neq.at(i, 1),
			'≤' => Token::Lte.at(i, 1),
			'≥' => Token::Gte.at(i, 1),
			'→' => Token::RightArrow.at(i, 1),
			//'↔' => Token::Swap,

			// space
			' ' | '\t' | '\r' => continue,
			
			// track newlines
			'\n' => Token::Newline.at(i, 1),

			// comment
			'~' => {
				//let mut comment = String::new();
				
				loop {
					match chars.peek().map(|t| t.1) {
						Some('\n') | None => break,
						Some(c) => {
							chars.next();
							//comment.push(c);
						}
					}
				}
				
				//let len = comment.len();
				//Token::Comment(comment).at(i, len)
				continue;
			}
			
			c => Token::Other(c).at(i, 1),
		});
	}
	
	// remove any starting newlines
	while tokens.first().map(|t| &t.0) == Some(&Token::Newline) {
		tokens.remove(0);
	}
	
	tokens.dedup_by(|a, b|
		a.0 == Token::Newline && b.0 == Token::Newline
	);
	tokens.shrink_to_fit();
	Ok(TokenStream::new(tokens))
}


#[derive(Clone, Debug)]
pub struct TokenStream {
	stream: std::vec::IntoIter<(Token, Span)>,
}

impl Iterator for TokenStream {
	type Item = (Token, Span);
	
	fn next(&mut self) -> Option<Self::Item> {
		self.stream.next()
	}
}

impl ExactSizeIterator for TokenStream {
	fn len(&self) -> usize {
		self.stream.len()
	}
}

impl TokenStream {
	pub fn new(tokens: Vec<(Token, Span)>) -> Self {
		TokenStream {
			stream: tokens.into_iter(),
		}
	}
	
	pub fn as_slice(&self) -> &[(Token, Span)] {
		self.stream.as_slice()
	}
	
	pub fn peek(&self) -> Option<&(Token, Span)> {
		self.as_slice().first()
	}
	
	pub fn is_empty(&self) -> bool {
		self.stream.len() == 0
	}
}

