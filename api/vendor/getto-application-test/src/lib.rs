use std::sync::{Arc, Mutex};

pub struct ApplicationActionStateHolder(Arc<Mutex<Vec<String>>>);

impl ApplicationActionStateHolder {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(Vec::new())))
    }

    pub fn handler<S: std::fmt::Display>(&self) -> impl 'static + Fn(&S) + Send + Sync {
        let runner = Arc::clone(&self.0);
        move |state| {
            runner.lock().unwrap().push(format!("{}", state));
        }
    }

    pub fn extract(self) -> Vec<String> {
        self.0.lock().unwrap().clone()
    }
}
