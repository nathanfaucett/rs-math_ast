use alloc::boxed::Box;

use core_lexer::Input;
use collection_traits::Iterable;

use super::ast::Expr;
use super::lexer::Lexer;
use super::parser::Parser;


#[inline]
pub fn from<I>(input: I) -> Option<Box<Expr>>
    where I: Input
{
    let mut lexer = Lexer::from(input);
    let mut parser = Parser::new(&mut lexer);
    parser.parse()
}

#[inline]
pub fn from_str<'a>(input: &'a str) -> Option<Box<Expr>> {
    let mut lexer = Lexer::from(input);
    let mut parser = Parser::new(&mut lexer);
    parser.parse()
}

#[inline]
pub fn from_iter<'a, I>(input: &'a I) -> Option<Box<Expr>>
    where I: Iterable<'a, &'a char>
{
    let mut lexer = Lexer::from(input);
    let mut parser = Parser::new(&mut lexer);
    parser.parse()
}


#[cfg(test)]
mod test {
    use alloc::vec::Vec;
    use super::*;


    #[test]
    fn test_from() {
        let mut vec = Vec::new();

        vec.push('|');
        vec.push('x');
        vec.push('+');
        vec.push('1');
        vec.push('|');

        assert_eq!(from(vec.clone()).unwrap().to_tex(), "|x + 1|");
        assert_eq!(from_str("|x + 1|").unwrap().to_tex(), "|x + 1|");
        assert_eq!(from_iter(&vec).unwrap().to_tex(), "|x + 1|");
    }
}
