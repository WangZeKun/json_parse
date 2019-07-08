//
// parse.rs
// Copyright (C) 2019 zekin <zekin@DESKTOP-78TBROT>
// Distributed under terms of the MIT license.
//

use super::err::JsonParseError;
use super::token_reader::{TokenType, Tokenizer};
use std::collections::{HashMap, VecDeque};
use std::mem::replace;

#[derive(Debug)]
enum Value {
    NULL,
    BOOLEAN(bool),
    NUMBER(i64),
    STRING(String),
    ARRAY(Vec<Value>),
    OBJECT(HashMap<String, Value>),
}

type ParseJsonResult = Result<Value, JsonParseError>;

struct JsonParse {}

impl JsonParse {
    pub fn parse(json: &str) -> ParseJsonResult {
        let tokens_ = Tokenizer::from(json);
        match tokens_ {
            Err(err) => Err(err),
            Ok(mut tokens) => {
                if tokens.len() == 1 {
                    match tokens.pop_front().unwrap() {
                        TokenType::STRING(s) => Ok(Value::STRING(s)),
                        TokenType::BOOLEAN(b) => Ok(Value::BOOLEAN(b)),
                        TokenType::NULL => Ok(Value::NULL),
                        TokenType::NUMBER(n) => Ok(Value::NUMBER(n)),
                        TokenType::BeginObject | TokenType::BeginArray => {
                            Err(JsonParseError::ExceptValue)
                        }
                        _ => Err(JsonParseError::InvaildValue),
                    }
                } else {
                    match tokens.pop_front().unwrap() {
                        TokenType::BeginObject => JsonParse::parse_object(&mut tokens),
                        TokenType::BeginArray => JsonParse::parse_array(&mut tokens),
                        TokenType::EndArray
                        | TokenType::SepColon
                        | TokenType::SepComma
                        | TokenType::EndObject => Err(JsonParseError::InvaildValue),
                        _ => Err(JsonParseError::RootNotSingular),
                    }
                }
            }
        }
    }
    fn parse_object(tokens: &mut VecDeque<TokenType>) -> ParseJsonResult {
        let mut map = HashMap::new();
        let mut now_token = TokenType::BeginObject;
        let mut key = String::new();
        while !tokens.is_empty() {
            match (now_token, tokens.pop_front().unwrap()) {
                (TokenType::SepColon, TokenType::BeginObject) => {
                    match JsonParse::parse_object(tokens) {
                        Ok(value) => map.insert(replace(&mut key, String::new()), value),
                        Err(err) => return Err(err),
                    };
                    now_token = TokenType::EndObject;
                }
                (TokenType::SepColon, TokenType::BeginArray) => {
                    match JsonParse::parse_array(tokens) {
                        Ok(value) => map.insert(replace(&mut key, String::new()), value),
                        Err(err) => return Err(err),
                    };
                    now_token = TokenType::EndArray;
                }
                (TokenType::SepColon, TokenType::NULL) => {
                    map.insert(replace(&mut key, String::new()), Value::NULL);
                    now_token = TokenType::NULL;
                }
                (TokenType::SepColon, TokenType::STRING(s)) => {
                    map.insert(replace(&mut key, String::new()), Value::STRING(s));
                    now_token = TokenType::STRING(String::new());
                }
                (TokenType::SepColon, TokenType::NUMBER(n)) => {
                    map.insert(replace(&mut key, String::new()), Value::NUMBER(n));
                    now_token = TokenType::NUMBER(n);
                }
                (TokenType::SepColon, TokenType::BOOLEAN(b)) => {
                    map.insert(replace(&mut key, String::new()), Value::BOOLEAN(b));
                    now_token = TokenType::BOOLEAN(b);
                }
                (TokenType::SepComma, TokenType::STRING(s))
                | (TokenType::BeginObject, TokenType::STRING(s)) => {
                    key = s;
                    now_token = TokenType::STRING(String::new())
                }
                (TokenType::STRING(_), TokenType::SepColon) if !key.is_empty() => {
                    now_token = TokenType::SepColon
                }
                (ref t, TokenType::SepComma) if key.is_empty() && *t != TokenType::SepComma => {
                    now_token = TokenType::SepComma
                }
                (ref t, TokenType::EndObject)
                    if *t != TokenType::SepComma && *t != TokenType::SepColon =>
                {
                    return Ok(Value::OBJECT(map))
                }
                (t1, t2) => {
                    println!("{:?}", t1);
                    println!("{:?}", t2);
                    return Err(JsonParseError::InvaildValue);
                }
            };
        }
        Err(JsonParseError::ExceptValue)
    }

