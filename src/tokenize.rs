use logos::Logos;

pub type TokenStream<'src> = logos::Lexer<'src, Token>;

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
	#[token("fn")]    Fn,
	#[token("from")]  From,
	#[token("if")]    If,
	#[token("in")]    In,
	#[token("inout")] Inout,
	#[token("let")]   Let,
	#[token("loop")]  Loop,
	#[token("mod")]   Mod,
	#[token("not")]   Not,
	#[token("or")]    Or,
	#[token("out")]   Out,
	#[token("proc")]  Proc,
	#[token("skip")]  Skip,
	#[token("then")]  Then,
	#[token("undo")]  Undo,
	#[token("until")] Until,
	#[token("var")]   Var,
	
	// reserved keywords
	#[token("alias")] Alias,
	#[token("for")]   For,
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
	#[regex("[A-Za-z_][A-Za-z0-9_]*", |lex| lex.slice().to_string())]
	Ident(String),
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
