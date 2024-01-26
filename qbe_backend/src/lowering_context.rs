use std::collections::BTreeMap;

use crate::lower_lang::*;

pub type TmpRef = (String, String, Type);

pub struct LoweringCtx {
    prefix: String,
    pool: BTreeMap<String, String>,
    tmp_refs: Vec<Vec<TmpRef>>,
    uniq: u32,
}

impl LoweringCtx {
    pub fn new(prefix: &str) -> LoweringCtx {
        LoweringCtx {
            prefix: prefix.to_string(),
            pool: Default::default(),
            tmp_refs: Default::default(),
            uniq: 0,
        }
    }

    pub fn uniq_name(&mut self, prefix: &str) -> String {
        let name = format!("..{}..{}", prefix, self.uniq);
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

    pub fn add_tmp_ref(&mut self, tmp_ref: TmpRef) {
        let i = self.tmp_refs.len() - 1;
        self.tmp_refs[i].push(tmp_ref);
    }

    pub fn push_tmp_refs(&mut self) {
        self.tmp_refs.push(vec![]);
    }

    pub fn pop_tmp_refs(&mut self) -> Vec<TmpRef> {
        self.tmp_refs
            .pop()
            .expect("Attempting to pop when tmp_refs is empty")
    }
}
