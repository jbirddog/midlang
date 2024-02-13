use std::path::Path;

use ninja_writer::BuildVariables as _;
use ninja_writer::Ninja;

mod il;
mod lower;
mod lower_lang;
mod lowering_context;

use il::generate_il;
use lower::lower;

use midlang as m;

pub struct Backend<'a> {
    libraries: &'a Vec<String>,
    library_paths: &'a Vec<String>,
    output: &'a String,
}

pub fn new<'a>(
    libraries: &'a Vec<String>,
    library_paths: &'a Vec<String>,
    output: &'a String,
) -> Backend<'a> {
    Backend {
        libraries,
        library_paths,
        output,
    }
}

impl compiler::Backend for Backend<'_> {
    fn generate_build_artifacts(
        &self,
        modules: &[m::Module],
        ninja_writer: &mut Ninja,
    ) -> compiler::BackendResult {
        let comp_units = lower(modules);
        let build_artifacts = generate_il(&comp_units)?;

        set_link_flags_var(self.libraries, self.library_paths, ninja_writer);
        configure_ninja_build(&build_artifacts, self.output, ninja_writer);

        Ok(build_artifacts)
    }
}

fn set_link_flags_var(libraries: &[String], library_paths: &[String], ninja_writer: &mut Ninja) {
    let mut link_flags = libraries
        .iter()
        .map(|l| format!("-l{}", l))
        .collect::<Vec<_>>();

    link_flags.extend(
        library_paths
            .iter()
            .map(|l| format!("-L{}", l))
            .collect::<Vec<_>>(),
    );

    ninja_writer.variable("link_flags", link_flags.join(" "));
}

fn configure_ninja_build(
    build_artifacts: &compiler::BuildArtifacts,
    output: &String,
    ninja_writer: &mut Ninja,
) {
    let qbe = ninja_writer.rule("qbe", "qbe -o $out $in");
    let cc = ninja_writer.rule("cc", "cc -o $out -c $in");
    let link = ninja_writer.rule("link", "cc -o $out $in $link_flags");
    let mut objs = Vec::<String>::with_capacity(build_artifacts.len());

    for (il, _) in build_artifacts {
        let asm = with_ext(il, "s");
        let obj = with_ext(il, "o");

        qbe.build([&asm]).with([&il]);
        cc.build([&obj]).with([&asm]);
        objs.push(obj);
    }

    link.build([&output]).with(&objs);
    ninja_writer.defaults([&output]);
}

