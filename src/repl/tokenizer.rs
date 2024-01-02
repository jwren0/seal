#[derive(Debug, PartialEq)]
pub enum Token {
    Equals,
    OpPlus,
    OpMinus,
    OpMult,
    OpDiv,
    LParen,
    RParen,
    Ident(String),
    Value(i64),
}

pub struct Tokenizer<'a> {
    content: &'a str,
    cursor: usize,
}

fn is_predicate(slice: &str, predicate: impl Fn(char) -> bool) -> bool {
    slice.chars().fold(true, |acc, c| acc && predicate(c))
}

impl<'a> Tokenizer<'a> {
    fn make_error(&self, msg: &str) -> String {
        format!("{msg}.")
    }

    fn peek(&self) -> Option<&str> {
        let start = self.cursor;
        let mut end = start + 1;

        loop {
            if end > self.content.len() {
                return None;
            }

            if let Some(slice) = self.content.get(start..end) {
                return Some(slice);
            };

            end += 1;
        }
    }

    fn collect_while(&mut self, predicate: impl Fn(char) -> bool) -> String {
        let mut value = String::new();

        while let Some(slice) = self.peek() {
            if is_predicate(slice, &predicate) == false {
                break;
            }

            value.push_str(slice);
            self.cursor += slice.len();
        }

        return value;
    }

    fn skip_while(&mut self, predicate: impl Fn(char) -> bool) {
        while let Some(slice) = self.peek() {
            if is_predicate(slice, &predicate) == false {
                break;
            }

            self.cursor += slice.len();
        }
    }

    fn skip_whitespace(&mut self) {
        self.skip_while(|c| c.is_whitespace());
    }

    fn tokenize_ident(&mut self) -> Token {
        let token = self.collect_while(|c| {
            c.is_alphanumeric() || c == '_'
        });

        Token::Ident(token)
    }

    fn tokenize_number(&mut self) -> Result<Token, String> {
        let token = self.collect_while(|c| c.is_numeric());

        let Ok(value) = token.parse() else {
            let msg = match self.peek() {
                Some(v) => format!("got '{v}'"),
                None => "reached the end of the content".to_string(),
            };

            return Err(self.make_error(&format!(
                "Expected numeric value, but {msg}",
            )));
        };

        Ok(Token::Value(value))
    }

    fn tokenize(&mut self) -> Result<Token, String> {
        let Some(slice) = self.peek() else {
            return Err(self.make_error(
                "Unexpectedly reached the end of the content"
            ));
        };

        if is_predicate(slice, |c| c.is_alphabetic()) {
            // Identifier
            return Ok(self.tokenize_ident());
        }

        if is_predicate(slice, |c| c.is_numeric()) {
            // Number
            return Ok(self.tokenize_number()?);
        }

        let token = match slice {
            "=" => Token::Equals,
            "+" => Token::OpPlus,
            "-" => Token::OpMinus,
            "*" => Token::OpMult,
            "/" => Token::OpDiv,
            "(" => Token::LParen,
            ")" => Token::RParen,

            v => return Err(self.make_error(&format!(
                "Unexpected token: '{v}'"
            ))),
        };

        self.cursor += slice.len();

        Ok(token)
    }

    pub fn run(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while self.peek() != None {
            self.skip_whitespace();
            tokens.push(self.tokenize()?);
        }

        Ok(tokens)
    }

    pub fn new(content: &'a str) -> Self {
        Tokenizer {
            content,
            cursor: 0,
        }
    }
}
