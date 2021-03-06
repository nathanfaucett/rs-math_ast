use alloc::vec::Vec;

use core_lexer::{self, Input};
use collection_traits::Iterable;

use super::token::Token;
use super::readers;


pub struct Lexer<I: Input> {
    lexer: core_lexer::Lexer<Token, I>,
}

impl<I: Input> Lexer<I> {

    #[inline(always)]
    fn new(input: I) -> Self {
        let mut lexer = core_lexer::Lexer::new(input);

        lexer.readers
            .add(readers::FunctionReader)
            .add(readers::IdentifierReader)
            .add(readers::NumberReader)
            .add(readers::SyntaxReader)
            .add(readers::VariableReader)
            .add(readers::WhitespaceReader)
            .sort();

        Lexer {
            lexer: lexer,
        }
    }
}

impl<I> From<I> for Lexer<I>
    where I: Input
{
    #[inline(always)]
    fn from(value: I) -> Self {
        Self::new(value)
    }
}

impl<'a, I> From<&'a I> for Lexer<Vec<char>>
    where I: Iterable<'a, &'a char>
{
    #[inline(always)]
    fn from(value: &'a I) -> Self {
        Self::new(value.iter().map(|ch| *ch).collect())
    }
}

impl<'a> From<&'a str> for Lexer<Vec<char>> {
    #[inline(always)]
    fn from(value: &'a str) -> Self {
        Self::new(value.chars().collect())
    }
}

impl<I: Input> Iterator for Lexer<I> {
    type Item = Token;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next()
    }
}


#[cfg(test)]
mod test {
    use alloc::vec::Vec;

    use super::{Lexer, Token};


    #[test]
    fn test_lexer() {
        let lexer = Lexer::from("\\frac{1}{2} + sqrt(2) * 2");
        let tokens: Vec<Token> = lexer.collect();
        assert_eq!(tokens.len(), 18);
    }
}
