use std::ops::Range;

pub type Span = Range<usize>;

type BoxedSpannedExpr<'a> = Spanned<Box<Expr<'a>>>;
type Ident<'a> = &'a str;

pub struct Spanned<T> {
    pub span: Span,
    pub val: T,
}

pub struct FuncDecl<'a> {
    pub name: Ident<'a>,
    pub body: BoxedSpannedExpr<'a>,
}

pub enum Expr<'a> {
    IfThenElse {
        condition: BoxedSpannedExpr<'a>,
        then_branch: BoxedSpannedExpr<'a>,
        else_branch: Option<BoxedSpannedExpr<'a>>,
    },
    UnOperation(UnOperator, BoxedSpannedExpr<'a>),
    BinOperation(BinOperator, BoxedSpannedExpr<'a>, BoxedSpannedExpr<'a>),
    Lit(Literal<'a>),
    Identifier(Ident<'a>),
}

pub enum BinOperator {
    Add,
    Sub,
    Mul,
    Div,
    EqEq,
    NotEq,
    LessEq,
    GreaterEq,
    Less,
    Greater,
    Mod,
}

pub enum UnOperator {
    Pos,
    Neg,
    Not,
}
pub enum Literal<'a> {
    Bool(bool),
    Number(&'a str),
    Str(&'a str),
}
