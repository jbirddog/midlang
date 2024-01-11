use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

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
    frontend: &'a dyn Frontend,
    backend: &'a dyn Backend,
    build_dir: &'a str,
    ninja: &'a str,
}

pub fn new<'a>(
    frontend: &'a dyn Frontend,
    backend: &'a dyn Backend,
    build_dir: &'a str,
    ninja: &'a str,
) -> Compiler<'a> {
    Compiler {
        frontend,
        backend,
        build_dir,
        ninja,
    }
}

impl Compiler<'_> {
    pub fn compile(&self) -> Result<(), Box<dyn Error>> {
        let midlang = self.frontend.parse()?;
        let mut ninja_writer = Ninja::new();
        let mut build_artifacts = self
            .backend
            .generate_build_artifacts(&midlang, &mut ninja_writer)?;
        build_artifacts.push(("build.ninja".to_string(), ninja_writer.to_string()));

        write_build_artifacts(&build_artifacts, self.build_dir)?;
        execute_build(self.ninja, self.build_dir)?;

        Ok(())
    }
}

fn write_build_artifacts(build_artifacts: &BuildArtifacts, build_dir: &str) -> io::Result<()> {
    let build_dir = Path::new(build_dir);
    fs::create_dir_all(build_dir)?;

    for (name, contents) in build_artifacts {
        let artifact = &build_dir.join(name);
        fs::write(artifact, contents)?;
    }

    Ok(())
}

fn execute_build(ninja: &str, build_dir: &str) -> io::Result<()> {
    let mut child = Command::new(ninja).arg("-C").arg(build_dir).spawn()?;
    child.wait()?;

    Ok(())
}
