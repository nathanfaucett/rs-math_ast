#![feature(alloc)]
#![feature(collections)]
//#![no_std]
extern crate core;


extern crate alloc;
#[macro_use]
extern crate collections;

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate collection_traits;
extern crate lexer as core_lexer;


mod ast;
mod lexer;
mod parser;
mod from;


pub use self::ast::{BinOp, Expr, UnOp};
pub use self::lexer::{Lexer, TokenValue, TokenKind, Token};
pub use self::parser::Parser;
pub use self::from::{from, from_str, from_iter};
