use std::io::{stdin, stdout, Write};
use std::option::Option;

const INDS: i32 = 2;
const VALUES: i32 = 6;
const TT_STRING: &'static str = "STRING";
const TT_KEYWORD: &'static str = "KEYWORD";
const TT_IDENTIFIER: &'static str = "IDENTIFIER";
const TT_ERROR: &'static str = "ERROR";

const KEYWORDS: &'static [&'static str] = &[
    "print",
    "let",
];

#[derive(Debug)]
struct Position {
    idx: usize,
    ln: usize,
    col: usize,
    fn_: String,
    ftxt: String,
}

impl Position {
    fn advance(&mut self, current_char: &Option<char>) -> &mut Self {
        self.idx += 1;
        self.col += 1;

        if current_char == &Option::<char>::Some('\n') {
            self.ln += 1;
            self.col = 0;
        }

        return self;
    }
}

#[derive(Debug)]
struct Token<'a> {
    ttype: &'a str,
    value: String,
}

impl Token<'_> {

}

#[derive(Debug)]
struct Lexer {
    fn_: String,
    text: String,
    pos: Position,
    current_char: Option<char>,
}

impl Lexer {
    fn new(fn_: String, text: String) -> Self {
        Self {
            fn_: fn_.clone(),
            text: text.clone(),
            pos: Position { 
                idx: 0, 
                ln: 0, 
                col: 0, 
                fn_: fn_.clone(), 
                ftxt: text.clone(), 
            },
            current_char: if 0 > text.len() { Option::None } else { Option::<char>::Some(text.as_bytes()[0] as char) },
        }
    }

    fn advance(&mut self) {
        self.pos.advance(&self.current_char);
        self.current_char = if self.pos.idx > self.text.len() { Option::None } else { Option::<char>::Some(self.text.as_bytes()[self.pos.idx] as char) };
    }

    fn make_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while self.pos.idx + 1 < self.text.len() {
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            let mut ind: String = String::new();
            let mut tmpvalue: String = String::new();
            while i != INDS {
                if let Option::Some(value) = self.current_char {
                    ind.push(value);
                    self.advance();
                }
                i += 1;
            } 
            
            while j != VALUES {
                if let Option::Some(value) = self.current_char {
                    tmpvalue.push(value);
                    if self.pos.idx + 1 < self.text.len() {
                        self.advance();
                    }
                }
                j += 1;
            }

            let mut tmpind = from_bin(ind.clone());

            let ttype: &str = match tmpind {
                1 => TT_IDENTIFIER,
                2 => TT_STRING,
                3 => TT_KEYWORD,
                _ => TT_ERROR
            };

            let mut value = from_bin(tmpvalue.clone());
            tokens.push(Token {
                ttype,
                value: value.to_string(),
            });
        }
        return tokens;
    }
}

fn main() {
    let mut inp: String = String::new();
    loop {
        print!("$ ");
        let _ = stdout().flush();
        stdin().read_line(&mut inp).expect("Invalid input");
        if let Some('\n')=inp.chars().next_back() {
            inp.pop();
        } 
        if let Some('\r')=inp.chars().next_back() {
            inp.pop();
        }
        if inp == String::from("exit") {
            break;
        }
        else {
            let mut lexer = Lexer::new(String::from("<stdin>"), inp.clone());
            lexer.make_tokens();
        }

        inp = String::new()
    }
}

fn from_bin(bin_str: String) -> i32 {
    let bin_str = bin_str.as_str();
    let int = i32::from_str_radix(bin_str, 2).expect("not binary!");
    return int
}