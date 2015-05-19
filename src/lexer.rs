use std::char;
use regex::Regex;

pub fn lex_input(input : &mut String){

}

fn lex_number(input : String) -> (Option<LNumber>, String) {
    NUMBER_REGEX.captures(input);
}

lazy_static!{
    static ref NUMBER_REGEX : Regex = regex!(r"^(-?\d+)");
}

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