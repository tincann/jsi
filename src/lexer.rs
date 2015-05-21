use std::char;
use regex::Regex;

lazy_static!{
    static ref NUMBER_REGEX : Regex = regex!(r"^(-?\d+)");
}

pub fn lex_input(input : &str){
    // let x = lex_number(input);
    // println!("{:?}", x);
    let x = lex_number(input);
    println!("{:?}", x);
}

fn lex_number(input : &str) -> Option<(LNumber, usize)> {
    let digits = NUMBER_REGEX.captures(&input).and_then(|x|x.at(0));
    digits.and_then(|x|x.parse::<i32>().ok())
    .and_then(|x|(LNumber::Value(x), digits.len()))
}

#[derive(Debug)]
enum LNumber {
    Value(i32)
}

enum LOperator {
    Plus,
    Minus,
    Multiply,
    Divide
}

enum LKeyword {

}