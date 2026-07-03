use crate::Database;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Transaction<'a> {
    db: &'a mut Database,
    pending: HashMap<String, Option<String>>,
}

impl Database {
    pub fn begin(&mut self) -> Transaction {
        Transaction {
            db: self,
            pending: HashMap::new(),
        }
    }
}

impl<'a> Transaction<'a> {
    pub fn set(&mut self, key: String, value: String) {
        self.pending.insert(key, Some(value));
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        match self.pending.get(key) {
            Some(Some(v)) => Some(v),
            Some(None) => None,
            None => self.db.get(key),
        }
    }

    pub fn delete(&mut self, key: String) {
        self.pending.insert(key, None);
    }

    pub fn commit(self) {
        for (key, value) in self.pending {
            match value {
                Some(v) => self.db.insert(key, v),
                None => self.db.delete(&key),
            }
        }
    }
}
