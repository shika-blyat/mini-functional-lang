use crate::{
    ast::{BinOperator, Expr, FuncDecl, Literal, Spanned, UnOperator},
    errors::{ErrReason, Error},
    tokens::Token,
};

use std::{iter::Peekable, ops::Range};

use logos::SpannedIter;

type SpannedTok<'a> = (Token<'a>, Range<usize>);
type ParseResult<'a, T> = Result<T, Error<'a>>;
type SpannedExpr<'a> = Spanned<Expr<'a>>;

macro_rules! token {
    ($name: ident, $pattern: pat, $err_msg: literal) => {
        fn $name(&mut self) -> ParseResult<'a, SpannedTok<'a>> {
            match self.peek() {
                Some(($pattern, _)) => Ok(self.next().unwrap()),
                Some((_, span)) => Err(Error {
                    reason: ErrReason::Expected($err_msg),
                    span: span.clone(),
                }),
                None => Err(Error {
                    reason: ErrReason::Expected($err_msg),
                    span: std::usize::MAX..std::usize::MAX,
                }),
            }
        }
    };
}
struct Parser<'a> {
    tokens: Peekable<SpannedIter<'a, Token<'a>>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: SpannedIter<'a, Token<'a>>) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    fn number(&mut self) -> ParseResult<'a, SpannedExpr<'a>> {
        Ok(match self.token_number() {
            Ok((Token::Num(n), span)) => Spanned {
                span,
                val: Expr::Lit(Literal::Number(n)),
            },
            Err(e) => return Err(e),
            _ => unreachable!(),
        })
    }

    fn bool(&mut self) -> ParseResult<'a, SpannedExpr<'a>> {
        Ok(match self.token_bool() {
            Ok((Token::Bool(b), span)) => Spanned {
                span,
                val: Expr::Lit(Literal::Bool(b)),
            },
            Err(e) => return Err(e),
            _ => unreachable!(),
        })
    }

    fn string(&mut self) -> ParseResult<'a, SpannedExpr<'a>> {
        Ok(match self.token_string() {
            Ok((Token::Str(s), span)) => Spanned {
                span,
                val: Expr::Lit(Literal::Str(s)),
            },
            Err(e) => return Err(e),
            _ => unreachable!(),
        })
    }

    fn identifier(&mut self) -> ParseResult<'a, SpannedExpr<'a>> {
        Ok(match self.token_ident() {
            Ok((Token::Identifier(i), span)) => Spanned {
                span,
                val: Expr::Identifier(i),
            },
            Err(e) => return Err(e),
            _ => unreachable!(),
        })
    }

    fn next(&mut self) -> Option<SpannedTok<'a>> {
        self.tokens.next()
    }

    fn peek(&mut self) -> Option<&'_ SpannedTok<'a>> {
        self.tokens.peek()
    }

    token!(token_ident, Token::Identifier(_), "an identifier literal");
    token!(token_string, Token::Str(_), "a string literal");
    token!(token_number, Token::Num(_), "a number literal");
    token!(token_bool, Token::Bool(_), "a bool literal");
    token!(data, Token::Data, "a data token");
    token!(if_, Token::If, "an if token");
    token!(then, Token::Then, "a then token");
    token!(else_, Token::Else, "an else token");
    token!(union, Token::Union, "an union token");
    token!(equal, Token::Equal, "an equal token");
    token!(rparen, Token::RParen, "a right parenthesis");
    token!(lparen, Token::LParen, "a left parenthesis");
    token!(rbrace, Token::RBrace, "a right brace");
    token!(lbrace, Token::LBrace, "a left brace");
}
