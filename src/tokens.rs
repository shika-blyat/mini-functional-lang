use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token<'a> {
    #[regex(r"[A-z_]\w*")]
    Identifier(&'a str),

    #[regex(r#"".*""#)]
    Str(&'a str),

    #[regex("[0-9]+")]
    Num(&'a str),

    #[regex("true|false", |lex| lex.slice().parse::<bool>())]
    Bool(bool),

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("if")]
    If,

    #[token("then")]
    Then,

    #[token("else")]
    Else,

    #[token("data")]
    Data,

    #[token("=")]
    Equal,

    #[token("|")]
    Union,

    #[regex(r"[\+\-\*/%(!=)(==)(>=)(<=)<>!]")]
    Op(&'a str),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}
