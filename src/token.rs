use logos::Logos;

pub type TokenStream<'src> = logos::Lexer<'src, Token>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Logos)]
pub enum Token {
	// keywords
	#[token("and")]    And,
	#[token("as")]     As,
	#[token("begin")]  Begin,
	#[token("do")]     Do,
	#[token("done")]   Done,
	#[token("drop")]   Drop,
	#[token("else")]   Else,
	#[token("end")]    End,
	#[token("fi")]     Fi,
	#[token("fn")]     Fn,
	#[token("from")]   From,
	#[token("if")]     If,
	#[token("let")]    Let,
	#[token("loop")]   Loop,
	#[token("module")] Mod,
	#[token("not")]    Not,
	#[token("or")]     Or,
	#[token("proc")]   Proc,
	#[token("skip")]   Skip,
	#[token("then")]   Then,
	#[token("undo")]   Undo,
	#[token("until")]  Until,
	#[token("var")]    Var,
	
	// reserved keywords
	#[token("alias")] Alias,
	#[token("for")]   For,
	#[token("match")] Match,
	#[token("when")]  When,
//	#[token("struct")] Struct,
	#[token("tag")]   Tag,
//	#[token("union")]  Union,
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
	
	// assignments
	#[token("<>")] Swap,
	#[token(":=")] Assign,
	#[token("+=")] AddAssign,
	#[token("-=")] SubAssign,
	#[token("*=")] MulAssign,
	#[token("/=")] DivAssign,
	
	// multi-purpose
	#[token(":")]  Colon,
	#[token(",")]  Comma,
	#[token(";")]  Semicolon,
	
	#[token("+")]  Plus,
	#[token("-")]  Minus,
	#[token(".")]  Period,
	#[token("*")]  Star,
	#[token("/")]  FSlash,
	#[token("%")]  Percent,
	#[token("!")]  Bang,
	#[token("^")]  Caret,
	#[token("#")]  Hash,
	
	// unused
	#[token("..")] Range,
	#[token("::")] Scope,
	#[token("->")] RightArrow,
	#[token("?")]  QMark,
	
	// v important
	#[token("\n")] Newline,
	
	// identifiers
	#[regex("[A-Za-z_][A-Za-z0-9_]*")]
	Ident,
	
	// literals
	//#[regex("[0-9][0-9']*")]
	#[regex("[0-9]+")]
	Number,
	#[regex(r#""(\\[ntr0"\\]|[^"\\])*""#)]
	String,
	#[regex(r"'(\\[ntr0'\\]|[^'\\])'")]
	Char,
	
	#[regex("~.*", logos::skip)]
	Comment,
	
	#[error]
	#[regex("[ \t\r]+", logos::skip)]
	Error,
}
