#![no_std]
use ulib::{alloc::sync::Arc, mutex::Mutex, print, println, sys, thread::Thread};

fn thread_fn(value: i32) -> i32 {
    if value & 1 == 1 {
        println!("This thread will call exit with code {}", value);
        sys::exit(value);
    }
    println!("This thread will return with code {}", value);
    value
}

fn lock_test(lock: Arc<Mutex<i32>>) -> i32 {
    for _ in 0..100000 {
        let mut var = lock.lock();
        *var += 1;
    }
    0
}

fn main() {
    for expected_code in 1..=2 {
        let thread = Thread::new(thread_fn, expected_code);
        let code = thread.join().unwrap();
        println!("Thread exit code: {}", code);
        assert_eq!(code, expected_code);
    }

    println!("Testing locks...");

    let var = Arc::new(Mutex::new(0));
    let thread1 = Thread::new(lock_test, var.clone());
    let thread2 = Thread::new(lock_test, var.clone());
    thread1.join();
    thread2.join();

    println!("Value of var: {}", *var.lock());

    assert_eq!(*var.lock(), 200000);

    println!("TEST PASSED!");
}
