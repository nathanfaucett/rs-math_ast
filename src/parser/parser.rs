use alloc::boxed::Box;

use collections::vec::Vec;

use core_lexer::Input;

use super::super::{Token, TokenValue, TokenKind, Lexer, BinOp, Expr};


static END_OF_INPUT: &'static str = "Unexpected End of Input";


#[derive(Clone)]
struct State {
    index: usize,
}

impl State {

    #[inline(always)]
    fn new() -> Self {
        State {
            index: 0,
        }
    }

    fn consume(&mut self, offset: usize) -> &mut Self {
        self.index += offset;
        self
    }
}


pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {

    #[inline(always)]
    pub fn new<I: Input>(lexer: &mut Lexer<I>) -> Self {
        Parser {
            tokens: lexer.filter(|ref token| token.kind() != &TokenKind::Whitespace).collect(),
        }
    }

    #[inline(always)]
    fn next_token(&self, state: &mut State) -> Option<&Token> {
        let token = self.peek_token(state, 0);
        state.consume(1);
        token
    }

    #[inline(always)]
    fn peek_token(&self, state: &mut State, offset: usize) -> Option<&Token> {
        self.tokens.get(state.index + offset)
    }

    #[inline]
    fn parse_comma_args(&self, state: &mut State) -> Vec<Box<Expr>> {
        let mut args = Vec::new();

        loop {
            args.push(self.parse_expr(state).expect(END_OF_INPUT));

            match self.peek_token(state, 0) {
                Some(token) => match token.kind() {
                    &TokenKind::Comma => {
                        state.consume(1);
                    },
                    &TokenKind::RParen => {
                        state.consume(1);
                        break;
                    },
                    _ => break,
                },
                None => break,
            }
        }

        args
    }

    #[inline]
    fn parse_args(&self, state: &mut State) -> Vec<Box<Expr>> {
        let mut args = Vec::new();

        loop {
            args.push(self.parse_expr(state).expect(END_OF_INPUT));

            match self.peek_token(state, 0) {
                Some(token) => if token.kind() == &TokenKind::RBracket {
                    state.consume(1);

                    match self.peek_token(state, 0) {
                        Some(token) => if token.kind() == &TokenKind::LBracket {
                            state.consume(1);
                        } else {
                            break;
                        },
                        None => break,
                    }
                } else {
                    break;
                },
                None => break,
            }
        }

        args
    }

    #[inline]
    fn parse_primary_expr(&self, state: &mut State) -> Option<Box<Expr>> {
        match self.next_token(state) {
            Some(token) => match token.kind() {
                &TokenKind::Num => match token.value() {
                    &TokenValue::Str(ref string) => Some(Box::new(Expr::Num(string.clone()))),
                    _ => None,
                },
                &TokenKind::Ident => match token.value() {
                    &TokenValue::Str(ref name) => match self.next_token(state) {
                        Some(token) => Some(Box::new(Expr::Func(
                            name.clone(),
                            match token.kind() {
                                &TokenKind::LParen => self.parse_comma_args(state),
                                &TokenKind::LBracket => self.parse_args(state),
                                _ => Vec::new(),
                            }
                        ))),
                        None => None,
                    },
                    _ => None,
                },
                &TokenKind::LParen => {
                    let expr = self.parse_expr(state);

                    match self.next_token(state) {
                        Some(token) => match token.kind() {
                            &TokenKind::RParen => expr,
                            _ => None,
                        },
                        None => None,
                    }
                },
                &TokenKind::Abs => {
                    let expr = self.parse_expr(state);

                    match self.next_token(state) {
                        Some(token) => match token.kind() {
                            &TokenKind::Abs => expr,
                            _ => None,
                        },
                        None => None,
                    }
                },
                _ => None,
            },
            None => None,
        }
    }

    #[inline(always)]
    fn parse_mul_expr(&self, state: &mut State) -> Option<Box<Expr>> {
        let mut expr = self.parse_primary_expr(state).expect(END_OF_INPUT);

        loop {
            match self.peek_token(state, 0) {
                Some(token) => match token.kind() {
                    &TokenKind::Op => match token.value() {
                        &TokenValue::Chr(ch) => if ch == '^' || ch == '*' || ch == '/' {
                            state.consume(1);
                            let rhs = self.parse_mul_expr(state).expect(END_OF_INPUT);
                            let lhs = expr;
                            expr = Box::new(Expr::BinOp(BinOp::from_char(ch), lhs, rhs));
                        } else {
                            break;
                        },
                        _ => break,
                    },
                    _ => break,
                },
                None => break,
            }
        }

        Some(expr)
    }

    #[inline(always)]
    fn parse_expr(&self, state: &mut State) -> Option<Box<Expr>> {
        let mut expr = self.parse_mul_expr(state).expect(END_OF_INPUT);

        loop {
            match self.peek_token(state, 0) {
                Some(token) => match token.kind() {
                    &TokenKind::Op => match token.value() {
                        &TokenValue::Chr(ch) => if ch == '+' || ch == '-' {
                            state.consume(1);
                            let rhs = self.parse_mul_expr(state).expect(END_OF_INPUT);
                            let lhs = expr;
                            expr = Box::new(Expr::BinOp(BinOp::from_char(ch), lhs, rhs));
                        } else {
                            break;
                        },
                        _ => break,
                    },
                    _ => break,
                },
                None => break,
            }
        }

        Some(expr)
    }

    #[inline(always)]
    pub fn parse(&mut self) -> Option<Box<Expr>> {
        self.parse_expr(&mut State::new())
    }
}


#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_parse() {
        let mut lexer = Lexer::from("\\frac{1}{2} + sqrt(2) * 2");
        let mut parser = Parser::new(&mut lexer);

        match parser.parse() {
            Some(ast) => assert_eq!(ast.to_string(), "\\frac{1}{2} + \\sqrt{2} * 2"),
            None => panic!("failed to parse tex"),
        }
    }
}
