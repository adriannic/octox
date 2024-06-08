#![no_std]
use ulib::{print, println, sys, thread::Thread};

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
        let thread = Thread::new(thread_fn, &[expected_code]);
        let code = thread.join().unwrap();
        println!("Thread exit code: {}", code);
        assert_eq!(code, expected_code);
    }
}
