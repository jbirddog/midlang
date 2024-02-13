use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

mod json_lang;
mod lower;
mod raise;

use lower::lower;
use raise::raise;

use midlang as m;

pub struct Frontend<'a> {
    filename: &'a str,
}

pub fn new(filename: &str) -> Frontend<'_> {
    Frontend { filename }
}

impl compiler::Frontend for Frontend<'_> {
    fn lower(&self) -> compiler::FrontendLowerResult {
        let path = PathBuf::from(self.filename);

        Self::lower_from_file(&path)
    }

    fn raise(&self, modules: &[m::Module]) -> compiler::FrontendRaiseResult {
        let path = PathBuf::from(self.filename);

        Self::raise_to_file(modules, &path)
    }
}

impl Frontend<'_> {
    fn lower_from_file(path: &Path) -> compiler::FrontendLowerResult {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let json_lang = serde_json::from_reader(reader)?;

        lower(&json_lang)
    }

    fn raise_to_file(modules: &[m::Module], path: &Path) -> compiler::FrontendRaiseResult {
        let modules = raise(modules)?;

        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        serde_json::to_writer_pretty(&mut writer, &modules)?;

        writer.flush()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use compiler::Frontend;
    use std::error::Error;
    use std::path::Path;

    type TestResult = Result<(), Box<dyn Error>>;

    macro_rules! test {
        ($test_case:ident) => {{
            let path = Path::new(env!("TEST_CASES_DIR"))
                .join("json")
                .join(stringify!($test_case))
                .with_extension("json");
            let filename = &path.display().to_string();
            let frontend = new(&filename);
            let m = mtc::$test_case();

            frontend.raise(&m)?;
            frontend.lower()?;

            Ok(())
        }};
    }

    #[test]
    fn hello_world() -> TestResult {
        test!(hello_world)
    }

    #[test]
    fn hello_world2() -> TestResult {
        test!(hello_world2)
    }

    #[test]
    fn cond() -> TestResult {
        test!(cond)
    }

    #[test]
    fn fabs() -> TestResult {
        test!(fabs)
    }

    #[test]
    fn frexp() -> TestResult {
        test!(frexp)
    }

    #[test]
    fn cmp() -> TestResult {
        test!(cmp)
    }

    #[test]
    fn not() -> TestResult {
        test!(not)
    }
}
