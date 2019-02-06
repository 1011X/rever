use crate::ast::*;

/*
#[derive(Debug)]
pub enum FlatOp {
	Add(Factor),
	Sub(Factor),
}
*/

#[derive(Debug)]
pub enum Statement {
	Var(bool, String, Option<Type>, Literal),
	Drop(bool, String, Option<Type>, Literal),
	
	Not(LValue),
	//Neg(LValue),
	
	RotLeft(LValue, Factor),
	RotRight(LValue, Factor),
	
	//CCNot(LValue, Factor, Factor),
	//Xor(LValue, Vec<Factor>),
	Xor(LValue, Factor),
	
	//Add(LValue, Vec<FlatOp>),
	Add(LValue, Factor),
	//Sub(LValue, Vec<FlatOp>),
	Sub(LValue, Factor),
	
	Swap(LValue, LValue),
	//CSwap(Factor, LValue, LValue),
	
	Do(LValue, Vec<Factor>),
	Undo(LValue, Vec<Factor>),
	
	If(Expr, Vec<Statement>, Option<Vec<Statement>>, Expr),
	
	From(Expr, Option<Vec<Statement>>, Option<Vec<Statement>>, Expr),
	
	//Switch(String, Vec<String, Vec<Statement>>),
	//Unsafe(Vec<Statement>),
}

use self::Statement::*;
impl Statement {
    pub fn reverse(self) -> Self {
        match self {
            Var(n, t, v) => Drop(n, t, v),
            Drop(n, t, v) => Var(n, t, v),
            
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
            
            involution => self
        }
    }
	
