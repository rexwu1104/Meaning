use std::fs;

use super::token::{Token, TokenType, TokenError};

pub enum FileType {
    SourceCode,
    ByteCode
}

pub struct File {
    pub name: String,
    pub file_type: FileType, // m or mbc
    
    prev_line_length: u16,
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
            prev_line_length: 0,
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
            char::from_u32_unchecked(0xffff)
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

                            let mut hex: u32 = 0;
                            while
                                ('a' <= chr && chr <= 'f') ||
                                ('A' <= chr && chr <= 'F') ||
                                ('0' <= chr && chr <= '9') {
                                if ('a' <= chr && chr <= 'f') || ('A' <= chr && chr <= 'F') {
                                    hex = hex * 16 + u32::from_str_radix(chr.to_string().as_str(), 16).unwrap();
                                } else {
                                    hex = hex * 16 + chr.to_string().parse::<u32>().unwrap();
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

                            println!("{}", hex);

                            unsafe { char::from_u32_unchecked(hex) }
                        },
                        other => other
                    });
                } else {
                    string.push(chr);
                    chr = self.read_char();
                }
            }

            if error {
                return error_token;
            }

            return Token{
                token_type: TokenType::String(string),
                tokens: vec![]
            };
        }

        if chr == '\'' {
            chr = self.read_char();
            if chr != '\'' {
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

                            let mut hex: u32 = 0;
                            while
                                ('a' <= chr && chr <= 'f') ||
                                ('A' <= chr && chr <= 'F') ||
                                ('0' <= chr && chr <= '9') {
                                if ('a' <= chr && chr <= 'f') || ('A' <= chr && chr <= 'F') {
                                    hex = hex * 16 + u32::from_str_radix(chr.to_string().as_str(), 16).unwrap();
                                } else {
                                    hex = hex * 16 + chr.to_string().parse::<u32>().unwrap();
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

                            if chr == '}' { self.read_char(); }

                            println!("{}", hex);

                            unsafe { char::from_u32_unchecked(hex) }
                        },
                        other => other
                    });
                } else {
                    string.push(chr);
                    self.read_char();
                }
            }

            if error {
                return error_token;
            }

            return Token{
                token_type: TokenType::Char(string),
                tokens: vec![]
            };
        }

        if ('0' <= chr && chr <= '9') || chr == '.' {
            let concat_number = |s: &mut String, mode: &str, chr: &mut char, file: &mut File| {
                *chr = file.read_char();
                match mode {
                    "hex" => {
                        while
                            ('a' <= *chr && *chr <= 'f') ||
                            ('A' <= *chr && *chr <= 'F') ||
                            ('0' <= *chr && *chr <= '9') {
                                s.push(*chr);
                                
                                *chr = file.read_char();
                            }
                    },
                    "oct" => {
                        while '0' <= *chr && *chr <= '7' {
                            s.push(*chr);
                            
                            *chr = file.read_char();
                        }
                    },
                    "bin" => {
                        while '0' <= *chr && *chr <= '1' {
                            s.push(*chr);
                            
                            *chr = file.read_char();
                        }
                    },
                    "dec" => {
                        while '0' <= *chr && *chr <= '9' {
                            s.push(*chr);
                            
                            *chr = file.read_char();
                        }
                    },
                    _ => ()
                }
            };

            match chr {
                '0' => {
                    chr = self.read_char();
                    match chr {
                        'x' => concat_number(&mut string, "hex", &mut chr, self),
                        'o' => concat_number(&mut string, "oct", &mut chr, self),
                        'b' => concat_number(&mut string, "bin", &mut chr, self),
                        '0' ..= '9' => concat_number(&mut string, "dec", &mut chr, self),
                        _ => {
                            error = true;
                            error_token.tokens.push(Token{
                                token_type: TokenType::Error(TokenError{
                                    message: String::from("undefined number format"),
                                    path: self.name.clone(),
                                    position: self.position
                                }),
                                tokens: vec![]
                            })
                        }
                    }
                },
                _ => {
                    if '0' <= chr && chr <= '9' {
                        string.push(chr);
                        concat_number(&mut string, "dec", &mut chr, self);
                    }

                    if string.is_empty() && chr == '.' {
                        return Token{
                            token_type: TokenType::Operator(chr.to_string()),
                            tokens: vec![]
                        }
                    }

                    if chr != '.' {
                        return Token{
                            token_type: TokenType::Number(string),
                            tokens: vec![]
                        }
                    }

                    string.push('.');
                    concat_number(&mut string, "dec", &mut chr, self);
                }
            };

            if error {
                return error_token;
            }

            return Token{
                token_type: TokenType::Number(string),
                tokens: vec![]
            }
        }

        if ('a' <= chr && chr <= 'z') || ('A' <= chr && chr <= 'Z') || chr == '_' {
            string.push(chr);
            chr = self.read_char();
            while
                ('a' <= chr && chr <= 'z') ||
                ('A' <= chr && chr <= 'Z') ||
                ('0' <= chr && chr <= '9') ||
                chr == '_' {
                    string.push(chr);

                    chr = self.read_char();
                }

            return match string.as_str() {
                "var" | "class" |
                "u8" | "i8" |
                "u16" | "i16" |
                "u32" | "i32" | "f32" |
                "u64" | "i64" | "f64" |
                "u128" | "i128" | "f128" |
                "str" | "chr" | "bool" | "const" |
                "if" | "elif" | "else" | "match" | "when" |
                "symbol" | "function" | "attribute" |
                "for" | "while" | "loop" |
                "override" | "operator" => Token{
                    token_type: TokenType::Keyword(string),
                    tokens: vec![]
                },
                "true" | "false" => Token{
                    token_type: TokenType::Boolean(string),
                    tokens: vec![]
                },
                _ => Token{
                    token_type: TokenType::Identifier(string),
                    tokens: vec![]
                }
            };
        }

        if
            chr == '!' ||
            ('#' <= chr && chr <= '&') ||
            ('(' <= chr && chr <= '/') ||
            (':' <= chr && chr <= '@') ||
            chr == '[' ||
            (']' <= chr && chr <= '^') ||
            ('{' <= chr && chr <= '~') {
                while chr == '!' ||
                    ('#' <= chr && chr <= '&') ||
                    ('(' <= chr && chr <= '/') ||
                    (':' <= chr && chr <= '@') ||
                    chr == '[' ||
                    (']' <= chr && chr <= '^') ||
                    ('{' <= chr && chr <= '~') {
                        string.push(chr);
    
                        chr = self.read_char();
                    }

                match string.as_str() {
                    "+" | "-" | "*" | "/" | "//" | "%" | "**" |
                    ">>" | "<<" | "|" | "&" | "^" | "~" | "!" |
                    "(" | ")" | "()" | "[" | "]" | "[]" | "{" | "}" | "{}" |
                    "<" | "<=" | "==" | ">=" | ">" | "&&" | "||" | "!" |
                    ":" | ";" | "," | "." | "@" | "#" | "=" | "?" => {
                        return Token{
                            token_type: TokenType::Operator(string),
                            tokens: vec![]
                        };
                    },
                    _ => {
                        error_token.tokens.push(Token{
                            token_type: TokenType::Error(TokenError{
                                message: String::from("unexcept operator"),
                                path: self.name.clone(),
                                position: self.position
                            }),
                            tokens: vec![]
                        });

                        return error_token;
                    }
                }
            }

        if chr == unsafe {
            char::from_u32_unchecked(0xffff)
        } {
            return Token{
                token_type: TokenType::End,
                tokens: vec![]
            };
        }

        Token{
            token_type: TokenType::Space,
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
                TokenType::String(_) => Some(&token),
                TokenType::Char(_) => Some(&token),
                TokenType::Number(_) => Some(&token),
                TokenType::Other(_) => {
                    if let Some(e) = token.tokens.first() {
                        match e.token_type {
                            TokenType::Error(_) => Some(&token),
                            _ => None
                        }
                    } else {
                        Some(&token)
                    }
                },
                _ => Some(&token)
            });

            token = self.read_token();
        }

        token_vec
    }
}