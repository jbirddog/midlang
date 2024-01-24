use std::collections::BTreeMap;

use crate::lower_lang::*;

pub struct LoweringCtx {
    prefix: String,
    pool: BTreeMap<String, String>,
    uniq: u32,
}

impl LoweringCtx {
    pub fn new(prefix: &str) -> LoweringCtx {
        LoweringCtx {
            prefix: prefix.to_string(),
            pool: Default::default(),
            uniq: 0,
        }
    }

    pub fn uniq_name(&mut self, prefix: &str) -> String {
        let name = format!("{}{}", prefix, self.uniq);
        self.uniq += 1;
        name
    }

    pub fn name_for_str(&mut self, str: &str) -> String {
        let len = self.pool.len();
        self.pool
            .entry(str.to_string())
            .or_insert_with(|| format!("{}_str{}", self.prefix, len))
            .to_string()
    }

    pub fn decls(&self) -> Vec<Decl> {
        fn fields(value: &str) -> Vec<DataField> {
            vec![
                (Type::B, format!("\"{}\"", str::replace(value, "\n", "\\n"))),
                (Type::B, "0".to_string()),
            ]
        }

        self.pool
            .iter()
            .map(|(k, v)| Decl::Data(v.to_string(), fields(k)))
            .collect()
    }

    pub fn decls_len(&self) -> usize {
        self.pool.len()
    }
}
