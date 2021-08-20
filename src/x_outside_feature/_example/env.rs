use std::env::var;

pub struct Env {
    pub port: String,
}

impl Env {
    pub fn new() -> Self {
        Self { port: load("PORT") }
    }
}

fn load(key: &'static str) -> String {
    var(key).expect(format!("env not specified: {}", key).as_str())
}
