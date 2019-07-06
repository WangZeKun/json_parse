#[derive(Debug, PartialEq)]
pub enum JsonParseError {
    ExceptValue,
    InvaildValue,
    RootNotSingular,
}
