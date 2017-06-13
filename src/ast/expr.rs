use alloc::boxed::Box;

use collections::vec::Vec;

use core::fmt;

use super::bin_op::BinOp;
use super::un_op::UnOp;


#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Expr {

    Group(char, char, Vec<Box<Expr>>),

    BinOp(BinOp, Box<Expr>, Box<Expr>),
    UnOp(UnOp, Box<Expr>),

    Num(String),
    Var(String),

    Func(String, Vec<Box<Expr>>),
}

impl fmt::Debug for Expr {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Expr::Group(ref open, ref close, ref exprs) => {
                write!(f, "{}", open)?;
                for expr in exprs.iter() {
                    write!(f, "{:?}", expr)?;
                }
                write!(f, "{}", close)
            },

            &Expr::BinOp(ref bin_op, ref lhs, ref rhs) => write!(f, "{:?} {} {:?}", lhs, bin_op, rhs),
            &Expr::UnOp(ref un_op, ref expr) => write!(f, "{}{:?}", un_op, expr),

            &Expr::Num(ref num) => write!(f, "{}", num),
            &Expr::Var(ref sym) => write!(f, "{}", sym),

            &Expr::Func(ref sym, ref exprs) => {
                write!(f, "\\{}", sym)?;
                for expr in exprs.iter() {
                    write!(f, "{{{:?}}}", expr)?;
                }
                write!(f, "")
            },
        }
    }
}

impl fmt::Display for Expr {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
