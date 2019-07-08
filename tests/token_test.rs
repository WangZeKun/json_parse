//
// token_test.rs
// Copyright (C) 2019 zekin <zekin@DESKTOP-78TBROT>
// Distributed under terms of the MIT license.
//

use my_json::token::*;
use my_json::err::*;

#[test]
fn true_test() {
    let json = "true";
    let result = Tokenizer::from(json).ok().unwrap();
    assert_eq!(result[0], TokenType::BOOLEAN(true));
}

#[test]
fn false_test() {
    let json = "false";
    let result = Tokenizer::from(json).ok().unwrap();
    assert_eq!(result[0], TokenType::BOOLEAN(false));
}
#[test]
fn null_test() {
    let json = "null";
    let result = Tokenizer::from(json).ok().unwrap();
    assert_eq!(result[0], TokenType::NULL);
}

#[test]
fn number_test() {
    let json = "1234123";
    let result = Tokenizer::from(json).ok().unwrap();
    assert_eq!(result[0], TokenType::NUMBER(1234123));
}

#[test]
fn number_test2() {
    let json = "-1234123";
    let result = Tokenizer::from(json).ok().unwrap();
    assert_eq!(result[0], TokenType::NUMBER(-1234123));
}

#[test]
fn string_test() {
    let json = "\"123\"";
    let result = Tokenizer::from(json).ok().unwrap();
    assert_eq!(result[0], TokenType::STRING(String::from("123")));
}

#[test]
fn array_test() {
    let json = "[]";
    let result = Tokenizer::from(json).ok().unwrap();
    assert_eq!(result[0], TokenType::BeginArray);
    assert_eq!(result[1], TokenType::EndArray);
}

#[test]
fn array_test1() {
    let json = "[123,234]";
    let result = Tokenizer::from(json).ok().unwrap();
    assert_eq!(result[0], TokenType::BeginArray);
    assert_eq!(result[1], TokenType::NUMBER(123));
    assert_eq!(result[2], TokenType::SepComma);
    assert_eq!(result[3], TokenType::NUMBER(234));
    assert_eq!(result[4], TokenType::EndArray);
}

#[test]
fn object_test() {
    let json = "{}";
    let result = Tokenizer::from(json).ok().unwrap();
    assert_eq!(result[0], TokenType::BeginObject);
    assert_eq!(result[1], TokenType::EndObject);
}

#[test]
fn object_test1() {
    let json = "{\"11\":123}";
    let result = Tokenizer::from(json).ok().unwrap();
    assert_eq!(result[0], TokenType::BeginObject);
    assert_eq!(result[1], TokenType::STRING(String::from("11")));
    assert_eq!(result[2], TokenType::SepColon);
    assert_eq!(result[3], TokenType::NUMBER(123));
    assert_eq!(result[4], TokenType::EndObject);
}

#[test]
fn error_test() {
    let json = "nllu";
    assert_eq!(Tokenizer::from(json), Err(JsonParseError::InvaildValue));
}

#[test]
fn error_test2() {
    let json = "\"nllu";
    assert_eq!(Tokenizer::from(json), Err(JsonParseError::ExceptValue));
}
