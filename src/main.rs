#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;

#[macro_use]
extern crate lazy_static;

mod lexer;

fn main() {
    let mut reader = std::io::stdin();
    loop{
        let mut line = String::new();
        let _ = reader.read_line(&mut line);
        lexer::lex_input(&line);
        println!("{:?}", line);
    }
}
