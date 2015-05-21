#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;

#[macro_use]
extern crate lazy_static;

mod lexer;

fn main() {
    let mut reader = std::io::stdin();
    loop{
        let line = &mut String::new();
        let _ = reader.read_line(line);
        lexer::lex_input(line);
        println!("{:?}", line);
    }
}
