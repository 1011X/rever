use logos::Logos;

#[derive(Debug, Clone, PartialEq, Eq, Logos)]
pub enum Token {
	// keywords
	#[token("and")]   And,
	#[token("as")]    As,
	#[token("begin")] Begin,
	#[token("do")]    Do,
	#[token("drop")]  Drop,
	#[token("else")]  Else,
	#[token("end")]   End,
	#[token("fi")]    Fi,
	#[token("from")]  From,
	#[token("if")]    If,
	#[token("let")]   Let,
	#[token("loop")]  Loop,
	#[token("mod")]   Mod,
	#[token("not")]   Not,
	#[token("or")]    Or,
	#[token("proc")]  Proc,
	#[token("skip")]  Skip,
	#[token("then")]  Then,
	#[token("undo")]  Undo,
	#[token("until")] Until,
	#[token("var")]   Var,
	
	// reserved keywords
	#[token("alias")] Alias,
	#[token("fn")]    Fn,
	#[token("for")]   For,
	#[token("in")]    In,
	#[token("match")] Match,
	#[token("tag")]   Tag,
	//Goto,
	//ComeFrom,
	
	// brackets
	#[token("(")] LParen,
	#[token(")")] RParen,
	#[token("[")] LBracket,
	#[token("]")] RBracket,
	#[token("{")] LBrace,
	#[token("}")] RBrace,
	
	// relational
	#[token("!=")] Neq,
	#[token("<")]  Lt,
	#[token(">")]  Gt,
	#[token("<=")] Lte,
	#[token(">=")] Gte,
	#[token("=")]  Eq,
	#[token(":<")] Rol,
	#[token(":>")] Ror,
	
	// statements
	#[token("<>")] Swap,
	#[token(":=")] Assign,
	#[token("+=")] AddAssign,
	#[token("-=")] SubAssign,
	
	// multi-purpose
	#[token("+")]  Plus,
	#[token(":")]  Colon,
	#[token(",")]  Comma,
	#[token(".")]  Period,
	#[token(";")]  Semicolon,
	#[token("-")]  Minus,
	#[token("*")]  Star,
	#[token("/")]  FSlash,
	#[token("!")]  Bang,
	#[token("^")]  Caret,
	#[token("..")] Range,
	#[token("::")] Scope,
	#[token("#")]  Hash,
	#[token("->")] RightArrow,
	#[token("?")]  QMark,
	
	// v important
	#[token("\n")] Newline,
	
	// ident and number
	//#[regex("[A-Za-z_][A-Za-z0-9_]*")]
	Ident(String),
	//#[regex("[0-9]+")]
	Number(String),
	String(String),
	Char(char),
	
	Other(char),
	
	#[error]
	#[regex("[ \t\r]+", logos::skip)]
	Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenError {
	Eof,
	InvalidEscChar(usize, char),
	InvalidChar(usize, char),
}

//pub type Tokens = TokenStream;

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
					"begin" => Token::Begin,
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
				
				token
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
				Token::String(string)
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
				
				t
			}

			'!' => match chars.peek().map(|t| t.1) {
				Some('=') => {
					chars.next();
					Token::Neq
				}
				_ => Token::Bang,
			}
			'<' => match chars.peek().map(|t| t.1) {
				Some('>') => {
					chars.next();
					Token::Swap
				}
				Some('=') => {
					chars.next();
					Token::Lte
				}
				_ => Token::Lt,
			}
			'>' => match chars.peek().map(|t| t.1) {
				Some('=') => {
					chars.next();
					Token::Gte
				}
				_ => Token::Gt,
			}

			'(' => Token::LParen,
			')' => Token::RParen,
			'[' => Token::LBracket,
			']' => Token::RBracket,
			'{' => Token::LBrace,
			'}' => Token::RBrace,
			',' => Token::Comma,
			'.' => match chars.peek().map(|t| t.1) {
				Some('.') => {
					chars.next();
					Token::Range
				}
				_ => Token::Period,
			}
			':' => match chars.peek().map(|t| t.1) {
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
				_ => Token::Colon,
			}
			';' => Token::Semicolon,
			'=' => Token::Eq,
			'+' => match chars.peek().map(|t| t.1) {
				Some('=') => {
					chars.next();
					Token::AddAssign
				}
				_ => Token::Plus,
			}
			'-' => match chars.peek().map(|t| t.1) {
				Some('=') => {
					chars.next();
					Token::SubAssign
				}
				Some('>') => {
					chars.next();
					Token::RightArrow
				}
				_ => Token::Minus,
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
			//'↔' => Token::Swap,

			// space
			' ' | '\t' | '\r' => continue,
			
			// track newlines
			'\n' => Token::Newline,

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
				//Token::Comment(comment)
				continue;
			}
			
			c => Token::Other(c),
		});
	}
	
	// remove any starting newlines
	while tokens.first() == Some(&Token::Newline) {
		tokens.remove(0);
	}
	
	tokens.shrink_to_fit();
	Ok(TokenStream::new(tokens))
}

//pub type TokenStream<'src> = std::iter::Peekable<logos::SpannedIter<'src, Token>>;

#[derive(Clone, Debug)]
pub struct TokenStream {
	stream: std::vec::IntoIter<Token>,
}

impl Iterator for TokenStream {
	type Item = Token;
	
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
	pub fn new(tokens: Vec<Token>) -> Self {
		TokenStream {
			stream: tokens.into_iter(),
		}
	}
	
	pub fn as_slice(&self) -> &[Token] {
		self.stream.as_slice()
	}
	
	pub fn peek(&self) -> Option<&Token> {
		self.as_slice().first()
	}
	
	pub fn is_empty(&self) -> bool {
		self.stream.len() == 0
	}
}

