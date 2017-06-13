use core_lexer::{Reader, State, Input, TokenMeta};

use super::super::{TokenKind, TokenValue, Token};


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SyntaxReader;

impl Reader<Token> for SyntaxReader {

    #[inline(always)]
    fn priority(&self) -> usize {
        20usize
    }

    fn read(&self, input: &Input, current: &State, next: &mut State) -> Option<Token> {
        match input.read(next) {
            Some(ch) => match ch {
                '(' => Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenKind::LParen,
                    TokenValue::Chr(ch)
                )),
                ')' => Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenKind::RParen,
                    TokenValue::Chr(ch)
                )),
                '{' => Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenKind::LBracket,
                    TokenValue::Chr(ch)
                )),
                '}' => Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenKind::RBracket,
                    TokenValue::Chr(ch)
                )),
                '|' => Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenKind::Abs,
                    TokenValue::Chr(ch)
                )),
                ',' => Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenKind::Comma,
                    TokenValue::Chr(ch)
                )),
                '+' | '-' | '*' | '/' | '^' => Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenKind::Op,
                    TokenValue::Chr(ch)
                )),
                _ => None,
            },
            None => None,
        }
    }
}
