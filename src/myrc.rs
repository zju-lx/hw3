use std::ptr::NonNull;
use std::cell::RefCell;
struct Data<T> {
    value: T,
    counter: usize,
}
pub struct MyRc<T> (RefCell<NonNull<Data<T>>>);

impl<T> MyRc<T> {
    pub fn new(val: T) -> Self {
        let data = Data {
            value: val,
            counter: 1
        };
        MyRc(RefCell::new(NonNull::new(Box::into_raw(Box::new(data))).expect("Box is null")))
    }
    pub fn clone(&self) -> Self {
        unsafe{
        self.0.borrow_mut().as_mut().counter += 1;
        MyRc(RefCell::clone(&self.0))
        }
    }
    pub fn strong_count(&self) -> usize {
        unsafe {
        self.0.borrow_mut().as_ref().counter
        }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
        self.0.borrow_mut().as_mut().counter -= 1;
        if self.0.borrow_mut().as_ref().counter == 0 {
            drop(*self.0.borrow_mut())
        }
        }
    }
}

use std::ops::Deref;
impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe{
        &self.0.borrow_mut().as_ref().value
        }
    }
}