use std::{
    rc::Rc,
    sync::{mpsc, Arc, Mutex},
    thread::{self},
};

fn main() {
    let mut handles = Vec::new();

    // Channel
    let (tx, rx) = mpsc::channel();
    handles.push(thread::spawn(move || {
        tx.send(String::from("Hey")).unwrap();
    }));

    let data = rx.recv().unwrap();
    println!("data from channel: {}", data);
    println!("================");

    // MultiThreads
    for i in 0..5 {
        println!("Hello, world {}!", i);
        handles.push(thread::spawn(move || inthread(i)));
    }

    for handle in handles {
        handle.join().unwrap();
    }
    // handles.clear();
    println!("================");

    // Mutex
    let m = Mutex::new(5);

    let mut n = m.lock().unwrap();
    (*n) += 1;
    drop(n);

    let n2 = m.lock().unwrap();
    println!("n2: {}", n2);
    drop(n2);

    println!("================");
    let mut handles = Vec::new();
    // RC
    let counter = Arc::new(m);
    for _ in 0..10 {
        let nc = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut n = nc.lock().unwrap();
            *n += 1;
        }))
    }

    let mut n = counter.lock().unwrap();
    (*n) += 1;
    drop(n);

    for handle in handles {
        handle.join().unwrap();
    }
}

fn inthread(i: i32) {
    println!("in thread {}", i)
}
