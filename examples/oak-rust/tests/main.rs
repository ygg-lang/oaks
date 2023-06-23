#![feature(new_range_api)]

mod builder;
mod files;
mod formatter;
mod highlighter;
mod lexer;
mod parser;

#[test]
fn ready() {
    println!("Oak Rust Compiler Ready!")
}
