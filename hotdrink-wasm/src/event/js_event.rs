use hotdrink_rs::event::Event;

#[derive(Debug)]
pub struct JsEvent<T, E> {
    component: String,
    variable: String,
    event: Event<T, E>,
}

impl<T, E> JsEvent<T, E> {
    pub fn new(component: String, variable: String, event: Event<T, E>) -> Self {
        Self {
            component,
            variable,
            event,
        }
    }

    pub fn get_component(&self) -> &str {
        &self.component
    }

    pub fn get_variable(&self) -> &str {
        &self.variable
    }

    pub fn get_event(self) -> Event<T, E> {
        self.event
    }
}
