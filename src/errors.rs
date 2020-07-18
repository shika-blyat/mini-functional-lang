use crate::ast::Span;

pub struct Error<'a> {
    pub reason: ErrReason<'a>,
    pub span: Span,
}

pub enum ErrReason<'a> {
    Expected(&'a str),
}
