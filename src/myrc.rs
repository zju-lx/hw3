struct Data<T> {
    value: T,
    counter: usize,
}
pub struct MyRc<T> (*mut Data<T>);

impl<T> MyRc<T> {
    pub fn new(val: T) -> Self {
        let data = Data {
            value: val,
            counter: 1
        };
        MyRc(Box::into_raw(Box::new(data)))
    }
    pub fn clone(&self) -> Self {
        unsafe{
        (*self.0).counter += 1;
        MyRc(self.0)
        }
    }
    pub fn strong_count(&self) -> usize {
        unsafe {
        (*self.0).counter
        }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
        (*self.0).counter -= 1;
        if (*self.0).counter == 0 {
            drop(self.0)
        }
        }
    }
}

use std::ops::Deref;
impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe{
        &(*self.0).value
        }
    }
}