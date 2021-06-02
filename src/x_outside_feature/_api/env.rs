use std::env::var;

pub struct Env {}

impl Env {
    pub fn new() -> Self {
        Self {}
    }
}

impl Env {
    pub fn load(&self, key: &'static str) -> String {
        var(key).expect(format!("env not specified: {}", key).as_str())
    }
}
