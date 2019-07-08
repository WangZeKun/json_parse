//
// parse_test.rs
// Copyright (C) 2019 zekin <zekin@DESKTOP-78TBROT>
// Distributed under terms of the MIT license.
//
use my_json::parse::*;

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
