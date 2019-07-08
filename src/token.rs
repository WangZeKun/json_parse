use super::err::JsonParseError;
use std::collections::VecDeque;
use std::str::Chars;

pub struct CharReader<'t> {
    chars: Chars<'t>,
    peek: Option<char>,
}

impl<'t> CharReader<'t> {
    pub fn new(json_string: &'t str) -> Self {
        let mut chars = json_string.chars();
        let peek = chars.next();
        CharReader { chars, peek }
    }

    pub fn next(&mut self) -> Option<char> {
        let next = self.peek;
        self.peek = self.chars.next();
        return next;
    }

    pub fn pop(&mut self) {
        self.peek = self.chars.next();
    }

    pub fn peek(&self) -> Option<char> {
        self.peek
    }

    pub fn whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch != ' ' {
                break;
            }
            self.pop();
        }
    }
}
type TokenParseResult = Result<TokenType, JsonParseError>;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    BeginObject,
    EndObject,
    BeginArray,
    EndArray,
    NULL,
    NUMBER(i64),
    STRING(String),
    BOOLEAN(bool),
    SepColon, //:
    SepComma, //,
    EndDocument,
}

pub struct Tokenizer<'t> {
    buffer: CharReader<'t>,
    token_list: VecDeque<TokenType>,
}

impl<'t> Tokenizer<'t> {
    pub fn from(json_string: &'t str) -> Result<VecDeque<TokenType>, JsonParseError> {
        let mut tokenizer = Tokenizer {
            buffer: CharReader::new(json_string),
            token_list: VecDeque::new(),
        };
        loop {
            let token = tokenizer.charge();
            match token {
                Ok(TokenType::EndDocument) => break,
                Ok(token) => tokenizer.token_list.push_back(token),
                Err(err) => return Err(err),
            }
        }
        Ok(tokenizer.token_list)
    }

    fn charge(&mut self) -> TokenParseResult {
        self.buffer.whitespace();
        match self.buffer.next() {
            None => return Ok(TokenType::EndDocument),
            Some(ch) => match ch {
                '{' => Ok(TokenType::BeginObject),
                '}' => Ok(TokenType::EndObject),
                '[' => Ok(TokenType::BeginArray),
                ']' => Ok(TokenType::EndArray),
                ',' => Ok(TokenType::SepComma),
                ':' => Ok(TokenType::SepColon),
                'n' => self.read_null(),
                '"' => self.read_string(),
                't' => self.read_true(),
                'f' => self.read_false(),
                '-' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
                    self.read_number(ch)
                }
                _ => Err(JsonParseError::InvaildValue),
            },
        }
    }

    fn read_null(&mut self) -> TokenParseResult {
        if self.buffer.next() == Some('u')
            && self.buffer.next() == Some('l')
            && self.buffer.next() == Some('l')
        {
            Ok(TokenType::NULL)
        } else {
            Err(JsonParseError::InvaildValue)
        }
    }

    fn read_true(&mut self) -> TokenParseResult {
        if self.buffer.next() == Some('r')
            && self.buffer.next() == Some('u')
            && self.buffer.next() == Some('e')
        {
            Ok(TokenType::BOOLEAN(true))
        } else {
            Err(JsonParseError::InvaildValue)
        }
    }

    fn read_false(&mut self) -> TokenParseResult {
        if self.buffer.next() == Some('a')
            && self.buffer.next() == Some('l')
            && self.buffer.next() == Some('s')
            && self.buffer.next() == Some('e')
        {
            Ok(TokenType::BOOLEAN(false))
        } else {
            Err(JsonParseError::InvaildValue)
        }
    }

    fn read_string(&mut self) -> TokenParseResult {
        let mut s = String::new();
        while let Some(ch) = self.buffer.next() {
            if ch != '"' {
                s.push(ch);
            } else {
                return Ok(TokenType::STRING(s));
            }
        }
        Err(JsonParseError::ExceptValue)
    }

    fn read_number(&mut self, peek: char) -> TokenParseResult {
        let mut s = String::new();
        s.push(peek);
        while let Some(ch) = self.buffer.peek() {
            if ch.is_digit(10) {
                s.push(ch);
                self.buffer.pop();
            } else {
                return Ok(TokenType::NUMBER(s.parse().unwrap()));
            }
        }
        return Ok(TokenType::NUMBER(s.parse().unwrap()));
    }
}


