use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

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
    fn parse(&self) -> compiler::FrontendResult {
        let path = PathBuf::from(self.filename);

        Self::parse_file(&path)
    }
}

impl Frontend<'_> {
    fn parse_file(path: &Path) -> compiler::FrontendResult {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let json_lang = serde_json::from_reader(reader)?;

        lower(&json_lang)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use compiler::Frontend;
    use std::error::Error;
    use std::path::Path;

    type TestResult = Result<(), Box<dyn Error>>;

    #[test]
    fn hello_world() -> TestResult {
        let path = Path::new(env!("TEST_CASES_DIR"))
            .join("json")
            .join("hello_world.json");
        let filename = &path.display().to_string();

        new(&filename).parse()?;

        Ok(())
    }

    #[test]
    fn hello_world2() -> TestResult {
        let path = Path::new(env!("TEST_CASES_DIR"))
            .join("json")
            .join("hello_world2.json");
        let filename = &path.display().to_string();

        new(&filename).parse()?;

        Ok(())
    }

    #[test]
    fn hello_world_cond() -> TestResult {
        let path = Path::new(env!("TEST_CASES_DIR"))
            .join("json")
            .join("hello_world_cond.json");
        let filename = &path.display().to_string();

        new(&filename).parse()?;

        Ok(())
    }

    #[test]
    fn math() -> TestResult {
        let path = Path::new(env!("TEST_CASES_DIR"))
            .join("json")
            .join("fabs.json");
        let filename = &path.display().to_string();

        new(&filename).parse()?;

        Ok(())
    }

    #[test]
    fn cmp() -> TestResult {
        let path = Path::new(env!("TEST_CASES_DIR"))
            .join("json")
            .join("cmp.json");
        let filename = &path.display().to_string();

        new(&filename).parse()?;

        Ok(())
    }
}
