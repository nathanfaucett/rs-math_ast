use collections::string::String;

use core_lexer::{Reader, State, Input, TokenMeta};

use super::super::{TokenKind, TokenValue, Token};


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct IdentifierReader;

impl Reader<Token> for IdentifierReader {

    #[inline(always)]
    fn priority(&self) -> usize {
        30usize
    }

    fn read(&self, input: &Input, current: &State, next: &mut State) -> Option<Token> {
        match input.read(next) {
            Some(ch) => if ch.is_alphabetic() {
                let mut string = String::new();

                string.push(ch);

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

                match string.as_str() {
                    "sqrt" | "mod" => Some(Token::new(
                        TokenMeta::new_state_meta(current, next),
                        TokenKind::Ident,
                        TokenValue::Str(string)
                    )),
                    _ => None,
                }
            } else {
                None
            },
            None => None,
        }
    }
}
