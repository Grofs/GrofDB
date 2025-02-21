use std::collections::BTreeMap;

#[allow(non_camel_case_types)]
pub struct sparse_Index {
    pub index: BTreeMap<&str, usize>,
}

impl sparse_Index {
    pub fn new() -> Self {
        sparse_Index {
            index: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: &str, fpos: usize) {
        self.index.insert(key, fpos);
    }

    pub fn lookup(&self, key: &str) -> Option<(&str, usize)> {
        let table_index = self
            .index
            .range(..=key)
            .next_back()
            .map(|(&k, &pos)| (k, pos));
        table_index
    }
}
