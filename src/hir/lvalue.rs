use super::*;

#[derive(Debug, Clone)]
pub enum Deref {
	Direct,
	Index(Expr),
	Field(String),
}

#[derive(Debug, Clone)]
pub struct LValue {
	pub id: String,
	pub ops: Vec<Deref>,
}

impl From<crate::ast::LValue> for LValue {
	fn from(v: crate::ast::LValue) -> Self {
		unimplemented!()
	}
}

impl LValue {
	pub fn eval(&self, t: &Scope) -> Value {
	    t.iter().rfind(|(id, _)| *id == self.id).unwrap().1.clone()
	}
}

#[cfg(test)]
mod tests {
	use crate::tokenize::tokenize;
    use super::*;
    
    #[test]
    fn simple() {
        assert_eq!(
            LValue::parse(&tokenize("a").unwrap()).unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: Vec::new(),
            }, &[][..])
        );
    }
    #[test]
    fn direct() {
        assert_eq!(
            LValue::parse(&tokenize("a!").unwrap()).unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![Deref::Direct],
            }, &[][..])
        );
        assert_eq!(
            LValue::parse(&tokenize("a   !").unwrap()).unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![Deref::Direct],
            }, &[][..])
        );
        assert_eq!(
            LValue::parse(&tokenize("a!!").unwrap()).unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![Deref::Direct, Deref::Direct],
            }, &[][..])
        );
    }
    #[test]
    fn index() {
        assert_eq!(
            LValue::parse(&tokenize("a[0]").unwrap()).unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![Deref::Index(Term::Lit(Literal::Unsigned(0)))],
            }, &[][..])
        );
        assert_eq!(
            LValue::parse(&tokenize("a   [   0   ]").unwrap()).unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![Deref::Index(Term::Lit(Literal::Unsigned(0)))],
            }, &[][..])
        );
        assert_eq!(
            LValue::parse(&tokenize("a[0][b]").unwrap()).unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![
                    Deref::Index(Term::Lit(Literal::Unsigned(0))),
                    Deref::Index(Term::LVal(LValue {
                        id: "b".to_string(),
                        ops: Vec::new(),
                    })),
                ],
            }, &[][..])
        );
    }
    #[test]
    fn field() {
        assert_eq!(
            LValue::parse(&tokenize("a.b").unwrap()).unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![Deref::Field("b".to_string())],
            }, &[][..])
        );
        assert_eq!(
            LValue::parse(&tokenize("a   .    b").unwrap()).unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![Deref::Field("b".to_string())],
            }, &[][..])
        );
        assert_eq!(
            LValue::parse(&tokenize("a.b.c").unwrap()).unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![
                    Deref::Field("b".to_string()),
                    Deref::Field("c".to_string()),
                ],
            }, &[][..])
        );
    }
}
