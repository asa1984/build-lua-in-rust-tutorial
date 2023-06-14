use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug)]
pub enum Token {
    Name(String),
    String(String),
    Eos,
}

#[derive(Debug)]
pub struct Lex {
    input: File,
}

impl Lex {
    pub fn new(input: File) -> Self {
        Lex { input }
    }

    fn read_char(&mut self) -> char {
        let mut buf: [u8; 1] = [0];
        if self.input.read(&mut buf).unwrap() == 1 {
            buf[0] as char
        } else {
            '\0'
        }
    }

    pub fn next(&mut self) -> Token {
        let ch = self.read_char();
        match ch {
            // 空白文字は読み飛ばす
            ' ' | '\r' | '\n' | '\t' => self.next(),
            // 終端文字
            '\0' => Token::Eos,
            // 文字列
            '"' => {
                let mut s = String::new();
                loop {
                    match self.read_char() {
                        '\0' => panic!("unfinishedd literal string"),
                        '"' => break,
                        ch => s.push(ch),
                    }
                }
                Token::String(s)
            }
            // Name
            'A'..='Z' | 'a'..='z' | '_' => {
                let mut name = String::new();
                name.push(ch);
                loop {
                    match self.read_char() {
                        '\0' => break,
                        '_' => name.push('_'),
                        ch if ch.is_alphabetic() => name.push(ch),
                        _ => {
                            // 1文字戻す
                            self.input.seek(SeekFrom::Current(-1)).unwrap();
                            break;
                        }
                    }
                }
                Token::Name(name)
            }
            _ => panic!("unexpected character: {ch}"),
        }
    }
}
