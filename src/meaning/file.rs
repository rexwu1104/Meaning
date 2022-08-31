use std::fs;

use super::token::{Token, TokenType, TokenError};

pub enum FileType {
    SourceCode,
    ByteCode
}

pub struct File {
    pub name: String,
    pub file_type: FileType, // m or mbc
    
    lines: Vec<String>,
    position: (u16, u16),
    raw_data: String,
    tree: Option<Token>
}

pub struct Directory {
    pub(crate) name: String,
    pub(crate) files: Vec<File>
}

impl File {
    pub fn new(path: String, file_type: FileType) -> File {
        File{
            name: path,
            file_type,
            lines: vec![],
            position: (0, 0),
            raw_data: String::new(),
            tree: None
        }
    }
}

impl File {
    fn read(&mut self) -> () {
        self.raw_data = fs::read_to_string(self.name.as_str()).expect("file is not exsited.");
        self.lines = self.raw_data.split('\n').into_iter().map(|s| String::from(s)).collect();
    }

    unsafe fn _read_char(&mut self) -> char {
        static mut INDEX: u32 = 0;
        static mut LINE_INDEX: i16 = 0;
        static mut LINE: u16 = 0;

        if self.raw_data.len() == (INDEX + 1) as usize {
            '\u{FFFF}'
        } else {
            let chr = self.raw_data.chars().nth(INDEX as usize).unwrap();
            if chr == '\n' {
                LINE += 1;
                LINE_INDEX = -1;
            }
            self.position = (LINE, (LINE_INDEX + 1) as u16);

            LINE_INDEX += 1;
            INDEX += 1;
            chr
        }
    }

    fn read_char(&mut self) -> char {
        unsafe { self._read_char() }
    }

    fn read_token(&mut self) -> Token {
        let mut chr = self.read_char();
        let mut string = String::new();
        let mut error = false;
        let mut error_token = Token{
            token_type: TokenType::Other("error".to_string()),
            tokens: vec![]
        };

        if chr == '"' {
            chr = self.read_char();
            while chr != '"' {
                if chr == '\\' {
                    string.push(match self.read_char() {
                        '\'' => '\'',
                        '"' => '\"',
                        '\\' => '\\',
                        't' => '\t',
                        'r' => '\r',
                        'n' => '\n',
                        'b' => '\u{8}',
                        'f' => '\u{C}',
                        'u' => {
                            chr = self.read_char();
                            if chr != '{' {
                                error = true;
                                error_token.tokens.push(Token{
                                    token_type: TokenType::Error(TokenError{
                                        message: "missing '{{'".to_string(),
                                        path: self.name.clone(),
                                        position: self.position
                                    }),
                                    tokens: vec![]
                                });
                            } else {
                                chr = self.read_char();
                            }

                            let mut hex: u64 = 0;
                            while
                                ('a' <= chr && chr <= 'f') ||
                                ('A' <= chr && chr <= 'F') ||
                                ('0' <= chr && chr <= '9') {
                                if ('a' <= chr && chr <= 'f') || ('A' <= chr && chr <= 'F') {
                                    hex = hex * 16 + u64::from_str_radix(chr.to_string().as_str(), 16).unwrap();
                                } else {
                                    hex = hex * 16 + chr.to_string().parse::<u64>().unwrap();
                                }

                                chr = self.read_char();
                            }

                            if chr != '}' {
                                error = true;
                                error_token.tokens.push(Token{
                                    token_type: TokenType::Error(TokenError{
                                        message: "missing '}}'".to_string(),
                                        path: self.name.clone(),
                                        position: self.position
                                    }),
                                    tokens: vec![]
                                });
                            }

                            if chr == '}' {
                                chr = self.read_char();
                            }

                            println!("{}", char::from(hex as u8));

                            char::from(hex as u8)
                        },
                        other => other
                    });
                } else {
                    string.push(chr);
                    chr = self.read_char();
                }
            }

            println!("{}", string);
            if error {
                return error_token;
            }

            return Token{
                token_type: TokenType::String(string),
                tokens: vec![]
            };
        }

        if chr == '\u{FFFF}' {
            return Token{
                token_type: TokenType::End,
                tokens: vec![]
            };
        }

        Token{
            token_type: TokenType::Other(String::new()),
            tokens: vec![]
        }
    }
}

impl File {
    pub fn tokenize(&mut self) -> Vec<Token> {
        self.read();
        let token_vec: Vec<Token> = vec![];
        let mut token = self.read_token();
        while match token.token_type {
            TokenType::End => false,
            _ => true
        } {
            println!("{:#?}", match token.token_type {
                // TokenType::String(_) => Some(&token),
                TokenType::Other(_) => {
                    if let Some(e) = token.tokens.first() {
                        match e.token_type {
                            TokenType::Error(_) => Some(token),
                            _ => None
                        }
                    } else {
                        None
                    }
                },
                _ => None
            });

            token = self.read_token();
        }

        token_vec
    }
}