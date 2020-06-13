use std::ops::Range;

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
	LineComment(String),
	BlockComment(String),
	DocComment(String),
	Char(char),
	
	Other(char),
}

impl Token {
	pub fn with_span(self, start: usize, len: usize) -> TokenSpan {
		TokenSpan {
			span: start .. start + len,
			kind: self,
		}
	}
}

#[derive(Debug, Clone)]
pub struct TokenSpan {
	pub kind: Token,
	pub span: Range<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenError {
	Eof,
	InvalidEscChar(usize, char),
	InvalidChar(usize, usize, char),
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
				let kind = match token.as_str() {
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
				
				kind.with_span(i, len)
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
				Token::Number(token).with_span(i, len)
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
								return Err(TokenError::InvalidEscChar(
									i,
									c
								)),
							None =>
								return Err(TokenError::Eof),
						}),
						Some(c) => string.push(c),
						None => return Err(TokenError::Eof),
					}
				}
				
				string.shrink_to_fit();
				let len = string.len();
				Token::String(string).with_span(i, len)
			}
			
			'\'' => {
				let t = match chars.next().map(|t| t.1) {
					Some('\\') => TokenSpan {
						span: i .. i+4,
						kind: Token::Char(match chars.next().map(|t| t.1) {
							Some('\\') => '\\',
							Some('\'') => '\'',
							Some('n') => '\n',
							Some('t') => '\t',
							Some('r') => '\r',
							Some('0') => '\0',
							Some(c) => return Err(TokenError::InvalidEscChar(i, c)),
							None => return Err(TokenError::Eof),
						}),
					},
					Some(c) => TokenSpan {
						span: i .. i+3,
						kind: Token::Char(c),
					},
					None => return Err(TokenError::Eof),
				};
				
				match chars.next().map(|t| t.1) {
					Some('\'') => {}
					Some(c) => return Err(TokenError::InvalidChar(
						i, 
						tokens.into_iter()
							.filter(|t: &TokenSpan| t.kind == Token::Newline)
							.count(),
						c
					)),
					None => return Err(TokenError::Eof),
				}
				
				t
			}

			'!' => match chars.peek().map(|t| t.1) {
				Some('=') => {
					chars.next();
					Token::Neq.with_span(i, 2)
				}
				_ => Token::Bang.with_span(i, 1)
			}
			'<' => match chars.peek().map(|t| t.1) {
				Some('>') => {
					chars.next();
					Token::Swap.with_span(i, 2)
				}
				Some('=') => {
					chars.next();
					Token::Lte.with_span(i, 2)
				}
				_ => Token::Lt.with_span(i, 1)
			}
			'>' => match chars.peek().map(|t| t.1) {
				Some('=') => {
					chars.next();
					Token::Gte.with_span(i, 2)
				}
				_ => Token::Gt.with_span(i, 1)
			}

			'(' => Token::LParen.with_span(i, 1),
			')' => Token::RParen.with_span(i, 1),
			'[' => Token::LBracket.with_span(i, 1),
			']' => Token::RBracket.with_span(i, 1),
			'{' => Token::LBrace.with_span(i, 1),
			'}' => Token::RBrace.with_span(i, 1),
			',' => Token::Comma.with_span(i, 1),
			'.' => match chars.peek().map(|t| t.1) {
				Some('.') => {
					chars.next();
					Token::Range.with_span(i, 2)
				}
				_ => Token::Period.with_span(i, 1)
			}
			':' => match chars.peek().map(|t| t.1) {
				Some('=') => {
					chars.next();
					Token::Assign.with_span(i, 2)
				}
				Some('>') => {
					chars.next();
					Token::Ror.with_span(i, 2)
				}
				Some('<') => {
					chars.next();
					Token::Rol.with_span(i, 2)
				}
				Some(':') => {
					chars.next();
					Token::Scope.with_span(i, 2)
				}
				/*
				Some('-') => {
					chars.next();
					unimplemented!()
				}
				*/
				_ => Token::Colon.with_span(i, 1)
			}
			';' => Token::Semicolon.with_span(i, 1),
			'=' => Token::Eq.with_span(i, 1),
			'+' => match chars.peek().map(|t| t.1) {
				Some('=') => {
					chars.next();
					Token::AddAssign.with_span(i, 2)
				}
				_ => Token::Plus.with_span(i, 1)
			}
			'-' => match chars.peek().map(|t| t.1) {
				Some('=') => {
					chars.next();
					Token::SubAssign.with_span(i, 2)
				}
				Some('>') => {
					chars.next();
					Token::RightArrow.with_span(i, 2)
				}
				_ => Token::Minus.with_span(i, 1)
			}
			'*' => Token::Star.with_span(i, 1),
			'/' => Token::FSlash.with_span(i, 1),
			'^' => Token::Caret.with_span(i, 1),
			'#' => Token::Hash.with_span(i, 1),
			'?' => Token::QMark.with_span(i, 1),
			
			// unicode options
			'≠' => Token::Neq.with_span(i, '≠'.len_utf8()),
			'≤' => Token::Lte.with_span(i, '≤'.len_utf8()),
			'≥' => Token::Gte.with_span(i, '≥'.len_utf8()),
			'→' => Token::RightArrow.with_span(i, '→'.len_utf8()),
			//'↔' => Token::Swap,

			// space
			' ' | '\t' | '\r' => continue,
			
			// track newlines
			'\n' => Token::Newline.with_span(i, 1),

			// comment
			'~' => match chars.peek().map(|t| t.1) {
				None => Token::LineComment(String::new()).with_span(i, 1),
				Some('[') => {
					chars.next();
					
					let mut comment = String::new();
					
					loop {
						match chars.next().map(|t| t.1) {
							Some(']') => match chars.peek().map(|t| t.1) {
								Some('~') => {
									chars.next();
									break;
								}
								Some(_) => comment.push(']'),
								None => return Err(TokenError::Eof),
							}
							Some(c) => comment.push(c),
							None => return Err(TokenError::Eof),
						}
					}
					
					let len = comment.len() + 4;
					Token::BlockComment(comment).with_span(i, len)
				}
				Some('{') => {
					chars.next();
					
					let mut comment = String::new();
					
					loop {
						match chars.next().map(|t| t.1) {
							Some('}') => match chars.peek().map(|t| t.1) {
								Some('~') => {
									chars.next();
									break;
								}
								Some(_) => comment.push('}'),
								None => return Err(TokenError::Eof),
							}
							Some(c) => comment.push(c),
							None => return Err(TokenError::Eof),
						}
					}
					
					let len = comment.len() + 4;
					Token::DocComment(comment).with_span(i, len)
				}
				Some(_) => {
					let mut comment = String::new();
					
					loop {
						match chars.peek().map(|t| t.1) {
							Some('\n') | None => break,
							Some(c) => {
								chars.next();
								comment.push(c);
							}
						}
					}
					
					let len = comment.len();
					Token::LineComment(comment).with_span(i, len)
				}
			}
			
			c => Token::Other(c).with_span(i, 1),
		});
	}
	
	while tokens.last().map(|t| &t.kind) == Some(&Token::Newline) {
		tokens.pop();
	}
	
	while tokens.first().map(|t| &t.kind) == Some(&Token::Newline) {
		tokens.remove(0);
	}
	
	tokens.dedup_by(|a, b|
		a.kind == Token::Newline && b.kind == Token::Newline
	);
	tokens.shrink_to_fit();
	Ok(TokenStream::new(tokens))
}


