use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use midlang::compiler;
use midlang::middle_lang as m;

mod json_lang;
mod lower;

use lower::lower;

pub struct Frontend<'a> {
    filename: &'a str,
}

pub fn new(filename: &str) -> Frontend<'_> {
    Frontend { filename }
}

impl compiler::Frontend for Frontend<'_> {
    fn parse(&self) -> Result<m::MidLang, Box<dyn Error>> {
        let path = PathBuf::from(self.filename);

        Self::parse_file(&path)
    }
}

impl Frontend<'_> {
    fn parse_file(path: &Path) -> Result<m::MidLang, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let json_lang = serde_json::from_reader(reader)?;

        lower(&json_lang)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    type TestResult = Result<(), Box<dyn Error>>;

    #[test]
    fn hello_world() -> TestResult {
        let path = Path::new(env!("TEST_CASES_DIR"))
            .join("json")
            .join("hello_world.json");

        Frontend::parse_file(&path)?;

        Ok(())
    }
}
