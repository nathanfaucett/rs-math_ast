use alloc::string::String;

use core_lexer::{Reader, State, Input, TokenMeta};

use super::super::{TokenKind, TokenValue, Token};


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FunctionReader;

impl Reader<Token> for FunctionReader {

    #[inline(always)]
    fn priority(&self) -> usize {
        50usize
    }

    fn read(&self, input: &Input, current: &State, next: &mut State) -> Option<Token> {
        match input.read(next) {
            Some(ch) => if ch == '\\' {
                let mut string = String::new();

                while !input.done(next) {
                    if let Some(ch) = input.peek(next, 0) {
                        if ch.is_alphanumeric() {
                            input.read(next);
                            string.push(ch);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenKind::Ident,
                    TokenValue::Str(string)
                ))
            } else {
                None
            },
            None => None,
        }
    }
}