fn with_ext(filename: &str, ext: &str) -> String {
    Path::new(filename)
        .with_extension(ext)
        .display()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fs::read_to_string;
    use std::path::Path;

    use compiler::Backend as _;

    type TestResult = Result<(), Box<dyn Error>>;

    fn generate_build_artifacts(
        modules: &[m::Module],
        ninja_writer: &mut Ninja,
    ) -> compiler::BackendResult {
        let output = "a.out".to_string();
        new(&vec![], &vec![], &output).generate_build_artifacts(modules, ninja_writer)
    }

    #[test]
    fn hello_world() -> TestResult {
        let modules = mtc::hello_world();

        let mut ninja_writer = Ninja::new();
        let ba = generate_build_artifacts(&modules, &mut ninja_writer)?;
        assert_eq!(ba.len(), 1);
        assert_eq!(ba[0].0, "hello_world.il");

        let path = Path::new(env!("TEST_CASES_DIR"))
            .join("qbe")
            .join("hello_world.il");
        let expected_il = read_to_string(&path)?;

        assert_eq!(ba[0].1, expected_il);

        let ninja_build = ninja_writer.to_string();
        assert!(ninja_build.contains("hello_world.il"));
        assert!(ninja_build.contains("hello_world.s"));
        assert!(ninja_build.contains("hello_world.o"));
        assert!(ninja_build.contains("a.out"));

        Ok(())
    }

    #[test]
    fn hello_world2() -> TestResult {
        let modules = mtc::hello_world2();

        let mut ninja_writer = Ninja::new();
        let ba = generate_build_artifacts(&modules, &mut ninja_writer)?;
        assert_eq!(ba.len(), 2);
        assert_eq!(ba[0].0, "hello_world2.il");
        assert_eq!(ba[1].0, "hello_world2_sayer.il");

        {
            let path = Path::new(env!("TEST_CASES_DIR"))
                .join("qbe")
                .join("hello_world2.il");
            let expected_il = read_to_string(&path)?;

            assert_eq!(ba[0].1, expected_il);
        }

        {
            let path = Path::new(env!("TEST_CASES_DIR"))
                .join("qbe")
                .join("hello_world2_sayer.il");
            let expected_il = read_to_string(&path)?;

            assert_eq!(ba[1].1, expected_il);
        }

        let ninja_build = ninja_writer.to_string();
        assert!(ninja_build.contains("hello_world2.il"));
        assert!(ninja_build.contains("hello_world2.s"));
        assert!(ninja_build.contains("hello_world2.o"));
        assert!(ninja_build.contains("hello_world2_sayer.il"));
        assert!(ninja_build.contains("hello_world2_sayer.s"));
        assert!(ninja_build.contains("hello_world2_sayer.o"));
        assert!(ninja_build.contains("a.out"));

        Ok(())
    }

    #[test]
    fn fabs() -> TestResult {
        let modules = mtc::fabs();

        let mut ninja_writer = Ninja::new();
        let ba = generate_build_artifacts(&modules, &mut ninja_writer)?;
        assert_eq!(ba.len(), 1);
        assert_eq!(ba[0].0, "fabs.il");

        let path = Path::new(env!("TEST_CASES_DIR"))
            .join("qbe")
            .join("fabs.il");
        let expected_il = read_to_string(&path)?;

        assert_eq!(ba[0].1, expected_il);

        let ninja_build = ninja_writer.to_string();
        assert!(ninja_build.contains("fabs.il"));
        assert!(ninja_build.contains("fabs.s"));
        assert!(ninja_build.contains("fabs.o"));
        assert!(ninja_build.contains("a.out"));

        Ok(())
    }

    #[test]
    fn frexp() -> TestResult {
        let modules = mtc::frexp();

        let mut ninja_writer = Ninja::new();
        let ba = generate_build_artifacts(&modules, &mut ninja_writer)?;
        assert_eq!(ba.len(), 1);
        assert_eq!(ba[0].0, "frexp.il");

        let path = Path::new(env!("TEST_CASES_DIR"))
            .join("qbe")
            .join("frexp.il");
        let expected_il = read_to_string(&path)?;

        assert_eq!(ba[0].1, expected_il);

        let ninja_build = ninja_writer.to_string();
        assert!(ninja_build.contains("frexp.il"));
        assert!(ninja_build.contains("frexp.s"));
        assert!(ninja_build.contains("frexp.o"));
        assert!(ninja_build.contains("a.out"));

        Ok(())
    }

    #[test]
    fn cmp() -> TestResult {
        let modules = mtc::cmp();

        let mut ninja_writer = Ninja::new();
        let ba = generate_build_artifacts(&modules, &mut ninja_writer)?;
        assert_eq!(ba.len(), 1);
        assert_eq!(ba[0].0, "cmp.il");

        let path = Path::new(env!("TEST_CASES_DIR")).join("qbe").join("cmp.il");
        let expected_il = read_to_string(&path)?;

        assert_eq!(ba[0].1, expected_il);

        let ninja_build = ninja_writer.to_string();
        assert!(ninja_build.contains("cmp.il"));
        assert!(ninja_build.contains("cmp.s"));
        assert!(ninja_build.contains("cmp.o"));
        assert!(ninja_build.contains("a.out"));

        Ok(())
    }

    #[test]
    fn cond() -> TestResult {
        let modules = mtc::cond();

        let mut ninja_writer = Ninja::new();
        let ba = generate_build_artifacts(&modules, &mut ninja_writer)?;
        assert_eq!(ba.len(), 1);
        assert_eq!(ba[0].0, "cond.il");

        let path = Path::new(env!("TEST_CASES_DIR"))
            .join("qbe")
            .join("cond.il");
        let expected_il = read_to_string(&path)?;

        assert_eq!(ba[0].1, expected_il);

        let ninja_build = ninja_writer.to_string();
        assert!(ninja_build.contains("cond.il"));
        assert!(ninja_build.contains("cond.s"));
        assert!(ninja_build.contains("cond.o"));
        assert!(ninja_build.contains("a.out"));

        Ok(())
    }

    #[test]
    fn not() -> TestResult {
        let modules = mtc::not();

        let mut ninja_writer = Ninja::new();
        let ba = generate_build_artifacts(&modules, &mut ninja_writer)?;
        assert_eq!(ba.len(), 1);
        assert_eq!(ba[0].0, "not.il");

        let path = Path::new(env!("TEST_CASES_DIR")).join("qbe").join("not.il");
        let expected_il = read_to_string(&path)?;

        assert_eq!(ba[0].1, expected_il);

        let ninja_build = ninja_writer.to_string();
        assert!(ninja_build.contains("not.il"));
        assert!(ninja_build.contains("not.s"));
        assert!(ninja_build.contains("not.o"));
        assert!(ninja_build.contains("a.out"));

        Ok(())
    }
}
