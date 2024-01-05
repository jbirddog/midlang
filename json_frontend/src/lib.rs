use serde::Deserialize;
use serde_json;

use midlang;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum MidLang {
    Module { name: String, decls: Vec<Decl> },
}

#[derive(Deserialize)]
struct Decl {}

pub fn parse_string<'a, T>(str: &'a str) -> serde_json::Result<T>
where
    T: Deserialize<'a>,
{
    serde_json::from_str::<T>(str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_module() -> serde_json::Result<()> {
        let json = "{\"module\": {\"name\": \"empty\", \"decls\": []}}";

        match parse_string::<MidLang>(json)? {
            MidLang::Module { name, decls } => {
                assert_eq!(name.as_str(), "empty");
                assert_eq!(decls.len(), 0);
            }
        }

        Ok(())
    }
}
