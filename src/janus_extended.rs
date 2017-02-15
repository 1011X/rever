#[derive(Debug)]
pub enum BinOp {
	Add, Sub, Xor,
	Mul, Div, Mod,
	BitAnd, BitOr,
	LogAnd, LogOr,
	Gt, Lt,
	Eq, Neq,
	Gte, Lte,
}

#[derive(Debug)]
pub enum ModOp {
	Add, Sub, Xor,
}

type Block<'a> = Vec<Statement<'a>>;

#[derive(Debug)]
pub enum Statement<'a> {
	Local(Type, Factor, Expr),
	Delocal(Type, Factor, Expr),
	ModOp(Factor, ModOp, Expr),
	Swap(Factor, Factor),
	If(Expr, Block<'a>, Option<Block<'a>>, Expr),
	From(Expr, Option<Block<'a>>, Option<Block<'a>>, Expr),
	Call(&'a str),
	Uncall(&'a str),
	Skip,
}

named!(stmt<Statement>, alt!(
	ws!(do_parse!(
		ident
	))
	| ifstmt => { Statement::If }
	| dostmt => { Statement::Do }
	| callstmt => { Statement::Call }
	| modstmt => { Statement::Mod }
));
