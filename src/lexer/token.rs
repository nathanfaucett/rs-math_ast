use core_lexer;

use super::token_value::TokenValue;
use super::token_kind::TokenKind;


pub type Token = core_lexer::Token<TokenKind, TokenValue>;
