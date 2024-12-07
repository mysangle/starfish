use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct Headers {
    headers: HashMap<String, String>,
}

impl Headers {
    pub fn new() -> Headers {
        Headers {
            headers: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Headers {
        Headers {
            headers: HashMap::with_capacity(capacity),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }
}
