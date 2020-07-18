use std::ops::Range;

pub type Span = Range<usize>;

type SpannedExpr<'a> = Spanned<Box<Expr<'a>>>;
type Ident<'a> = &'a str;

pub struct Spanned<T> {
    pub span: Span,
    pub val: T,
}

pub struct FuncDecl<'a> {
    pub name: Ident<'a>,
    pub body: SpannedExpr<'a>,
}

pub enum Expr<'a> {
    IfThenElse {
        condition: SpannedExpr<'a>,
        then_branch: SpannedExpr<'a>,
        else_branch: Option<SpannedExpr<'a>>,
    },
    UnOperation(UnOperator, SpannedExpr<'a>),
    BinOperation(BinOperator, SpannedExpr<'a>, SpannedExpr<'a>),
    Lit(Literal<'a>),
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
    Number(&'a str),
    Str(&'a str),
}
