use core::fmt;


#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl BinOp {

    #[inline(always)]
    pub fn from_char(ch: char) -> Self {
        match ch {
            '+' => BinOp::Add,
            '-' => BinOp::Sub,
            '*' => BinOp::Mul,
            '/' => BinOp::Div,
            '^' => BinOp::Pow,
            _ => panic!("Invalid Binary Operation"),
        }
    }

    #[inline(always)]
    fn to_char(&self) -> char {
        match self {
            &BinOp::Add => '+',
            &BinOp::Sub => '-',
            &BinOp::Mul => '*',
            &BinOp::Div => '/',
            &BinOp::Pow => '^',
        }
    }
}

impl fmt::Debug for BinOp {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl fmt::Display for BinOp {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
