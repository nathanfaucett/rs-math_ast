

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Whitespace,

    Op,

    LParen,
    RParen,

    LBracket,
    RBracket,

    Abs,

    Comma,

    Ident,
    Var,
    Num,
}
