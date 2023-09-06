use std::cell::RefCell;

pub struct SimpleStack<T> {
    stack: RefCell<Vec<T>>,
}

impl<T> SimpleStack<T> {
    pub fn new() -> Self {
        SimpleStack { stack: RefCell::new(Vec::new()) }
    }

    pub fn push(&self, item: T) {
        self.stack.borrow_mut().push(item);
    }

    pub fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }
}