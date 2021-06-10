use std::{fmt::Display, rc::Rc, sync::Mutex};

use pretty_assertions::assert_eq;

pub struct ActionTestRunner {
    store: Mutex<Vec<String>>,
}

impl ActionTestRunner {
    pub fn new<S: Display>() -> (impl Fn(&S), impl Fn(Vec<&str>)) {
        let runner = Rc::new(Self {
            store: Mutex::new(vec![]),
        });
        let handler_runner = Rc::clone(&runner);
        (
            move |state| handler_runner.push(state),
            move |expect| runner.assert(expect),
        )
    }

    fn push(&self, state: &impl Display) {
        let mut store = self.store.lock().unwrap();
        store.push(format!("{}", state))
    }
    fn assert(&self, expect: Vec<&str>) {
        assert_eq!(self.store.lock().unwrap().clone(), expect)
    }
}
