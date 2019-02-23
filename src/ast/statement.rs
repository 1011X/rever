use crate::ast::*;

#[derive(Debug, Clone)]
pub enum Statement {
	Decl(String, Option<Type>, Expr, Vec<Statement>, Expr),
	
	Not(LValue),
	//Neg(LValue),
	
	RotLeft(LValue, Expr),
	RotRight(LValue, Expr),
	
	Xor(LValue, Expr),
	Add(LValue, Expr),
	Sub(LValue, Expr),
	
	Swap(LValue, LValue),
	//CSwap(Factor, LValue, LValue),
	
	Do(LValue, Vec<Expr>),
	Undo(LValue, Vec<Expr>),
	
	If(Expr, Vec<Statement>, Vec<Statement>, Expr),
	From(Expr, Vec<Statement>, Vec<Statement>, Expr),
	
    //Let(String, Option<Type>, Literal),
    //When(Expr, Vec<Statement>, Vec<Statement>),
	//Switch(String, Vec<_, Vec<Statement>>),
	//FromVar(String, Expr, Vec<Statement>, Vec<Statement>, Expr),
}

use self::Statement::*;
impl Statement {
    pub fn invert(self) -> Self {
        match self {
            Decl(name, typ, init, scope, dest) =>
                Decl(name, typ, dest, scope, init),
            
            RotLeft(l, v) => RotRight(l, v),
            RotRight(l, v) => RotLeft(l, v),
            Add(l, v) => Sub(l, v),
            Sub(l, v) => Add(l, v),
            
            Do(p, args) => Undo(p, args),
            Undo(p, args) => Do(p, args),
            
            If(test, block, else_block, assert) =>
                If(assert, block, else_block, test),
            From(assert, do_block, loop_block, test) =>
                From(test, do_block, loop_block, assert),
            
            _ => self
        }
    }
	
	pub fn eval(&self, t: &mut ScopeTable) {
	    match self {
	        /*
	        Var(id, _, lit) => {
	            t.locals.insert(id.clone(), Value::from(*lit));
	        }
	        Drop(id, _, lit) => {
	            assert_eq!(t.locals[id], Value::from(lit.clone()));
	            t.locals.remove(id);
	        }
	        */
	        Not(lval) => {
	            *t.locals.get_mut(&lval.id).unwrap() = match lval.eval(t) {
                    Value::Bool(b) => Value::Bool(!b),
                    Value::Int(i) => Value::Int(!i),
                };
            }
            /*
	        RotLeft(lval, fact) => match (lval.eval(t), fact.eval(t)) {
                (Value::Int(l), Value::Int(r)) =>
                    *t.locals.get_mut(&lval.id).unwrap() = Value::Int(l.rotate_left(r as u32)),
                _ => panic!("tried to do something illegal"),
            }
            RotRight(lval, fact) => match (lval.eval(t), fact.eval(t)) {
                (Value::Int(l), Value::Int(r)) =>
                    *t.locals.get_mut(&lval.id).unwrap() = Value::Int(l.rotate_right(r as u32)),
                _ => panic!("tried to do something illegal"),
            }
            */
            Xor(lval, fact) => match (lval.eval(t), fact.eval(t)) {
                (Value::Int(l), Value::Int(r)) =>
                    *t.locals.get_mut(&lval.id).unwrap() = Value::Int(l ^ r),
                _ => panic!("tried to do something illegal"),
            }
            /*
            Add(lval, fact) => match (lval.eval(t), fact.eval(t)) {
                (Value::Int(l), Value::Int(r)) =>
                    *t.locals.get_mut(&lval.id).unwrap() = Value::Int(l.wrapping_add(r)),
                _ => panic!("tried to do something illegal"),
            }
            Sub(lval, fact) => match (lval.eval(t), fact.eval(t)) {
                (Value::Int(l), Value::Int(r)) =>
                    *t.locals.get_mut(&lval.id).unwrap() = Value::Int(l.wrapping_sub(r)),
                _ => panic!("tried to do something illegal"),
            }
            */
            Swap(left, right) => {
                // TODO check types match
                let temp = left.eval(t);
                *t.locals.get_mut(&left.id).unwrap() = right.eval(t);
                *t.locals.get_mut(&right.id).unwrap() = temp;
            }
            /*
            Do(name, args) => {
                let vals: Vec<Value> = args.iter()
                    .map(|arg| arg.eval(t))
                    .collect();
                t.procedures[&name.id].eval(vals, t);
            }
            Undo(name, args) => {
                let vals: Vec<Value> = args.iter()
                    .map(|arg| arg.eval(t))
                    .collect();
                t.procedures[&name.id].uneval(vals, t);
            }
            If(test, block, else_block, assert) => {
                match test.eval(t) {
                    Value::Bool(true) => {
                        for stmt in block {
                            stmt.eval(t);
                        }
                        assert_eq!(assert.eval(t), Value::Bool(true));
                    }
                    Value::Bool(false) => {
                        for stmt in else_block {
                            stmt.eval(t);
                        }
                        assert_eq!(assert.eval(t), Value::Bool(false));
                    }
                    _ => panic!("tried to do something illegal")
                }
            }
            From(assert, do_block, loop_block, test) => {
                assert_eq!(assert.eval(t), Value::Bool(true));
                loop {
                    for stmt in do_block {
                        stmt.eval(t);
                    }
                    
                    match test.eval(t) {
                        Value::Bool(true) => break,
                        Value::Bool(false) =>
                            for stmt in loop_block {
                                stmt.eval(t);
                            }
                        _ => panic!("tried to do something illegal")
                    }
                    
                    assert_eq!(assert.eval(t), Value::Bool(false));
                }
            }
            */
            _ => unreachable!()
	    }
	}
	
