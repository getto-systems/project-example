pub struct ActionStatePubSub<E> {
    handlers: Vec<Box<dyn Fn(&E)>>,
}

impl<E> ActionStatePubSub<E> {
    pub fn new() -> Self {
        Self { handlers: vec![] }
    }

    pub fn subscribe(&mut self, handler: impl 'static + Fn(&E)) {
        self.handlers.push(Box::new(handler));
    }
    pub fn post(&self, event: E) -> E {
        self.handlers.iter().for_each(|handler| handler(&event));
        event
    }
}
