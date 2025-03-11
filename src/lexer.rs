use std::vec;

#[derive(Debug)]
pub enum LexError {
    UnknownSymbol,
}
impl LexError {
    pub fn to_string(self) -> String {
        return format!("LexError: {:#?}", self);
    }
}

#[derive(Clone, Debug, Default)]
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

    // Other
    #[default]
    None,
    Unknown,
}

#[derive(Clone, Default)]
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
    pub fn to_string(&mut self) -> String {
        format!("{}: {:#?}", self.value, self.ttype)
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
                    token.ttype = TokenType::Minus;
                    i += 1;
                }
                '*' => {
                    token.ttype = TokenType::Star;
                    i += 1;
                }
                '/' => {
                    if i + 1 < code.len() && code[i + 1] == '/' {
                        while code[i] != '\n' {
                            i += 1;
                        }
                        line += 1;
                        i += 1;
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
