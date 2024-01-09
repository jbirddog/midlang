use crate::errors::ParseError;
use crate::middle_lang::MidLang;

pub trait Frontend {
    fn parse_file_named(filename: &str) -> Result<MidLang, ParseError>;
}
