use std::rc::Rc;

struct Handle<T>(Rc<T>);

impl<T> Handle<T> {
    pub fn new(value: T) -> Rc<T> {
        Rc::new(value)
    }
}
