use collections::string::String;

use core_lexer::{Reader, State, Input, TokenMeta};

use super::super::{TokenKind, TokenValue, Token};


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct NumberReader;

impl Reader<Token> for NumberReader {

    #[inline(always)]
    fn priority(&self) -> usize {
        10usize
    }

    fn read(&self, input: &Input, current: &State, next: &mut State) -> Option<Token> {
        match input.read(next) {
            Some(ch) => {
                let mut dot_read = ch == '.';
                let is_neg = if let Some(next_ch) = input.peek(next, 0) {
                    ch == '-' && next_ch.is_numeric()
                } else {
                    false
                };

                if dot_read || is_neg || ch.is_numeric() {
                    let mut string = String::new();

                    if dot_read {
                        string.push('0');
                        string.push(ch);
                    } else if is_neg {
                        string.push('-');
                        string.push(input.read(next).expect("failed to read negative number"));
                    } else {
                        string.push(ch);
                    }

                    while !input.done(next) {
                        if let Some(ch) = input.peek(next, 0) {
                            if ch == '.' {
                                if dot_read {
                                    break;
                                } else {
                                    dot_read = true;
                                    input.read(next);
                                    string.push(ch);
                                }
                            } else if ch.is_numeric() {
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
                        TokenKind::Num,
                        TokenValue::Str(string)
                    ))
                } else {
                    None
                }
            },
            None => None,
        }
    }
}
