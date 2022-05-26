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
	#[token("then")]   Then,
	#[token("undo")]   Undo,
	#[token("until")]  Until,
	#[token("var")]    Var,
	
	// reserved keywords
	#[token("alias")]  Alias,
	#[token("const")]  Const,
	#[token("done")]   Done,
	#[token("for")]    For,
	#[token("match")]  Match,
	#[token("when")]   When,
//	#[token("struct")] Struct,
	#[token("tag")]    Tag,
//	#[token("union")]  Union,
	
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
	#[token("^=")] XorAssign,
	#[token("+=")] AddAssign,
	#[token("-=")] SubAssign,
	#[token("*=")] MulAssign,
	#[token("/=")] DivAssign,
	#[token(":<")] RolAssign,
	#[token(":>")] RorAssign,
	
	// multi-purpose
	#[token(":")] Colon,
	#[token(",")] Comma,
	#[token(";")] Semicolon,
	
	#[token("+")] Plus,
	#[token("-")] Minus,
	#[token(".")] Period,
	#[token("*")] Star,
	#[token("/")] FSlash,
	#[token("%")] Percent,
	#[token("!")] Bang,
	#[token("^")] Caret,
	#[token("#")] Hash,
	#[token("_")] Underscore,
	
	// unused
	#[token("..")] Range,
	#[token("::")] Scope,
	#[token("->")] RightArrow,
	#[token("?")]  QMark,
	#[token(":-")] Impls,
	
	#[token("\r?\n")] Newline,
	
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
	
	#[regex("~.*", logos::skip)]
	Comment,
	
	#[error]
	#[regex("[ \t\r]+", logos::skip)]
	Error,
}
