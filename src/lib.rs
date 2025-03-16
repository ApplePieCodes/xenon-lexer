use std::{fmt::Display, vec};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum LexError {
    UnknownSymbol,
    UnexpectedEof,
    #[default]
    Unknown
}
impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexError::UnknownSymbol => write!(f, "Unknown symbol encountered").expect("Should pass a formatter"),
            LexError::UnexpectedEof => write!(f, "Unexpected End-of-File").expect("Should pass a formatter"),
            LexError::Unknown => write!(f, "Unknown Error").expect("Should pass a formatter")
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    // Literals
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    Identifier,

    // Keywords
    True,
    False,
    Return,
    Public,
    Private,
    Module,
    Fn,
    Let,
    If,
    Else,
    While,
    For,
    Loop,
    Struct,
    Impliment,
    Enum,
    Unsafe,
    ASM,
    Trait,
    Switch,
    Async,


    // Symbols
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenCurly,
    CloseCurly,
    Semicolon,

    // Operators
    Plus,
    Minus,
    Star,
    Divide,
    Equals,
    EqualsEquals,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Arrow,
    Colon,
    Bang,
    ShBang,
    And,
    Or,

    // Other
    #[default]
    None,
    Unknown,
}

#[derive(Clone, Default, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct Token {
    pub value: String,
    pub ttype: TokenType,
    pub line: usize,
}
impl Token {
    fn new() -> Token {
        Token {
            value: String::new(),
            ttype: TokenType::None,
            line: 0,
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token at line {}: Type {:#?} with value {}", self.line, self.ttype, self.value).expect("Should pass a formatter");
        Ok(())
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, LexError> {
    let code: Vec<char> = input.chars().collect();
    let mut tokens: Vec<Token> = vec![];
    let mut i: usize = 0;
    let mut line: usize = 1;
    let mut buffer: String = String::new();

    while i < code.len() {
        let mut token = Token::new();
        token.line = line;

        if code[i].is_alphabetic() || code[i] == '_' {
            buffer.push(code[i]);
            i += 1;
            while i < code.len() && (code[i].is_alphanumeric() || code[i] == '_') {
                buffer.push(code[i]);
                i += 1;
            }
            match buffer.clone().as_str() {
                "true" => {
                    token.ttype = TokenType::True;
                    token.value = buffer.clone();
                }
                "false" => {
                    token.ttype = TokenType::False;
                    token.value = buffer.clone();
                }
                "return" => {
                    token.ttype = TokenType::Return;
                    token.value = buffer.clone();
                }
                "public" => {
                    token.ttype = TokenType::Public;
                    token.value = buffer.clone();
                }
                "private" => {
                    token.ttype = TokenType::Private;
                    token.value = buffer.clone();
                }
                "module" => {
                    token.ttype = TokenType::Module;
                    token.value = buffer.clone();
                }
                "let" => {
                    token.ttype = TokenType::Let;
                    token.value = buffer.clone();
                }
                "fn" => {
                    token.ttype = TokenType::Fn;
                    token.value = buffer.clone();
                }
                "if" => {
                    token.ttype = TokenType::If;
                    token.value = buffer.clone();
                }
                "else" => {
                    token.ttype = TokenType::Else;
                    token.value = buffer.clone();
                }
                "while" => {
                    token.ttype = TokenType::While;
                    token.value = buffer.clone();
                }
                "for" => {
                    token.ttype = TokenType::For;
                    token.value = buffer.clone();
                }
                "loop" => {
                    token.ttype = TokenType::Loop;
                    token.value = buffer.clone();
                }
                "struct" => {
                    token.ttype = TokenType::Struct;
                    token.value = buffer.clone();
                }
                "impliment" => {
                    token.ttype = TokenType::Impliment;
                    token.value = buffer.clone();
                }
                "enum" => {
                    token.ttype = TokenType::Enum;
                    token.value = buffer.clone();
                }
                "unsafe" => {
                    token.ttype = TokenType::Unsafe;
                    token.value = buffer.clone();
                }
                "asm" => {
                    token.ttype = TokenType::ASM;
                    token.value = buffer.clone();
                }
                "trait" => {
                    token.ttype = TokenType::Trait;
                    token.value = buffer.clone();
                }
                "switch" => {
                    token.ttype = TokenType::Switch;
                    token.value = buffer.clone();
                }
                "async" => {
                    token.ttype = TokenType::Async;
                    token.value = buffer.clone();
                }
                _ => {
                    token.ttype = TokenType::Identifier;
                    token.value = buffer.clone();
                }
            }
        } else if code[i].is_numeric() {
            buffer.push(code[i]);
            i += 1;
            while i < code.len() && (code[i].is_numeric() || code[i] == '.') {
                buffer.push(code[i]);
                i += 1;
            }
            if buffer.contains('.') {
                token.ttype = TokenType::FloatLiteral;
            } else {
                token.ttype = TokenType::IntegerLiteral;
            }
            token.value = buffer.clone();
        } else if code[i] == '"' {
            i += 1;
            while i < code.len() && code[i] != '"' {
                buffer.push(code[i]);
                i += 1;
            }
            i += 1;
            token.ttype = TokenType::StringLiteral;
            token.value = buffer.clone();
        } else if code[i] == '\n' {
            line += 1;
            i += 1;
            continue;
        } else if code[i].is_whitespace() {
            i += 1;
            continue;
        } else {
            match code[i] {
                '(' => {
                    token.ttype = TokenType::OpenParen;
                    i += 1;
                }
                ')' => {
                    token.ttype = TokenType::CloseParen;
                    i += 1;
                }
                '[' => {
                    token.ttype = TokenType::OpenBracket;
                    i += 1;
                }
                ']' => {
                    token.ttype = TokenType::CloseBracket;
                    i += 1;
                }
                '{' => {
                    token.ttype = TokenType::OpenCurly;
                    i += 1;
                }
                '}' => {
                    token.ttype = TokenType::CloseCurly;
                    i += 1;
                }
                ';' => {
                    token.ttype = TokenType::Semicolon;
                    i += 1;
                }
                '+' => {
                    token.ttype = TokenType::Plus;
                    i += 1;
                }
                '-' => {
                    if i + 1 < code.len() && code[i + 1] == '>' {
                        token.ttype = TokenType::Arrow;
                        i += 2;
                    } else {
                        token.ttype = TokenType::Minus;
                        i += 1;
                    }
                }
                '*' => {
                    token.ttype = TokenType::Star;
                    i += 1;
                }
                '/' => {
                    if i + 1 < code.len() && code[i + 1] == '/' {
                        while i < code.len() && code[i] != '\n' {
                            i += 1;
                        }
                        line += 1;
                        i += 1;
                        continue;
                    }
                    else if i + 1 < code.len() && code[i + 1] == '*' {
                        i+=2;
                        loop {
                            if i < code.len() && code[i] != '*' {
                                i+=1;
                                continue;
                            }
                            else {
                                if i + 1 >= code.len() {
                                    return Err(LexError::UnexpectedEof);
                                }
                                else {
                                    if code[i + 1] == '/' {
                                        i+=2;
                                        break;
                                    }
                                }
                            }
                        }
                        continue;
                    }
                    token.ttype = TokenType::Divide;
                    i += 1;
                }
                '=' => {
                    if i + 1 < code.len() && code[i + 1] == '=' {
                        token.ttype = TokenType::EqualsEquals;
                        i += 2;
                    } else {
                        token.ttype = TokenType::Equals;
                        i += 1;
                    }
                }
                '<' => {
                    if i + 1 < code.len() && code[i + 1] == '=' {
                        token.ttype = TokenType::LessEqual;
                        i += 2;
                    } else {
                        token.ttype = TokenType::Less;
                        i += 1;
                    }
                }
                '>' => {
                    if i + 1 < code.len() && code[i + 1] == '=' {
                        token.ttype = TokenType::GreaterEqual;
                        i += 2;
                    } else {
                        token.ttype = TokenType::Greater;
                        i += 1;
                    }
                }
                '#' => {
                    if i + 1 < code.len() && code[i + 1] == '!' {
                        token.ttype = TokenType::ShBang;
                        i += 2;
                    }
                    else {
                        return Result::Err(LexError::UnknownSymbol);
                    }
                }
                '!' => {
                    token.ttype = TokenType::Bang;
                    i += 1;
                }
                '&' => {
                    if i + 1 < code.len() && code[i + 1] == '&' {
                        token.ttype = TokenType::And;
                        i += 2;
                    }
                    else {
                        return Result::Err(LexError::UnknownSymbol);
                    }
                }
                '|' => {
                    if i + 1 < code.len() && code[i + 1] == '|' {
                        token.ttype = TokenType::Or;
                        i += 2;
                    }
                    else {
                        return Result::Err(LexError::UnknownSymbol);
                    }
                }
                ':' => {
                    token.ttype = TokenType::Colon;
                    i += 1;
                }
                _ => {
                    return Result::Err(LexError::UnknownSymbol);
                }
            }
        }

        buffer.clear();
        tokens.push(token.clone());
    }

    Result::Ok(tokens)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn xenon_code() {
        let tokens = tokenize(
            "void main() {
                        let i = 0;
                        // i am a comment
                        /* I am a multi-line comment*/
                        return 0;
                    }",
        );

        match tokens.clone() {
            Ok(t) => assert!(t.len() == 14),
            Err(e) => panic!("{}", e.to_string()),
        }
        let boxed_arr: Box<[Token]> = tokens.clone().unwrap().try_into().unwrap();
        assert!(boxed_arr[0] == Token { value: "void".to_string(), ttype: TokenType::Identifier, line: 1});
        assert!(boxed_arr[1] == Token { value: "main".to_string(), ttype: TokenType::Identifier, line: 1});
        assert!(boxed_arr[2] == Token { value: "".to_string(), ttype: TokenType::OpenParen, line: 1});
        assert!(boxed_arr[3] == Token { value: "".to_string(), ttype: TokenType::CloseParen, line: 1});
        assert!(boxed_arr[4] == Token { value: "".to_string(), ttype: TokenType::OpenCurly, line: 1});
        assert!(boxed_arr[5] == Token { value: "let".to_string(), ttype: TokenType::Let, line: 2});
        assert!(boxed_arr[6] == Token { value: "i".to_string(), ttype: TokenType::Identifier, line: 2});
        assert!(boxed_arr[7] == Token { value: "".to_string(), ttype: TokenType::Equals, line: 2});
        assert!(boxed_arr[8] == Token { value: "0".to_string(), ttype: TokenType::IntegerLiteral, line: 2});
        assert!(boxed_arr[9] == Token { value: "".to_string(), ttype: TokenType::Semicolon, line: 2});
        assert!(boxed_arr[10] == Token { value: "return".to_string(), ttype: TokenType::Return, line: 5});
        assert!(boxed_arr[11] == Token { value: "0".to_string(), ttype: TokenType::IntegerLiteral, line: 5});
        assert!(boxed_arr[12] == Token { value: "".to_string(), ttype: TokenType::Semicolon, line: 5});
        assert!(boxed_arr[13] == Token { value: "".to_string(), ttype: TokenType::CloseCurly, line: 6       });
    }

    #[test]
    fn unknown_symbol() {
        let tokens = tokenize("^");

        assert!(tokens.is_err());
    }
}