    fn parse_array(tokens: &mut VecDeque<TokenType>) -> ParseJsonResult {
        let mut array = Vec::new();
        let mut now_token = TokenType::BeginArray;
        while !tokens.is_empty() {
            match (now_token, tokens.pop_front().unwrap()) {
                (TokenType::BeginArray, TokenType::NUMBER(n))
                | (TokenType::SepComma, TokenType::NUMBER(n)) => {
                    array.push(Value::NUMBER(n));
                    now_token = TokenType::NUMBER(n);
                }
                (TokenType::BeginArray, TokenType::STRING(s))
                | (TokenType::SepComma, TokenType::STRING(s)) => {
                    array.push(Value::STRING(s));
                    now_token = TokenType::STRING(String::new());
                }
                (TokenType::BeginArray, TokenType::NULL)
                | (TokenType::SepComma, TokenType::NULL) => {
                    array.push(Value::NULL);
                    now_token = TokenType::NULL;
                }
                (TokenType::BeginArray, TokenType::BOOLEAN(b))
                | (TokenType::SepComma, TokenType::BOOLEAN(b)) => {
                    array.push(Value::BOOLEAN(b));
                    now_token = TokenType::BOOLEAN(b);
                }
                (TokenType::BeginArray, TokenType::BeginObject)
                | (TokenType::SepComma, TokenType::BeginObject) => {
                    match JsonParse::parse_object(tokens) {
                        Ok(value) => array.push(value),
                        Err(err) => return Err(err),
                    };
                    now_token = TokenType::EndObject;
                }
                (ref t, TokenType::SepComma) if *t != TokenType::SepComma => {
                    now_token = TokenType::SepComma
                }
                (ref t, TokenType::EndArray) if *t != TokenType::SepComma => {
                    return Ok(Value::ARRAY(array))
                }
                _ => return Err(JsonParseError::InvaildValue),
            }
        }
        Err(JsonParseError::ExceptValue)
    }
}

impl Value {
    pub fn is_object(&self) -> bool {
        self.as_object().is_some()
    }

    pub fn as_object(&self) -> Option<&HashMap<String,Value>> {
        match self {
            Value::OBJECT(ref map) => Some(map),
            _ => None,
        }
    }
    
    pub fn as_object_mut(&mut self) -> Option<&mut HashMap<String,Value>> {
        match self {
            Value::OBJECT(ref mut map) => Some(map),
            _ => None,
        }
    }

    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::ARRAY(ref array) => Some(array),
            _ => None,
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            Value::ARRAY(ref mut array) => Some(array),
            _ => None,
        }
    }

    pub fn is_string(&self) -> bool {
        self.as_string().is_some()
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::STRING(s) => Some(s),
            _=>None
        }
    }

    pub fn as_string_mut(&mut self) -> Option<&mut String> {
        match self {
            Value::STRING(ref mut s) => Some(s),
            _=>None
        }
    }

    pub fn is_number(&self) -> bool {
        self.as_number().is_some()
    }

    pub fn as_number(&self) -> Option<&i64> {
        match self {
            Value::NUMBER(n) => Some(n),
            _ => None,
        }
    }


    pub fn is_boolean(&self) -> bool {
        self.as_boolean().is_some()
    }

    pub fn as_boolean(&self) -> Option<&bool> {
        match self {
            Value::BOOLEAN(b) => Some(b),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            Value::NULL => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_parse() {
        let json = "null";
        let result = JsonParse::parse(json).unwrap();
        assert!(result.is_null())
    }

    #[test]
    fn number_parse() {
        let json = "-1234";
        let result = JsonParse::parse(json).unwrap();
        assert!(result.is_number())
    }

    #[test]
    fn string_parse() {
        let json = "\"-1234\"";
        let result = JsonParse::parse(json).unwrap();
        assert!(result.is_string())
    }

    #[test]
    fn boolean_parse() {
        let json = "true";
        let result = JsonParse::parse(json).unwrap();
        assert!(result.is_boolean())
    }

    #[test]
    fn simple_array_parse() {
        let json = "[]";
        let result = JsonParse::parse(json).unwrap();
        assert!(result.is_array())
    }

    #[test]
    fn simple_array_parse1() {
        let json = "[111,234]";
        let result = JsonParse::parse(json).unwrap();
        assert!(result.is_array())
    }

    #[test]
    fn simple_array_parse2() {
        let json = "[111,234,{}]";
        let result = JsonParse::parse(json).unwrap();
        assert!(result.is_array())
    }

    #[test]
    fn simple_object_parse() {
        let json = "{}";
        let result = JsonParse::parse(json).unwrap();
        assert!(result.is_object())
    }

    #[test]
    fn simple_object_parse1() {
        let json = "{\"hh\":123}";
        let result = JsonParse::parse(json).unwrap();
        assert!(result.is_object())
    }

    #[test]
    fn simple_object_parse2() {
        let json = "{\"hh\":123,\"123\":[{\"456\":null},{\"789\":true},{}]}";
        let result = JsonParse::parse(json).unwrap();
        assert!(result.is_object())
    }
}
