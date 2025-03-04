use super::{AddressParserResult, token::Token};

pub struct Lexer<'a> {
    pub curr: char,
    pub prev: Option<char>,
    pub pos: usize,
    pub src: &'a str,
    pub eof: bool,
    pub error: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        let eof = src.is_empty();
        let curr = if !eof {
            src.chars().nth(0).unwrap()
        } else {
            '\0'
        };

        Self {
            curr,
            prev: None,
            pos: 0,
            src,
            eof,
            error: false,
        }
    }

    fn bump(&mut self) {
        self.pos += 1;
        if self.pos >= self.src.len() {
            self.eof = true;
            return;
        }

        self.prev = Some(self.curr);
        self.curr = self.src.chars().nth(self.pos).unwrap();
    }

    fn consume_whitespace(&mut self) {
        while self.curr.is_whitespace() {
            self.bump();
        }
    }

    pub fn next_token(&mut self) -> AddressParserResult<Token> {
        if self.eof {
            return Ok(Token::Eof);
        }

        self.consume_whitespace();

        match self.curr {
            '(' => {
                self.bump();
                Ok(Token::LParent)
            }
            ')' => {
                self.bump();
                Ok(Token::RParent)
            }
            '[' => {
                self.bump();
                Ok(Token::OpenBrackets)
            }
            ']' => {
                self.bump();
                Ok(Token::CloseBrackets)
            }
            c if c.is_ascii_hexdigit() || c.is_alphabetic() => {
                let start = self.pos;
                let start_char = self.curr;
                let mut end = start + 1;

                self.bump();

                // yes this is hex number
                if start_char == '0' && (self.curr == 'x' || self.curr == 'X') {
                    // move next
                    self.bump();
                    end += 1;

                    while !self.eof && !self.curr.is_whitespace() {
                        if self.curr == '.' {
                            // why we get . here
                            self.error = true;
                            return Err(eyre::eyre!(
                                "{}{}",
                                obfstr!("Failed parse type hex number at "),
                                self.pos
                            ));
                        }

                        self.bump();
                        end += 1;
                    }

                    // +2 for remove 0x || 0X
                    return Ok(Token::Number(isize::from_str_radix(
                        &self.src[start + 2..end],
                        16,
                    )?));
                }

                let mut is_module_name = false;
                let mut has_non_numberic = !start_char.is_ascii_digit();
                // 3 case left
                //
                // number base 10
                // symbol name: let, with, while, custume variable name
                // module name: target.dll
                while !self.eof
                    && (self.curr.is_ascii_hexdigit() // this include number
                        || self.curr.is_alphabetic() // abcd
                        || self.curr == '.')
                {
                    // dot => module
                    if !is_module_name && self.curr == '.' {
                        is_module_name = true;
                    }

                    if !has_non_numberic && !self.curr.is_ascii_digit() {
                        has_non_numberic = true;
                    }

                    self.bump();
                    end += 1;
                }

                if is_module_name {
                    return Ok(Token::ModuleSymbol(self.src[start..end].to_string()));
                }

                if !has_non_numberic {
                    return Ok(Token::Number(isize::from_str_radix(
                        &self.src[start..end],
                        10,
                    )?));
                }
                Ok(Token::Symbol(self.src[start..end].to_string()))
            }
            '+' => {
                self.bump();
                Ok(Token::Add)
            }

            '-' => {
                self.bump();
                Ok(Token::Sub)
            }

            '*' => {
                self.bump();
                Ok(Token::Mul)
            }

            '/' => {
                self.bump();
                Ok(Token::Div)
            }

            '^' => {
                self.bump();
                Ok(Token::Pow)
            }

            '=' => {
                self.bump();
                Ok(Token::Equals)
            }

            token => {
                self.error = true;
                Err(eyre::eyre!(
                    "{} {token} at {}",
                    obfstr!("Unknown token while parse"),
                    self.pos
                ))
            }
        }
    }
}

impl<'a> std::fmt::Display for Lexer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.curr)
    }
}
