use crate::ast::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Deref {
	Direct,
	Index(Factor),
	Field(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LValue {
	pub id: String,
	pub ops: Vec<Deref>,
}

impl LValue {
	pub fn parse(s: &str) -> ParseResult<Self> {
	    let mut ops = Vec::new();
	    let (id, mut s) = ident(s)?;
	    
	    loop {
	        s = s.trim_start();
	        
	        if s.starts_with('!') {
	            s = &s[1..];
	            ops.push(Deref::Direct);
	            continue;
            }
            
            if s.starts_with('[') {
                s = s[1..].trim_left();
                let (fact, sx) = Factor::parse(s)?;
                s = sx.trim_left();
                if !s.starts_with(']') {
                    return Err("no closing bracket at indexed deref".to_string());
                }
                s = &s[1..];
                ops.push(Deref::Index(fact));
                continue;
            }
            
            if s.starts_with('.') {
                s = s[1..].trim_left();
                let (name, sx) = ident(s)?;
                s = sx;
                ops.push(Deref::Field(name.to_string()));
                continue;
            }
            
            break;
        }
        
        Ok((LValue { id: id.to_string(), ops }, s))
	}
	
	pub fn eval(&self, t: &ScopeTable) -> Value {
	    t.locals[&self.id].clone()
	}
	
	/*
	pub fn compile(&self, st: &mut SymbolTable) -> (rel::Reg, Vec<rel::Op>) {
		// TODO maybe move some of the stuff SymbolTable::get does over here?
		st.get(&self.id)
	}
	*/
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple() {
        assert_eq!(
            LValue::parse("a").unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: Vec::new(),
            }, "")
        );
    }
    #[test]
    fn direct() {
        assert_eq!(
            LValue::parse("a!").unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![Deref::Direct],
            }, "")
        );
        assert_eq!(
            LValue::parse("a   !").unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![Deref::Direct],
            }, "")
        );
        assert_eq!(
            LValue::parse("a!!").unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![Deref::Direct, Deref::Direct],
            }, "")
        );
    }
    #[test]
    fn index() {
        assert_eq!(
            LValue::parse("a[0]").unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![Deref::Index(Factor::Lit(Literal::Num(0)))],
            }, "")
        );
        assert_eq!(
            LValue::parse("a   [   0   ]").unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![Deref::Index(Factor::Lit(Literal::Num(0)))],
            }, "")
        );
        assert_eq!(
            LValue::parse("a[0][b]").unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![
                    Deref::Index(Factor::Lit(Literal::Num(0))),
                    Deref::Index(Factor::LVal(LValue {
                        id: "b".to_string(),
                        ops: Vec::new(),
                    })),
                ],
            }, "")
        );
    }
    #[test]
    fn field() {
        assert_eq!(
            LValue::parse("a.b").unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![Deref::Field("b".to_string())],
            }, "")
        );
        assert_eq!(
            LValue::parse("a   .    b").unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![Deref::Field("b".to_string())],
            }, "")
        );
        assert_eq!(
            LValue::parse("a.b.c").unwrap(),
            (LValue {
                id: "a".to_string(),
                ops: vec![
                    Deref::Field("b".to_string()),
                    Deref::Field("c".to_string()),
                ],
            }, "")
        );
    }
}
