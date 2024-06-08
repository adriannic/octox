use crate::sys::{clone, exit, join, Error};

#[derive(Debug)]
pub struct Thread(usize);

impl Drop for Thread {
    fn drop(&mut self) {
        let mut code = 0;
        let _ = join(self.0, &mut code);
    }
}

impl Thread {
    pub fn new<T>(f: impl Fn(T) -> i32, args: T) -> Self {
        match clone() {
            Err(_) => panic!("clone"),
            Ok(0) => exit(f(args)),
            Ok(tid) => Thread(tid),
        }
    }

    pub fn join(self) -> Result<i32, Error> {
        let mut code = 0;
        join(self.0, &mut code)?;
        Ok(code)
    }

    pub fn tid(&self) -> usize {
        self.0
    }
}
