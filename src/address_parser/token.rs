#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LParent,
    RParent,
    OpenBrackets,
    CloseBrackets,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Equals,
    Number(isize),
    // sin cos
    Symbol(String),
    ModuleSymbol(String),
    Eof,
}

impl Token {
    pub fn is_eof(&self) -> bool {
        matches!(self, &Token::Eof)
    }

    // not understand
    pub fn info(&self) -> Option<(usize, usize)> {
        match self {
            Token::Add | Token::Sub => Some((10, 0)),
            Token::Mul | Token::Div => Some((20, 0)),
            Token::Pow => Some((30, 1)),
            _ => None,
        }
    }
}

impl Into<char> for Token {
    fn into(self) -> char {
        match self {
            Token::LParent => '(',
            Token::RParent => ')',
            Token::OpenBrackets => '[',
            Token::CloseBrackets => ']',
            Token::Add => '+',
            Token::Sub => '-',
            Token::Mul => '*',
            Token::Div => '/',
            Token::Pow => '^',
            Token::Equals => '=',
            Token::Number(_) => 'N',
            Token::Symbol(_) => 'S',
            Token::ModuleSymbol(_) => 'M',
            Token::Eof => 'E',
        }
    }
}

impl Into<char> for &Token {
    fn into(self) -> char {
        match self {
            Token::LParent => '(',
            Token::RParent => ')',
            Token::OpenBrackets => '[',
            Token::CloseBrackets => ']',
            Token::Add => '+',
            Token::Sub => '-',
            Token::Mul => '*',
            Token::Div => '/',
            Token::Pow => '^',
            Token::Equals => '=',
            Token::Number(_) => 'N',
            Token::Symbol(_) => 'S',
            Token::ModuleSymbol(_) => 'M',
            Token::Eof => 'E',
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = self.into();
        write!(f, "{c}")
    }
}
