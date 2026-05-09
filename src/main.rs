use std::io::{stdin, stdout, Write};
use std::option::Option;
use std::env;
use std::fs;

const INDS: i32 = 2;
const VALUES: i32 = 6;
const TT_STRING: &'static str = "STRING";
const TT_KEYWORD: &'static str = "KEYWORD";
const TT_IDENTIFIER: &'static str = "IDENTIFIER";
const TT_ERROR: &'static str = "ERROR";
const TT_NEWLINE: &'static str = "NEWLINE";

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

#[derive(Debug, Clone)]
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

            if let Some('\n') = self.current_char {
                tokens.push(Token {
                    ttype: TT_NEWLINE,
                    value: 0.to_string(),
                });
                self.advance();
            }
            else {
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

                let tmpind = from_bin(ind.clone());

                let ttype: &str = match tmpind {
                    1 => TT_IDENTIFIER,
                    2 => TT_STRING,
                    3 => TT_KEYWORD,
                    _ => TT_ERROR
                };

                let value = from_bin(tmpvalue.clone());
                tokens.push(Token {
                    ttype,
                    value: value.to_string(),
                });
            }
        }
        println!("{:#?}", tokens);
        return tokens;
    }
}

enum Node {
    FuncNode,
    StringNode,
    VarAssignNode,
    CallNode,
}

struct ParseResult {
    error: Option<String>,
    node: Option<Node>,
    advance_count: i32,
    to_reverse_count: i32,
}

impl ParseResult {
    fn new() -> Self {
        Self{
            error: Option::<String>::None,
            node: Option::<Node>::None,
            advance_count: 0,
            to_reverse_count: 0,
        }
    }

    fn register_advancement(&mut self) {
        self.advance_count += 1;
    }

    fn register(&mut self, res: ParseResult) -> Option<Node> {
        self.advance_count += res.advance_count;
        if res.error != None { self.error = res.error }
        return res.node
    }
}

struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    tok_idx: usize,
    current_tok: Token<'a>,
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<Token<'a>>) -> Self {
        Self {
            tokens: tokens.clone(),
            tok_idx: 0,
            current_tok: tokens[0].clone(),
        }
    }

    fn advance(&mut self) -> Token {
        self.tok_idx += 1;
        self.update_current_tok();
        return self.current_tok.clone();
    }

    fn update_current_tok(&mut self) {
        if self.tok_idx >= 0 && self.tok_idx < self.tokens.len() {
            self.current_tok = self.tokens[self.tok_idx].clone();
        }
    }

    fn parse(&self) -> ParseResult {
        let mut res = ParseResult::new();
        return res
    }
}

fn main() {
    let mut inp: String = String::new();
    let read_from_file: i32 = 1;

    if read_from_file == 0 {
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
                let tokens: Vec<Token> = lexer.make_tokens();
                let mut parser = Parser::new(tokens);
                let ast = parser.parse();
            }

            inp = String::new();
        }
    }
    else {
        let args: Vec<String> = env::args().collect();
        let file_path = &args[1];
        println!("{}", file_path);
        let file_contents = fs::read_to_string(file_path)
            .expect("file was not read!");
        let mut lexer = Lexer::new(String::from("<stdin>"), file_contents);
        let tokens: Vec<Token> = lexer.make_tokens();
        let mut parser = Parser::new(tokens);
        let ast: ParseResult = parser.parse();
    }
}

fn from_bin(bin_str: String) -> i32 {
    let bin_str = bin_str.as_str();
    let int = i32::from_str_radix(bin_str, 2).expect("not binary!");
    return int
}