use std::thread::{self, JoinHandle};

fn main() {
    let mut handles = Vec::new();
    for i in 0..5 {
        println!("Hello, world {}!", i);
        handles.push(thread::spawn(move || inthread(i)));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn inthread(i: i32) {
    println!("in thread {}", i)
}
