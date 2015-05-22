use std::char;
use regex::Regex;

lazy_static!{
    static ref NUMBER_REGEX : Regex = regex!(r"^(-?\d+)");
}

pub fn lex_input(input : &str){
    // let x = lex_number(input);
    // println!("{:?}", x);
    let a =  lex_number(input);
    println!("{:?}", a);
}

fn lex_number(input : &str) -> Option<(LNumber, usize)> {
    let digits = NUMBER_REGEX.captures(&input).and_then(|x|x.at(0));
    digits.and_then(|x|x.parse::<i32>().ok())
    .map(|x|(LNumber::Value(x), digits.unwrap().len()))
}


fn lex_token<'a, T>(input : &'a str) -> Option<TokenResult<T>> where T: Parse<T> {
    input.from_string()
        .map(|t| (t, input)) //todo rest van input
}

fn parseToken<T>(input : &str) -> Option<T> where T: Parse<T> {
}


trait Parse<T> {
    fn from_string() -> T;
}


//todo use phf instead
type TokenMap<'a, T> = Fn(&'a str) -> Option<T>;

type TokenResult<'a, T> = (T, &'a str);

#[derive(Debug)]
enum LNumber {
    Value(i32)
}

#[derive(Debug)]
enum LOperator {
    Plus,
    Minus,
    Multiply,
    Divide
}

#[derive(Debug)]
enum LKeyword {

}