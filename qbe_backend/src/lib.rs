use std::collections::HashMap;

mod lower;
mod lower_lang;
mod lowering_context;

use lower::lower;

use midlang::compiler;
use midlang::middle_lang as m;

pub struct Backend {}

pub fn new() -> Backend {
    Backend {}
}

impl compiler::Backend for Backend {
    fn generate_build_artifacts(&self, midlang: &m::MidLang) -> HashMap<String, String> {
        let _ = lower(midlang);

        Default::default()
    }
}
