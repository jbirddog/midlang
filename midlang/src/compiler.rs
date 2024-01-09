use std::collections::HashMap;
use std::error::Error;

use crate::middle_lang::MidLang;

pub trait Frontend {
    fn parse_file_named(&self, filename: &str) -> Result<MidLang, Box<dyn Error>>;
}

pub trait Backend {
    fn generate_build_artifacts(
        &self,
        midlang: &MidLang,
    ) -> Result<HashMap<String, String>, Box<dyn Error>>;
}

pub struct Compiler<'a> {
    pub frontend: &'a dyn Frontend,
    pub backend: &'a dyn Backend,
}

impl Compiler<'_> {
    pub fn compile(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let midlang_module = self.frontend.parse_file_named(filename)?;
        let _ = self.backend.generate_build_artifacts(&midlang_module)?;

        Ok(())
    }
}
