use core::fmt;


#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnOp {
    Neg,
}

impl UnOp {

    #[inline(always)]
    pub fn from_char(ch: char) -> Self {
        match ch {
            '-' => UnOp::Neg,
            _ => panic!("Invalid Unary Operation"),
        }
    }

    #[inline(always)]
    fn to_char(&self) -> char {
        match self {
            &UnOp::Neg => '-',
        }
    }
}

impl fmt::Debug for UnOp {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl fmt::Display for UnOp {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
