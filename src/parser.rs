use crate::tokens::Token;

use std::{iter::Peekable, ops::Range};

use logos::SpannedIter;

type SpannedTok<'a> = (Token<'a>, Range<usize>);

struct Parser<'a> {
    tokens: Peekable<SpannedIter<'a, Token<'a>>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: SpannedIter<'a, Token<'a>>) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    fn next(&mut self) -> Option<SpannedTok<'a>> {
        self.tokens.next()
    }

    fn peek(&mut self) -> Option<&'_ SpannedTok<'a>> {
        self.tokens.peek()
    }
}
