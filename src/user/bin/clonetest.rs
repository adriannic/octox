#![no_std]
use ulib::{print, println, sys};

fn clone<T>(f: impl Fn(&[T]) -> i32, args: &[T]) -> usize {
    match sys::clone() {
        Err(_) => panic!("clone"),
        Ok(0) => sys::exit(f(args)),
        Ok(tid) => return tid,
    }
}

fn thread_fn(args: &[i32]) -> i32 {
    let value = args[0];
    if value & 1 == 1 {
        println!("This thread will call exit with code {}", value);
        sys::exit(value);
    }
    println!("This thread will return with code {}", value);
    value
}

fn main() {
    for expected_code in 1..=2 {
        let tid = clone(thread_fn, &[expected_code]);
        let mut code = 0;
        sys::join(tid, &mut code);
        println!("Thread exit code: {}", code);
        assert_eq!(code, expected_code);
    }
}