	pub fn parse(mut s: &str) -> ParseResult<Self> {
	    if s.starts_with("do")
	    && !s[2..].starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_') {
	        let (pr, sx) = LValue::parse(s)?;
	        s = sx.trim_start();
	        
	        if !s.starts_with('(') {
	            return Err("expected start of argument list".to_string());
            }
            s = &s[1..];
            
            let mut args = Vec::new();
            
            loop {
                s = s.trim_start();
                
                if s.starts_with(')') {
                    s = &s[1..];
                    break;
                }
                
                let (arg, sx) = Expr::parse(s)?;
                args.push(arg);
                s = sx.trim_start();
                
                if s.starts_with(',') {
                    s = &s[1..];
                }
            }
            
            return Ok((Statement::Do(pr, args), s));
	    }
	    
	    Err("unrecognized statement".to_string())
	}
    /*
	named!(pub parse<Self>, ws!(alt_complete!(
		// not/inversion
		// "!" lval
		map!(preceded!(tag!("!"), LValue::parse), Statement::Not)
		// negation
		// "-" lval
		| map!(preceded!(tag!("-"), LValue::parse), Statement::Neg)
		// procedure call
		// ("do" | "undo") lval "(" {factor ","} ")"
		| do_parse!(
			kw: alt!(tag!("do") | tag!("undo")) >>
			name: call!(LValue::parse) >>
			args: delimited!(
				tag!("("),
				separated_list!(tag!(","), Factor::parse),
				tag!(")")
			)
			>> (match kw {
				b"do" => Statement::Do(name, args),
				b"undo" => Statement::Undo(name, args),
				_ => unreachable!()
			})
		)
		// variable creation/destruction
		// ("let" | "drop") id [: type] literal
		| do_parse!(
			kw: alt!(tag!("let") | tag!("drop")) >>
			m: opt!(tag!("mut")) >>
			name: ident >>
			ty: opt!(preceded!(tag!(":"), Type::parse)) >>
			tag!("=") >>
			l: call!(Literal::parse)
			>> (match kw {
				b"let"  => Statement::Let(m.is_some(), name, ty, l),
				b"drop" => Statement::Drop(m.is_some(), name, ty, l),
				_ => unreachable!()
			})
		)
		// if block/branch
		// "if" expr "then" block ["else" block] "fi" expr
		| do_parse!(
			tag!("if") >> p: call!(BinExpr::parse) >>
			t: block >>
			e: opt!(preceded!(tag!("else"), block)) >>
			tag!("fi") >> a: call!(BinExpr::parse)
			>> (Statement::If(p, t, e, a))
		)
		// from loop
		// "from" binexpr [block] "until" binexpr [block]
		| do_parse!(
			tag!("from") >> a: call!(BinExpr::parse) >>
			d: opt!(block) >>
			tag!("until") >> p: call!(BinExpr::parse) >>
			l: opt!(block)
			>> (Statement::From(a, d, l, p))
		)
		// swap
		// lval "<>" lval
		| do_parse!( 
			left: call!(LValue::parse) >>
			tag!("<>") >>
			right: call!(LValue::parse)
			>> (Statement::Swap(left, right))
		)
		// ccnot, toffoli
		// lval "^=" factor "&" factor
		| do_parse!(
			dest: call!(LValue::parse) >>
			tag!("^=") >>
			lctrl: call!(Factor::parse) >>
			tag!("&") >>
			rctrl: call!(Factor::parse)
			>> (Statement::CCNot(dest, lctrl, rctrl))
		)
		// cswap, fredkin
		// factor "?" lval "<>" lval
		| do_parse!(
			ctrl: call!(Factor::parse) >>
			tag!("?") >>
			left: call!(LValue::parse) >>
			tag!("<>") >>
			right: call!(LValue::parse)
			>> (Statement::CSwap(ctrl, left, right))
		)
		// left/right rotation, xor/cnot, increment/add, decrement/subtract
		// lval ("<<=" | ">>=" | "^=" | "+=" | "-=") factor
		| do_parse!(
			l: call!(LValue::parse) >>
			op: alt!(
				tag!("<<=") | tag!(">>=")
				| tag!("^=")
				| tag!("+=") | tag!("-=")
			) >>
			r: call!(Factor::parse)
			>> (match op {
				b"<<=" => Statement::RotLeft(l, r),
				b">>=" => Statement::RotRight(l, r),
				b"^="  => Statement::Xor(l, r),
				b"+="  => Statement::Add(l, r),
				b"-="  => Statement::Sub(l, r),
				_      => unreachable!()
			})
		)
	)));
	
	pub fn verify(&mut self) {
		match self {
			Statement::Let(_, _, typ, lit) => {
				if let Some(typ) = *typ {
					match (typ, lit) {
						(Type::Bool, Literal::Bool(_))
						| (Type::Char, Literal::Char(_))
						| (Type::U16, Literal::Num(_))
						| (Type::I16, Literal::Num(_))
						| (Type::Usize, Literal::Num(_))
						| (Type::Isize, Literal::Num(_)) => {}
						_ => panic!("literal must match type given")
					}
				}
				// The following code is best left to a type check pass
				else {
					match lit {
						Literal::Bool(_) => {typ.get_or_insert(Type::Bool);}
						Literal::Char(_) => {typ.get_or_insert(Type::Char);}
						_ => unimplemented!()
					}
				}
			}
			
			Statement::RotLeft(lval, factor)
			| Statement::RotRight(lval, factor) => {
				if let Factor::LVal(fac) = factor {
					assert!(lval.id != fac.id, "value can't modify itself");
				}
			}
			
			Statement::CCNot(lval, c0, c1) => {
				if let Factor::LVal(fac) = c0 {
					assert!(lval.id != fac.id, "value can't modify itself");
				}
				if let Factor::LVal(fac) = c1 {
					assert!(lval.id != fac.id, "value can't modify itself");
				}
			}
			
			Statement::Xor(lval, facs) => {
				for fac in facs {
					if let Factor::LVal(fac) = fac {
						assert!(lval.id != fac.id, "value can't modify itself");
					}
				}
			}
			
			Statement::Add(lval, flops)
			| Statement::Sub(lval, flops) => {
				for flop in flops {
					match flop {
						FlatOp::Add(Factor::LVal(op)) =>
							assert!(lval.id != op.id, "value can't modify itself"),
						FlatOp::Sub(Factor::LVal(op)) =>
							assert!(lval.id != op.id, "value can't modify itself"),
						_ => {}
					}
				}
			}
			
			Statement::CSwap(fac, lv0, lv1) => {
				if let Factor::LVal(fac) = fac {
					assert!(fac.id != lv0.id, "value can't modify itself");
					assert!(fac.id != lv1.id, "value can't modify itself");
				}
			}
			
			Statement::From(_, fbody, rbody, _) =>
				assert!(fbody.is_some() || rbody.is_some(), "Must provide at least 1 body."),
			
			_ => {}
		}
	}
	
	pub fn compile(&self, st: &mut SymbolTable) -> Vec<rel::Op> {
		match self {
			Statement::Not(lval) => {
				let (r, mut code) = lval.compile(st);
				code.push(Op::Not(r));
				code
			}
			Statement::Neg(lval) => {
				let (r, mut code) = lval.compile(st);
				code.push(Op::Not(r));
				code.push(Op::AddImm(r, 1));
				code
			}
			Statement::Call(..) => {
				let code = vec![];
				
				for fac in args {
					code.extend(fac.compile(st));
				}
				
				code.extend(lval.compile(st, |r| vec![
					Op::SwapPc(r),
					Op::Immediate(Reg::R1, (offset >> 8) as u8),
					Op::RotLeftImm(Reg::R1, 8),
					Op::Immediate(Reg::R1, offset as u8),
					Op::CSub(r, )
				]));
				code
				vec![Op::Nop]
			}
			Statement::Add(lval, fact) => {
				let mut code: Vec<rel::Op> = Vec::new();
				let (l, fetch_l) = lval.compile(st);
				code.extend_from_slice(&fetch_l);
				match fact {
					Factor::LVal(rval) => {
						let (r, fetch_r) = rval.compile(st);
						code.extend_from_slice(&fetch_r);
						code.push(Op::Add(l, r));
						code.extend_from_slice(&Reverse::reverse(fetch_r));
						st.ret_reg(&mut code, r);
					}
					Factor::Lit(Literal::Num(n)) => match n {
						// if we're just adding zero, nothing needs to be done
						0 => return Vec::new(),
						1...255 => {
							code.push(Op::AddImm(l, n as u8));
						}
						_ => {
							let temp = st.get_reg(&mut code);
							code.extend_from_slice(&[
								Op::XorImm(temp, (n >> 8) as u8),
								Op::LRotImm(temp, 8),
								Op::XorImm(temp, n as u8),
								Op::Add(l, temp)
							]);
							st.ret_reg(&mut code, temp);
						}
					}
					Factor::Lit(_) => panic!("what are you doing")
				}
				code.extend_from_slice(&Reverse::reverse(fetch_l));
				st.ret_reg(&mut code, l);
				code
			}
			
			Statement::Sub(lval, fact) => {
				println!("{:?}", st);
				let mut code: Vec<rel::Op> = Vec::new();
				let (l, fetch_l) = lval.compile(st);
				code.extend_from_slice(&fetch_l);
				match fact {
					Factor::LVal(rval) => {
						let (r, fetch_r) = rval.compile(st);
						code.extend_from_slice(&fetch_r);
						code.push(Op::Sub(l, r));
						code.extend_from_slice(&Reverse::reverse(fetch_r));
						st.ret_reg(&mut code, r);
					}
					Factor::Lit(Literal::Num(n)) => match n {
						// if we're just subtracting zero, nothing needs to be done
						0 => return Vec::new(),
						1...255 => {
							code.push(Op::SubImm(l, n as u8));
						}
						_ => {
							let temp = st.get_reg(&mut code);
							code.extend_from_slice(&[
								Op::XorImm(temp, (n >> 8) as u8),
								Op::LRotImm(temp, 8),
								Op::XorImm(temp, n as u8),
								Op::Sub(l, temp)
							]);
							st.ret_reg(&mut code, temp);
						}
					}
					Factor::Lit(_) => panic!("what are you doing")
				}
				code.extend_from_slice(&Reverse::reverse(fetch_l));
				st.ret_reg(&mut code, l);
				code
			}
			
			Statement::Xor(lval, fact) => {
				let mut code: Vec<rel::Op> = Vec::new();
				let (l, fetch_l) = lval.compile(st);
				code.extend_from_slice(&fetch_l);
				match fact {
					Factor::LVal(rval) => {
						let (r, fetch_r) = rval.compile(st);
						code.extend_from_slice(&fetch_r);
						code.push(Op::Xor(l, r));
						code.extend_from_slice(&Reverse::reverse(fetch_r));
						st.ret_reg(&mut code, r);
					}
					Factor::Lit(Literal::Num(n)) => match n {
						// if we're just xoring zero, nothing needs to be done
						0 => return Vec::new(),
						1...255 => {
							code.push(Op::XorImm(l, n as u8));
						}
						_ => {
							code.extend_from_slice(&[
								Op::XorImm(l, n as u8),
								Op::RRotImm(l, 8),
								Op::XorImm(l, (n >> 8) as u8),
								Op::LRotImm(l, 8),
							]);
						}
					}
					Factor::Lit(_) => panic!("what are you doing")
				}
				code.extend_from_slice(&Reverse::reverse(fetch_l));
				st.ret_reg(&mut code, l);
				code
			}
			
			Statement::RotLeft(lval, fact) => {
				let mut code: Vec<rel::Op> = Vec::new();
				let (l, fetch_l) = lval.compile(st);
				code.extend_from_slice(&fetch_l);
				match fact {
					Factor::LVal(rval) => {
						let (r, fetch_r) = rval.compile(st);
						code.extend_from_slice(&fetch_r);
						code.push(Op::LRot(l, r));
						code.extend_from_slice(&Reverse::reverse(fetch_r));
						st.ret_reg(&mut code, r);
					}
					Factor::Lit(Literal::Num(n)) => match n {
						// if we're just adding zero, nothing needs to be done
						0 => return Vec::new(),
						_ => {
							let val = n % 16;
							code.push(Op::LRotImm(l, val as u8));
						}
					}
					Factor::Lit(_) => panic!("what are you doing")
				}
				code.extend_from_slice(&Reverse::reverse(fetch_l));
				st.ret_reg(&mut code, l);
				code
			}
			
			Statement::RotRight(lval, fact) => {
				let mut code: Vec<rel::Op> = Vec::new();
				let (l, fetch_l) = lval.compile(st);
				code.extend_from_slice(&fetch_l);
				match fact {
					Factor::LVal(rval) => {
						let (r, fetch_r) = rval.compile(st);
						code.extend_from_slice(&fetch_r);
						code.push(Op::RRot(l, r));
						code.extend_from_slice(&Reverse::reverse(fetch_r));
						st.ret_reg(&mut code, r);
					}
					Factor::Lit(Literal::Num(n)) => match n {
						// if we're just adding zero, nothing needs to be done
						0 => return Vec::new(),
						_ => {
							let val = n % 16;
							code.push(Op::RRotImm(l, val as u8));
						}
					}
					Factor::Lit(_) => panic!("what are you doing")
				}
				code.extend_from_slice(&Reverse::reverse(fetch_l));
				st.ret_reg(&mut code, l);
				code
			}
			_ => vec![Op::Nop]
		}
	}
	*/
}
