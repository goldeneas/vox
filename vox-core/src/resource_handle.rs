use std::rc::{Rc, Weak};

pub enum Handle<T> {
    Strong(Rc<T>),
    Weak(Weak<T>),
}
