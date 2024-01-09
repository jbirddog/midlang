use std::error::Error;

use crate::errors::{BuildError, ParseError};
use crate::middle_lang::MidLang;

pub trait Frontend {
    fn parse_file_named(filename: &str) -> Result<MidLang, Box<dyn Error>>;
}

pub trait Backend {
    fn generate_build_artifacts(midlang: &MidLang) -> Result<(), Box<dyn Error>>;
}

