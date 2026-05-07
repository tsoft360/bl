use std::io::{stdin, stdout, Write};

const BS: i32 = 5;

#[derive(Debug, PartialEq)]
enum Option<T> {
    Some(T),
    None,
}

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
struct Lexer {
    fn_: String,
    text: String,
    pos: Position,
    current_char: Option<char>,
}

impl Lexer {
    fn new(fn_: String, text: String) -> Self {
        let mut out = Self {
            fn_: fn_.clone(),
            text: text.clone(),
            pos: Position { 
                idx: 0, 
                ln: 0, 
                col: 0, 
                fn_: fn_.clone(), 
                ftxt: text.clone(), 
            },
            current_char: Option::None,
        };

        out.current_char = if out.pos.idx < out.text.len() { Option::None } else { Option::<char>::Some(out.text.as_bytes()[out.pos.idx] as char) };

        return out;
    }

    fn advance(&mut self) {
        self.pos.advance(&self.current_char);
        self.current_char = if self.pos.idx < self.text.len() { Option::None } else { Option::<char>::Some(self.text.as_bytes()[self.pos.idx] as char) };
    }
}

fn main() {
    let mut inp: String = String::new();
    while true {
        print!("$ ");
        let _ = stdout().flush();
        stdin().read_line(&mut inp).expect("Invalid input");
        if let Some('\n')=inp.chars().next_back() {
            inp.pop();
        } 
        if let Some('\r')=inp.chars().next_back() {
            inp.pop();
        }
        if inp == "exit" {
            break;
        }
        else {
            let mut lexer = Lexer::new("<stdin>".to_string(), inp.clone());
        }
    }
}