	pub fn eval(&self, t: &mut VarTable) {
	    match self {
	        Var(id, _, lit) => {
	            t.locals[id] = Value::from(lit);
	        }
	        Drop(id, _, lit) => {
	            assert_eq!(t.locals[id], Value::from(lit.clone()));
	            t.locals.remove(id);
	        }
	        Not(lval) => {
	            t.locals[lval.id] = match lval.eval(t) {
                    Value::Bool(b) => Value::Bool(!b),
                    Value::Int(i) => Value::Int(!i),
                };
            }
	        RotLeft(lval, fact) => match (lval.eval(t), fact.eval(t)) {
                (Value::Int(l), Value::Int(r)) =>
                    t.locals[lval.id] = Value::Int(l.rotate_left(r)),
                _ => panic!("tried to do something illegal"),
            }
            RotRight(lval, fact) => match (lval.eval(t), fact.eval(t)) {
                (Value::Int(l), Value::Int(r)) =>
                    t.locals[lval.id] = Value::Int(l.rotate_right(r)),
                _ => panic!("tried to do something illegal"),
            }
            Xor(lval, fact) => match (lval.eval(t), fact.eval(t)) {
                (Value::Int(l), Value::Int(r)) =>
                    t.locals[lval.id] = Value::Int(l ^ r),
                _ => panic!("tried to do something illegal"),
            }
            Add(lval, fact) => match (lval.eval(t), fact.eval(t)) {
                (Value::Int(l), Value::Int(r)) =>
                    t.locals[lval.id] = Value::Int(l.wrapping_add(r)),
                _ => panic!("tried to do something illegal"),
            }
            Sub(lval, fact) => match (lval.eval(t), fact.eval(t)) {
                (Value::Int(l), Value::Int(r)) =>
                    t.locals[lval.id] = Value::Int(l.wrapping_sub(r)),
                _ => panic!("tried to do something illegal"),
            }
            Swap(left, right) => {
                // TODO check types match
                let temp = left.eval(t);
                t.locals[left.id] = right.eval(t);
                t.locals[right.id] = temp;
            }
            Do(name, args) => {
                t.procedures[name.id].eval(args, t)
            }
            Undo(name, args) => {
                t.procedures[name.id].uneval(args, t)
            }
            If(test, block, else_block, assert) => {
                let result = test.eval(t);
                match test.eval(t) {
                    Value::Bool(true) => 
                        for stmt in block {
                            stmt.eval(t);
                        }
                    Value::Bool(false) =>
                        for stmt in else_block {
                            stmt.eval(t);
                        }
                    _ => panic!("tried to do something illegal")
                }
                assert_eq!(assert.eval(t), Value::Bool(true));
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
	    }
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
		// "if" binexpr block ["else" block] "fi" binexpr
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
		// left/right rotation
		// lval ("<<=" | ">>=") factor
		| do_parse!(
			l: call!(LValue::parse) >>
			op: alt!(tag!("<<=") | tag!(">>=")) >>
			r: call!(Factor::parse)
			>> (match op {
				b"<<=" => Statement::RotLeft(l, r),
				b">>=" => Statement::RotRight(l, r),
				_ => unreachable!()
			})
		)
		// xor, cnot
		// lval "^=" factor
		| do_parse!(
			l: call!(LValue::parse) >>
			tag!("^=") >>
			r: separated_nonempty_list!(
				tag!("^"),
				Factor::parse
			)
			>> (Statement::Xor(l, r))
		)
		// increment/add, decrement/subtract
		// lval ("+=" | "-=") factor
		| do_parse!(
			l: call!(LValue::parse) >>
			op: alt!(tag!("+=") | tag!("-=")) >>
			r: call!(Factor::parse) >>
			m: many0!(alt_complete!(
				map!(preceded!(tag!("+"), Factor::parse), FlatOp::Add)
				| map!(preceded!(tag!("-"), Factor::parse), FlatOp::Sub)
			))
			>> ({
				let mut m = m;
				m.insert(0, FlatOp::Add(r));
				match op {
					b"+=" => Statement::Add(l, m),
					b"-=" => Statement::Sub(l, m),
					_ => unreachable!()
				}
			})
		)
	)));
	
	pub fn verify(&mut self) {
		match *self {
			Statement::Let(_, _, ref typ, ref lit) => {
				if let Some(ref typ) = *typ {
					match (typ, lit) {
						(&Type::Bool, &Literal::Bool(_))
						| (&Type::Char, &Literal::Char(_))
						| (&Type::U16, &Literal::Num(_))
						| (&Type::I16, &Literal::Num(_))
						| (&Type::Usize, &Literal::Num(_))
						| (&Type::Isize, &Literal::Num(_)) => {}
						_ => panic!("literal must match type given")
					}
				}
				// The following code is best left to a type check pass
				else {
					match *lit {
						Literal::Bool(_) => {typ.get_or_insert(Type::Bool);}
						Literal::Char(_) => {typ.get_or_insert(Type::Char);}
						_ => unimplemented!()
					}
				}
			}
			
			Statement::RotLeft(ref lval, ref factor)
			| Statement::RotRight(ref lval, ref factor) => {
				if let Factor::LVal(ref fac) = *factor {
					assert!(lval.id != fac.id, "value can't modify itself");
				}
			}
			
			Statement::CCNot(ref lval, ref c0, ref c1) => {
				if let Factor::LVal(ref fac) = *c0 {
					assert!(lval.id != fac.id, "value can't modify itself");
				}
				if let Factor::LVal(ref fac) = *c1 {
					assert!(lval.id != fac.id, "value can't modify itself");
				}
			}
			
			Statement::Xor(ref lval, ref facs) => {
				for fac in facs {
					if let Factor::LVal(ref fac) = *fac {
						assert!(lval.id != fac.id, "value can't modify itself");
					}
				}
			}
			
			Statement::Add(ref lval, ref flops)
			| Statement::Sub(ref lval, ref flops) => {
				for flop in flops {
					match *flop {
						FlatOp::Add(Factor::LVal(ref op)) =>
							assert!(lval.id != op.id, "value can't modify itself"),
						FlatOp::Sub(Factor::LVal(ref op)) =>
							assert!(lval.id != op.id, "value can't modify itself"),
						_ => {}
					}
				}
			}
			
			Statement::CSwap(ref fac, ref lv0, ref lv1) => {
				if let Factor::LVal(ref fac) = *fac {
					assert!(fac.id != lv0.id, "value can't modify itself");
					assert!(fac.id != lv1.id, "value can't modify itself");
				}
			}
			
			Statement::From(_, ref fbody, ref rbody, _) =>
				assert!(fbody.is_some() || rbody.is_some(), "Must provide at least 1 body."),
			
			_ => {}
		}
	}
	
	pub fn compile(&self, st: &mut SymbolTable) -> Vec<rel::Op> {
		match *self {
			Statement::Not(ref lval) => {
				let (r, mut code) = lval.compile(st);
				code.push(Op::Not(r));
				code
			}
			Statement::Neg(ref lval) => {
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
			Statement::Add(ref lval, ref fact) => {
				let mut code: Vec<rel::Op> = Vec::new();
				let (l, fetch_l) = lval.compile(st);
				code.extend_from_slice(&fetch_l);
				match *fact {
					Factor::LVal(ref rval) => {
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
			
			Statement::Sub(ref lval, ref fact) => {
				println!("{:?}", st);
				let mut code: Vec<rel::Op> = Vec::new();
				let (l, fetch_l) = lval.compile(st);
				code.extend_from_slice(&fetch_l);
				match *fact {
					Factor::LVal(ref rval) => {
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
			
			Statement::Xor(ref lval, ref fact) => {
				let mut code: Vec<rel::Op> = Vec::new();
				let (l, fetch_l) = lval.compile(st);
				code.extend_from_slice(&fetch_l);
				match *fact {
					Factor::LVal(ref rval) => {
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
			
			Statement::RotLeft(ref lval, ref fact) => {
				let mut code: Vec<rel::Op> = Vec::new();
				let (l, fetch_l) = lval.compile(st);
				code.extend_from_slice(&fetch_l);
				match *fact {
					Factor::LVal(ref rval) => {
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
			
			Statement::RotRight(ref lval, ref fact) => {
				let mut code: Vec<rel::Op> = Vec::new();
				let (l, fetch_l) = lval.compile(st);
				code.extend_from_slice(&fetch_l);
				match *fact {
					Factor::LVal(ref rval) => {
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
