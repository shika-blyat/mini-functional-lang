use crate::ast::Span;

pub struct Error {
    pub reason: ErrReason,
    pub span: Span,
}

pub enum ErrReason {}
