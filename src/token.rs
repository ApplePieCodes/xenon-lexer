use std::fmt::Display;

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
    PlusEquals,
    MinusEquals,
    TimesEquals,
    DivideEquals,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Arrow,
    Colon,
    Bang,
    ShBang,
    And,
    GetRef,
    Or,
    Dot,
    Comma,
    Break,
    Continue,

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
    pub fn new() -> Token {
        Token {
            value: String::new(),
            ttype: TokenType::None,
            line: 0,
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token at line {}: Type {:#?} with value {}",
            self.line, self.ttype, self.value
        )
        .expect("Should pass a formatter");
        Ok(())
    }
}
