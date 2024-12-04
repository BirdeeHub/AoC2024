use std::fs::File;
use std::time::Instant;
use std::io::{self, Read};

fn read_file(file_path: &str) -> io::Result<String> {
    let mut contents = String::new();
    File::open(file_path)?.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> io::Result<()> {
    let start = Instant::now();
    let filestr = read_file("input")?;

    let mut parser = Parser::new(filestr);
    let output = parser.parse();
    let res = output.iter().map(|(k,v)|k*v).sum::<i32>();
    println!("{}", res);
    println!("Time taken: {:?}", start.elapsed());

    Ok(())
}

struct Parser {
    input: String,
    pos:usize,
}

impl Parser {
    fn new(input: String) -> Parser {
        Parser { input, pos: 0 }
    }
    fn get_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }
    fn advance(&mut self) {
        self.pos += self.get_char().unwrap_or_default().len_utf8();
    }
    fn parse(&mut self) -> Vec<(i32,i32)> {
        let mut output:Vec<(i32,i32)> = Vec::new();
        while let Some(c) = self.get_char() {
            match c {
                'd' => {
                    self.consume_dont();
                },
                'm' => {
                    self.consume_mult(&mut output);
                },
                _ => { self.advance(); },
            };
        };
        output
    }
    fn consume_dont(&mut self) {
        let start = self.pos;
        let startchar = "don't()";
        while self.get_char().is_some() {
            let current = &self.input[start..self.pos];
            if startchar == current {
                break;
            }
            if ! startchar.starts_with(current) {
                return;
            }
            self.advance();
        }
        let endchar = "do()";
        while self.get_char().is_some() {
            let remaining = &self.input[self.pos..];
            if remaining.starts_with(endchar) {
                let mut count = 0;
                while count < endchar.len() {
                    self.advance();
                    count += 1;
                }
                break;
            }
            self.advance();
        }
    }
    fn consume_mult(&mut self, output: &mut Vec<(i32, i32)>) {
        let start = self.pos;
        let mul = "mul(";
        while self.get_char().is_some() {
            let current = &self.input[start..self.pos];
            if mul == current {
                break;
            }
            if ! mul.starts_with(current) {
                return;
            }
            self.advance();
        }
        let mut firststr = String::new();
        while let Some(c) = self.get_char() {
            if !c.is_ascii_digit() {
                break;
            }
            if firststr.len() == 3 {
                break;
            }
            firststr.push(c);
            self.advance();
        }
        match self.get_char() {
            Some(c) if c != ',' => {
                return;
            },
            _ => {
                self.advance();
            }
        }
        let mut secondstr = String::new();
        while let Some(c) = self.get_char() {
            if !c.is_ascii_digit() {
                break;
            }
            if secondstr.len() == 3 {
                break;
            }
            secondstr.push(c);
            self.advance();
        }
        match self.get_char() {
            Some(c) if c != ')' => {
                return;
            },
            _ => {
                self.advance();
            }
        }
        output.push((firststr.parse().unwrap(), secondstr.parse().unwrap()));
    }
}