#[derive(Clone, Debug)]
pub struct TokenStream {
	stream: Vec<TokenSpan>,
}

impl TokenStream {
	fn new(tokens: Vec<TokenSpan>) -> Self {
		TokenStream {
			stream: tokens,
		}
	}
	
	pub fn as_inner(self) -> Box<[TokenSpan]> {
		self.stream.into_boxed_slice()
	}
	
	pub fn peek(&self) -> Option<&Token> {
		self.stream.first().map(|tok| &tok.kind)
	}
	
	pub fn peek_span(&self) -> Option<&TokenSpan> {
		self.stream.first()
	}
	
	pub fn next(&mut self) -> Option<Token> {
		if self.stream.is_empty() {
			None
		} else {
			Some(self.stream.remove(0).kind)
		}
	}
	
	pub fn expect(&mut self, tok: &Token) -> Option<Token> {
		if self.starts_with(tok) {
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
	
	pub fn expect_ident(&mut self) -> Option<String> {
		if let Some(&Token::Ident(_)) = self.peek() {
			if let Some(Token::Ident(id)) = self.next() {
				Some(id)
			} else {
				unreachable!()
			}
		} else {
			None
		}
	}
	
	pub fn starts_with(&self, tok: &Token) -> bool {
		matches!(self.peek(), Some(kind) if kind == tok)
	}
}

