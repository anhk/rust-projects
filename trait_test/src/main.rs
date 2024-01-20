// fn add<T>(a: T, b: T) -> T {
//     a + b
// }
use std::io::{Read, Result};

fn largest<T>(list: &[T]) -> &T
where
    T: std::cmp::PartialOrd,
{
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

trait Interface {
    fn to_string(&self) -> String;
}

struct A {
    a: String,
}

impl Interface for A {
    fn to_string(&self) -> String {
        self.a.clone()
    }
}

impl Read for A {
    fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
        Ok(0)
    }
}

struct B {
    b: String,
}

impl Interface for B {
    fn to_string(&self) -> String {
        self.b.clone()
    }
}

enum IntF {
    A,
    B,
}

fn alloc_intf(isA: bool) -> IntF {
    if isA {
        &A {
            a: String::from("A"),
        }
    } else {
        &B {
            b: String::from("B"),
        }
    }
}

fn main() {
    let list = vec![3, 4, 2, 3, 2];
    println!("largest: {}", largest(&list));

    let mut x = A {
        a: String::from("Hello A"),
    };

    let y = B {
        b: String::from("Hello B"),
    };

    println!("x: {}, y: {}", x.to_string(), y.to_string());
    let mut buf = [0u8; 256];
    println!("read: {}", x.read(&mut buf).unwrap());

    // println!("{}", add(3, 4));
    println!("Hello, world!");
}
