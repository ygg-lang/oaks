#![no_std]
#![feature(new_range_api)]

extern crate alloc;

pub mod ast;
pub mod errors;
pub mod kind;
pub mod language;
pub mod lexer;
pub mod parser;
