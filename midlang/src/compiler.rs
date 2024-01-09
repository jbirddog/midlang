use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

use crate::middle_lang::MidLang;

pub trait Frontend {
    fn parse_file(path: &Path) -> Result<MidLang, Box<dyn Error>>;
    fn parse_file_named(filename: &str) -> Result<MidLang, Box<dyn Error>>;
}

pub trait Backend {
    fn generate_build_artifacts(
        midlang: &MidLang,
    ) -> Result<HashMap<String, String>, Box<dyn Error>>;
}

pub struct Compiler<F, B> {
    pub frontend: F,
    pub backend: B,
}

impl<F, B> Compiler<F, B>
where
    F: Frontend,
    B: Backend,
{
    pub fn compile(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let midlang_module = F::parse_file_named(filename)?;
        let _ = B::generate_build_artifacts(&midlang_module)?;

        Ok(())
    }
}
