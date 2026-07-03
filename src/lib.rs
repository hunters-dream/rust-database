mod transaction;
use std::collections::HashMap;

#[derive(Debug)]
struct Database {
    data: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn delete(&mut self, key: &str) {
        self.data.remove(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut db = Database::new();

        db.insert("k".to_string(), "v".to_string());

        assert_eq!(db.get("k"), Some(&"v".to_string()))
    }

    #[test]
    fn commit_is_visible() {
        let mut db = Database::new();

        let mut tnx = db.begin();
        tnx.set("k".to_string(), "v".to_string());
        tnx.commit();

        assert!(db.get("k") == Some(&"v".to_string()))
    }

    #[test]
    fn drop_without_commit_is_rolled_back() {
        let mut db = Database::new();

        {
            let mut tnx = db.begin();
            tnx.set("k".to_string(), "v".to_string());
        }
        assert_eq!(db.get("k"), None);
    }
}
