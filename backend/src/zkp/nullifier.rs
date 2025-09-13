use anyhow::Result;
use std::collections::HashSet;

pub struct NullifierManager {
    nullifiers: HashSet<String>,
}

impl NullifierManager {
    pub fn new() -> Self {
        Self {
            nullifiers: HashSet::new(),
        }
    }

    pub fn add_nullifier(&mut self, nullifier: String) -> bool {
        self.nullifiers.insert(nullifier)
    }

    pub fn contains_nullifier(&self, nullifier: &str) -> bool {
        self.nullifiers.contains(nullifier)
    }

    pub fn is_nullifier_used(&self, nullifier: &str) -> bool {
        self.contains_nullifier(nullifier)
    }
}
