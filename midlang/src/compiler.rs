use std::error::Error;

use ninja_writer::Ninja;

use crate::middle_lang::MidLang;

pub type FrontendResult = Result<MidLang, Box<dyn Error>>;

pub trait Frontend {
    fn parse(&self) -> FrontendResult;
}

pub type BuildArtifacts = Vec<(String, String)>;
pub type BackendResult = Result<BuildArtifacts, Box<dyn Error>>;

pub trait Backend {
    fn generate_build_artifacts(
        &self,
        midlang: &MidLang,
        ninja_writer: &mut Ninja,
    ) -> BackendResult;
}

pub struct Compiler<'a> {
    pub frontend: &'a dyn Frontend,
    pub backend: &'a dyn Backend,
}

pub fn new<'a>(frontend: &'a dyn Frontend, backend: &'a dyn Backend) -> Compiler<'a> {
    Compiler { frontend, backend }
}

impl Compiler<'_> {
    pub fn compile(&self) -> Result<(), Box<dyn Error>> {
        let midlang = self.frontend.parse()?;
        let mut ninja_writer = Ninja::new();
        let _ = self
            .backend
            .generate_build_artifacts(&midlang, &mut ninja_writer)?;

        Ok(())
    }
}
