use std::char;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // keywords
    Proc, Do, Undo, From, Until, Loop, If, Then, Else, Fi, Let, Var, Drop,
    // unused
    Fn, Return, Match, As, Goto, ComeFrom,
    
    // brackets
    LParen, RParen, LBracket, RBracket, LBrace, RBrace,
    
    // unary
    Not,
    
    // binary
    And, Add, Sub, Or, Xor, Swap,
    
    // rotate
    Rol, Ror,
    
    // relational
    Eq, Neq, Lt, Gt, Lte, Gte,
    
    // assign
    AddAssign, SubAssign, XorAssign, RolAssign, RorAssign,
    
    // misc
    Comma, Colon,
    
    // ident and number
    Ident(String),
    Number(String),
}


pub fn tokenize(mut s: &str) -> Result<Vec<Token>, &str> {
    let mut tokens = Vec::new();
    
    // handle identifiers and keywords
    // [_A-Za-z]
    if s.starts_with(|c| c == '_' || c.is_ascii_alphabetic()) {
        let mut i = 1;
        
        while s[i..].starts_with(|c| c == '_' || c.is_ascii_alphanumeric()) {
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
            "xor" => Token::Xor,
            
            id => Token::Ident(id.to_string())
        });
        
        s = &s[i..];
    }
    // handle numbers
    // [0-9]
    else if s.starts_with(char::is_ascii_digit) {
        let mut i = 1;
        
        while s[i..].starts_with(char::is_ascii_digit) {
            i += 1;
        }
        
        tokens.push(Token::Number(s[..i].to_string()));
        s = &s[i..];
    }
    else if s.starts_with(char::is_ascii_punctuation) {
        
    }
    // whitespace
    // [ \t\n\f\c]
    else if s.starts_with(char::is_ascii_whitespace) {
        s = &s[1..];
    }
    match_str! s {
        "(" => Token::LParen,
        ")" => Token::RParen,
        "[" => Token::LBracket,
        "]" => Token::RBracket,
        "{" => Token::LBrace,
        "}" => Token::RBrace,
        "," => Token::Comma,
        ":" => Token::Colon,
        "=" => Token::Eq,
        "^="   => Token::XorAssign,
        "!=" => Token::Neq,
        
        "+" => Token::Add,
        "+="   => Token::AddAssign,
        "-" => Token::Sub,
        "-="   => Token::SubAssign,
        
        "<" => Token::Lt,
        "<>"  => Token::Swap,
        "<="  => Token::Lte,
        "<<"  => Token::Rol,
        "<<="  => Token::RolAssign,
        
        ">" => Token::Gt,
        ">>"  => Token::Ror,
        ">="  => Token::Gte,
        ">>="  => Token::RorAssign,
    }
}
