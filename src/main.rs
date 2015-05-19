use std::io;

mod lexer;

#[macro_use]
extern crate lazy_static;

#[feature(plugins)]
#[plugin(regex_macros)]
extern crate regex;


fn main() {
    loop{
        let line = &mut String::new();
        let _ = std::io::stdin().read_line(line);
        lexer::lex_input(line);
        println!("{:?}", line);
    }
}
