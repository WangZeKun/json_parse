//
// char_reader.rs
// Copyright (C) 2019 zekin <zekin@DESKTOP-UR3A57I>
// Distributed under terms of the MIT license.
//
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

    fn pop(&mut self) {
        self.peek = self.chars.next();
    }

    fn peek(&self) -> Option<char> {
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


