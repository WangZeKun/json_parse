pub mod char_reader;
pub mod err;
mod parse;
pub mod token_reader;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
