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

        db.insert("key".to_string(), "value".to_string());
    }
}
