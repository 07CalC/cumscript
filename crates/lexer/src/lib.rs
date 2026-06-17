use crate::token::Token;

mod token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    current_char: Option<char>,
    current_line: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            current_char: None,
            current_line: 1,
        };
        lexer.read_char();
        lexer
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        self.eat_whitespace();
        let token = match self.current_char {
            Some(ch) => match ch {
                '{' => Token::new(token::TokenKind::LeftBrace, self.current_line),
                '}' => Token::new(token::TokenKind::RightBrace, self.current_line),
                '(' => Token::new(token::TokenKind::LeftParen, self.current_line),
                ')' => Token::new(token::TokenKind::RightParen, self.current_line),
                ',' => Token::new(token::TokenKind::Comma, self.current_line),
                ';' => Token::new(token::TokenKind::Semicolon, self.current_line),
                '+' => Token::new(token::TokenKind::Plus, self.current_line),
                '-' => Token::new(token::TokenKind::Minus, self.current_line),
                '*' => Token::new(token::TokenKind::Asterisk, self.current_line),
                '/' => Token::new(token::TokenKind::Slash, self.current_line),
                '=' => {
                    if self.peek() == Some('=') {
                        self.read_char();
                        Token::new(token::TokenKind::DoubleEqual, self.current_line)
                    } else {
                        Token::new(token::TokenKind::Equal, self.current_line)
                    }
                }
                '!' => {
                    if self.peek() == Some('=') {
                        self.read_char();
                        Token::new(token::TokenKind::NotEqual, self.current_line)
                    } else {
                        Token::new(token::TokenKind::Bang, self.current_line)
                    }
                }
                '<' => {
                    if self.peek() == Some('=') {
                        self.read_char();
                        Token::new(token::TokenKind::LessThanOrEqual, self.current_line)
                    } else {
                        Token::new(token::TokenKind::LessThan, self.current_line)
                    }
                }
                '>' => {
                    if self.peek() == Some('=') {
                        self.read_char();
                        Token::new(token::TokenKind::GreaterThanOrEqual, self.current_line)
                    } else {
                        Token::new(token::TokenKind::GreaterThan, self.current_line)
                    }
                }
                '"' => {
                    let string_value = self.read_string();
                    Token::new(token::TokenKind::String(string_value), self.current_line)
                }
                '.' => {
                    if self.peek() == Some('.') {
                        self.read_char();
                        Token::new(token::TokenKind::DotDot, self.current_line)
                    } else {
                        Token::new(token::TokenKind::Dot, self.current_line)
                    }
                }
                ch if ch.is_alphabetic() || ch == '_' => {
                    let identifier = self.read_identifier();
                    let kind = match identifier.as_str() {
                        "let" => token::TokenKind::Let,
                        "fn" => token::TokenKind::Fn,
                        "return" => token::TokenKind::Return,
                        "if" => token::TokenKind::If,
                        "else" => token::TokenKind::Else,
                        "true" => token::TokenKind::True,
                        "false" => token::TokenKind::False,
                        "null" => token::TokenKind::Null,
                        "repeat" => token::TokenKind::Repeat,
                        "until" => token::TokenKind::Until,
                        "for" => token::TokenKind::For,
                        "in" => token::TokenKind::In,
                        _ => token::TokenKind::Identifier(identifier),
                    };
                    return Some(Token::new(kind, self.current_line));
                }
                ch if ch.is_digit(10) => {
                    let number_str = self.read_number();
                    let number_value = number_str.parse::<f64>().unwrap_or_else(|_| {
                        panic!(
                            "Invalid number format in line {}: {}",
                            self.current_line, number_str
                        )
                    });
                    return Some(Token::new(
                        token::TokenKind::Number(number_value),
                        self.current_line,
                    ));
                }
                _ => {
                    self.read_char();
                    panic!("Unexpected character in line {}: {}", self.current_line, ch);
                }
            },

            None => return None,
        };
        self.read_char();
        return Some(token);
    }

    fn read_number(&mut self) -> String {
        let mut result = String::new();
        while let Some(ch) = self.current_char {
            if ch.is_digit(10) || ch == '.' {
                if self.peek() == Some('.') && ch == '.' {
                    break;
                }
                result.push(ch);
                self.read_char();
            } else {
                break;
            }
        }
        result
    }

    fn read_identifier(&mut self) -> String {
        let mut result = String::new();
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                result.push(ch);
                self.read_char();
            } else {
                break;
            }
        }
        result
    }

    fn read_string(&mut self) -> String {
        let mut result = String::new();
        self.read_char();
        while let Some(ch) = self.current_char {
            if ch == '"' {
                break;
            }
            result.push(ch);
            self.read_char();
        }
        self.read_char();
        result
    }

    fn eat_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn peek(&self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            Some(self.input.chars().nth(self.read_position).unwrap())
        }
    }
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = Some(self.input.chars().nth(self.read_position).unwrap());
        }
        if self.current_char == Some('\n') {
            self.current_line += 1;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
}
