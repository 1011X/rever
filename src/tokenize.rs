use std::char;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // keywords
    Proc, Do, Undo, From, Until, Loop, If, Then, Else, Fi, Let, Var, Drop,
    // unused
    Fn, Return, Match, As, //Goto, ComeFrom,
    
    // brackets
    LParen, RParen, LBracket, RBracket, LBrace, RBrace,
    
    // single purpose
    Not, And, Or, Swap,
    // relational
    Neq, Lt, Gt,
    
    // multi-purpose
    Eq, Comma, Colon, Caret, Add, Sub, Rol, Ror,
    
    Space(usize),
    Comment(String),
    
    // ident and number
    Ident(String),
    Number(String),
    //String(String),
}

/*
order of ops:
1. parens
2. function call
3. - ~ @ ! not
4. ** << >> shl shr
5. * / & div mod and
6. + - | ^ or
7. = != < > <= >=
*/


pub fn tokenize(mut s: &str) -> Result<Vec<Token>, &str> {
    let mut tokens = Vec::new();
    
    // handle identifiers and keywords
    // [_A-Za-z]
    if s.starts_with(|c: char| c == '_' || c.is_ascii_alphabetic()) {
        let mut i = 1;
        
        while s[i..].starts_with(|c: char| c == '_' || c.is_ascii_alphanumeric()) {
            i += 1;
        }
        
        tokens.push(match &s[..i] {
            // keywords
            "proc"  => Token::Proc,
            "do"    => Token::Do,
            "undo"  => Token::Undo,
            "from"  => Token::From,
            "until" => Token::Until,
            "loop"  => Token::Loop,
            "if"    => Token::If,
            "then"  => Token::Then,
            "else"  => Token::Else,
            "fi"    => Token::Fi,
            "let"   => Token::Let,
            "var"   => Token::Var,
            "drop"  => Token::Drop,
            
            "fn"    => Token::Fn,
            "return" => Token::Return,
            "match" => Token::Match,
            "as"    => Token::As,
            
            // operations
            "not" => Token::Not,
            
            "and" => Token::And,
            "or"  => Token::Or,
            //"xor" => Token::Xor,
            
            id => Token::Ident(id.to_string())
        });
        
        s = &s[i..];
    }
    // handle numbers
    // [0-9]
    else if s.starts_with(|c: char| c.is_ascii_digit()) {
        let mut i = 1;
        
        while s[i..].starts_with(|c: char| c.is_ascii_digit()) {
            i += 1;
        }
        
        tokens.push(Token::Number(s[..i].to_string()));
        s = &s[i..];
    }
    else if s.starts_with(|c: char| c.is_ascii_punctuation()) {
        
    }
    // whitespace
    // [ \t\n\f\c]
    else if s.starts_with(|c: char| c.is_ascii_whitespace()) {
        s = &s[1..];
    }
    else if s.starts_with(|c: char| c.is_ascii_punctuation()) {
        tokens.push(match &s[..1] {
            "(" => Token::LParen,
            ")" => Token::RParen,
            "[" => Token::LBracket,
            "]" => Token::RBracket,
            "{" => Token::LBrace,
            "}" => Token::RBrace,
            "," => Token::Comma,
            ":" => Token::Colon,
            "=" => Token::Eq,
            "!" => match &s[1..2] {
                "=" => Token::Neq,
                _ => return Err("!=")
            }
            "+" => Token::Add,
            "-" => Token::Sub,
            
            "<" => Token::Lt,
            /*
            "<" => match &s[1..2] {
                ">" => Token::Swap,
                "=" => Token::Lte,
                _ => Err("")
            }
            */
            //"<<" => Token::Rol,
            
            ">" => Token::Gt,
            //">>" => Token::Ror,
            //">=" => Token::Gte,
            
            _ => return Err("unrecognized symbol")
        });
    }
    
    Ok(tokens)
}
