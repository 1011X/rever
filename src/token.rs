use logos::Logos;


pub type TokenStream<'src> = logos::Lexer<'src, Token>;

/// The different tokens produced by the lexer.
#[derive(Logos)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
	// keywords
	#[token("and")]    And,
	#[token("const")]  Const,
	#[token("do")]     Do,
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
	#[token("return")] Return,
	#[token("skip")]   Skip,
	#[token("undo")]   Undo,
	#[token("until")]  Until,
	#[token("var")]    Var,
	
	// reserved keywords
	#[token("alias")]  Alias,
	#[token("as")]     As,
	#[token("begin")]  Begin,
	#[token("done")]   Done,
	#[token("extern")] Extern,
	#[token("for")]    For,
	#[token("match")]  Match,
	#[token("struct")] Struct,
	#[token("tag")]    Tag,
	#[token("then")]   Then,
	#[token("union")]  Union,
	
	// brackets
	#[token("(")] LParen,
	#[token(")")] RParen,
	#[token("[")] LBracket,
	#[token("]")] RBracket,
	#[token("{")] LBrace,
	#[token("}")] RBrace,
	
	// relational
	#[token("=")]  Eq,
	#[token("!=")] Neq,
	#[token("<")]  Lt,
	#[token(">")]  Gt,
	#[token("<=")] Lte,
	#[token(">=")] Gte,
	
	// assignments
	#[token("<>")] Swap,
	#[token(":=")] Assign,
	#[token("+=")] AddAssign,
	#[token("-=")] SubAssign,
	//#[token("*=")] MulAssign,
	//#[token("/=")] DivAssign,
	#[token(":<")] RolAssign,
	#[token(":>")] RorAssign,
	#[token("^=")] XorAssign,
	
	// multi-purpose
	#[token(":")] Colon,
	#[token(".")] Period,
	#[token(",")] Comma,
	#[token(";")] Semicolon,
	
	#[token("+")] Plus,
	#[token("-")] Minus,
	#[token("*")] Star,
	#[token("/")] FSlash,
	#[token("%")] Percent,
	#[token("!")] Bang,
	#[token("^")] Caret,
	#[token("~")] Tilde,
	#[token("_")] Underscore,
	
	// unused
	#[token("::")] Scope,
	#[token("->")] RightArrow,
	#[token("?")]  QMark,
	#[token(":-")] Impls,
	
	// v important
	#[token("\n")] Newline,
	
	// identifiers
	
	#[regex("[a-z_][A-Za-z0-9_]*")]
	VarIdent,
	#[regex("[A-Z][A-Za-z0-9_]*")]
	ConIdent,
	
	// literals
	
	#[regex("0[1-9aA']*")]
	//#[regex("0d[0-9']+")]
	#[regex("0b[01']+")]
	#[regex("0x[0-9a-fA-F']+")]
	Number,
	
	#[regex(r#""(\\[ntr0"\\]|[^"\\])*""#)]
	String,
	#[regex(r"'(\\[ntr0'\\]|[^'\\])'")]
	Char,
	
	#[regex("#.*", logos::skip)]
	Comment,
	
	#[error]
	#[regex("[ \t\r]+", logos::skip)]
	Error,
}
