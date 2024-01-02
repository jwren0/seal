use std::collections::HashMap;

use crate::repl::tokenizer::{Tokenizer, Token};

pub struct Evaluator {
    tokens: Option<Vec<Token>>,
    vars: HashMap<String, i64>,
    cursor: usize,
}

impl Evaluator {
    fn peek_ahead(&self, count: usize) -> Option<&Token> {
        let Some(ref tokens) = self.tokens else {
            return None;
        };

        tokens.get(self.cursor + count)

    }

    fn peek(&self) -> Option<&Token> {
        self.peek_ahead(0)
    }

    fn next(&mut self) -> Result<&Token, String> {
        let Some(ref tokens) = self.tokens else {
            return Err("There are no tokens to evaluate.".to_string());
        };

        let Some(token) = tokens.get(self.cursor) else {
            return Err(
                "Unexpectedly reached the end of the tokens.".to_string()
            );
        };

        self.cursor += 1;

        Ok(token)
    }

    fn consume(&mut self, token: &Token) -> Result<(), String> {
        let actual = self.next()?;

        if actual != token {
            return Err(format!(
                "Expected '{token:?}', but got '{actual:?}'."
            ));
        }

        Ok(())
    }

    fn get_ident(&mut self) -> Result<String, String> {
        match self.next()? {
            Token::Ident(name) => Ok(name.to_string()),
            t => Err(format!(
                "Expected identifier, but got {t:?}."
            ))
        }
    }

    fn eval_factor(&mut self) -> Result<i64, String> {
        let Some(ref tokens) = self.tokens else {
            return Err("There are no tokens to evaluate".to_string());
        };

        let Some(token) = tokens.get(self.cursor) else {
            return Err("Unexpectedly reached the end of the tokens".to_string());
        };

        self.cursor += 1;

        match token {
            Token::LParen => {
                let result = self.eval_calc()?;
                self.consume(&Token::RParen)?;

                Ok(result)
            },
            Token::Value(n) => Ok(*n),
            Token::Ident(name) => {
                let Some(value) = self.vars.get(name) else {
                    return Err(format!("Unknown identifier: '{name}'"));
                };

                Ok(*value)
            }
            t => Err(format!("Expected a factor, but got '{t:?}'.")),
        }
    }

    fn eval_term(&mut self) -> Result<i64, String> {
        let mut result = self.eval_factor()?;

        let Some(ref tokens) = self.tokens else {
            return Ok(result);
        };
        let len = tokens.len();

        while self.cursor < len {
            match self.peek() {
                Some(Token::OpMult) => {
                    self.cursor += 1;
                    result *= self.eval_factor()?;
                }
                Some(Token::OpDiv) => {
                    self.cursor += 1;
                    result /= self.eval_factor()?;
                }
                _ => break,
            }
        }

        Ok(result)
    }

    fn eval_calc(&mut self) -> Result<i64, String> {
        let mut result = self.eval_term()?;

        let Some(ref tokens) = self.tokens else {
            return Ok(result);
        };
        let len = tokens.len();

        while self.cursor < len {
            match self.peek() {
                Some(Token::OpPlus) => {
                    self.cursor += 1;
                    result += self.eval_term()?;
                }
                Some(Token::OpMinus) => {
                    self.cursor += 1;
                    result -= self.eval_term()?;
                }
                _ => break,
            }
        }

        Ok(result)
    }

    fn eval_assign(&mut self) -> Result<i64, String> {
        let ident = self.get_ident()?;
        self.consume(&Token::Equals)?;
        let result = self.eval_calc()?;

        self.vars.insert(ident, result);

        Ok(result)
    }

    fn eval(&mut self) -> Result<i64, String> {
        self.cursor = 0;

        match self.peek_ahead(1) {
            Some(Token::Equals) => self.eval_assign(),
            _ => self.eval_calc(),
        }
    }

    pub fn run(&mut self, content: &str) {
        let mut tokenizer = Tokenizer::new(content);

        match tokenizer.run() {
            Ok(tokens) => self.tokens = Some(tokens),
            Err(msg) => {
                eprintln!("{msg}");
                return;
            }
        }

        match self.eval() {
            Ok(n) => println!("{n}"),
            Err(msg) => eprintln!("{msg}"),
        }
    }

    pub fn new() -> Self {
        Evaluator {
            tokens: None,
            vars: HashMap::new(),
            cursor: 0,
        }
    }
}
