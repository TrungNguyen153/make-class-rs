use super::{AddressParserResult, ast::Node, lexer::Lexer, token::Token};

pub struct AddressParser<'a> {
    pub curr: Token,
    pub lexer: Lexer<'a>,
    pub peeked: Option<Token>,
}

impl<'a> AddressParser<'a> {
    pub fn new(src: &'a str) -> Self {
        let lexer = Lexer::new(src);
        Self {
            curr: Token::Eof,
            lexer,
            peeked: None,
        }
    }

    #[tracing::instrument(name = "Parse", skip(self))]
    pub fn parse(&mut self) -> AddressParserResult<Box<Node>> {
        self.expr(1)
    }

    #[tracing::instrument(name = "Expr", skip(self))]
    fn expr(&mut self, prec: usize) -> AddressParserResult<Box<Node>> {
        let mut lhs = self.atom()?;
        loop {
            let curr = self.peek_token()?;
            if curr.is_eof() {
                break;
            }

            let Some((op_prec, op_assoc)) = curr.info() else {
                break;
            };

            if op_prec < prec {
                break;
            }

            self.next_token()?;
            let rhs = match op_assoc {
                0 => self.expr(op_prec + 1),
                _ => self.expr(op_prec),
            }?;

            lhs = self.operation(curr, lhs, rhs)?;
        }
        Ok(lhs)
    }

    #[tracing::instrument(name = "Atom", skip(self))]
    fn atom(&mut self) -> AddressParserResult<Box<Node>> {
        match self.peek_token()? {
            Token::LParent => {
                self.expect('(')?;
                let e = self.expr(1)?;
                self.expect(')')?;
                Ok(e)
            }
            Token::OpenBrackets => {
                self.expect('[')?;
                let e = self.expr(1)?;
                self.expect(']')?;
                Ok(Node::Dereference(e).boxed())
            }
            Token::Number(n) => {
                self.next_token()?;
                Ok(Node::Number(n).boxed())
            }
            Token::Symbol(symbol) => {
                self.next_token()?;
                match self.peek_token()? {
                    // sin(expr)
                    Token::LParent => {
                        self.expect('(')?;
                        let e = self.expr(1)?;
                        self.expect(')')?;
                        self.function(&symbol, e)
                    }
                    Token::Symbol(symbol_2) => match symbol_2.as_str() {
                        "let" => {
                            // let a = 10
                            self.next_token()?;
                            self.expect('=')?;
                            let e = self.expr(1)?;
                            Ok(Node::Assignment(symbol_2, e).boxed())
                        }
                        _ => eyre::bail!("Two consecutive symbols"),
                    },
                    _ => Ok(Node::Var(symbol).boxed()),
                }
            }
            Token::ModuleSymbol(v) => {
                self.next_token()?;
                Ok(Node::ModuleSymbol(v).boxed())
            }
            Token::Eof => Ok(Node::Number(0).boxed()),
            t => eyre::bail!("Unrecognized atom {t}"),
        }
    }

    #[tracing::instrument(name = "Operation", skip(self))]
    fn operation(
        &mut self,
        op: Token,
        lhs: Box<Node>,
        rhs: Box<Node>,
    ) -> AddressParserResult<Box<Node>> {
        Ok(match op {
            Token::Add => Node::Add(lhs, rhs).boxed(),
            Token::Sub => Node::Sub(lhs, rhs).boxed(),
            Token::Mul => Node::Mul(lhs, rhs).boxed(),
            Token::Div => Node::Div(lhs, rhs).boxed(),
            Token::Pow => Node::Pow(lhs, rhs).boxed(),

            _ => eyre::bail!("Token not valid operation {op}"),
        })
    }

    #[tracing::instrument(name = "Function", skip(self))]
    fn function(&mut self, op: &str, arg: Box<Node>) -> AddressParserResult<Box<Node>> {
        match op.to_lowercase().as_str() {
            "sin" | "sine" => Ok(Node::Sin(arg).boxed()),
            "cos" | "cosine" => Ok(Node::Cos(arg).boxed()),
            "sqrt" => Ok(Node::Sqrt(arg).boxed()),
            _ => eyre::bail!("Not support function: {op}"),
        }
    }

    fn expect(&mut self, expect_token: char) -> AddressParserResult<()> {
        self.next_token()?;
        let curr: char = self.curr.clone().into();
        if expect_token != curr {
            eyre::bail!("Expected '{expect_token}' but found '{}'", curr)
        }
        Ok(())
    }

    fn peek_token(&mut self) -> AddressParserResult<Token> {
        if self.peeked.is_none() {
            self.peeked = Some(self.lexer.next_token()?);
        }
        Ok(self.peeked.clone().unwrap())
    }

    fn next_token(&mut self) -> AddressParserResult<()> {
        match self.peeked.take() {
            Some(token) => {
                self.curr = token;
            }
            None => {
                self.curr = self.lexer.next_token()?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::address_parser::AddressResolver;

    use super::*;
    #[test]
    fn test_parser() {
        let src = "((10 + 11) + 1) / 2";
        let mut parser = AddressParser::new(src);
        let eval = parser.parse().unwrap();

        let mut env = HashMap::new();
        struct Resolver;
        impl AddressResolver for Resolver {
            fn module_symbol_to_address(&self, module_name: &str) -> Option<isize> {
                match module_name {
                    "target.dll" => Some(0x10),
                    "unTarget.dll" => Some(0x20),
                    _ => None,
                }
            }

            fn dereference(&self, address: usize) -> Option<isize> {
                if address == 0x10 {
                    return Some(0x20);
                }
                None
            }
        }

        let resolver = Resolver;
        let ret = eval.eval(&mut env, &resolver).unwrap();
        println!("Ret={ret}");
    }
}
