mod tunlinux;

// Reference: git@github.com:changlan/kytan.git
use std::io::Read;

fn main() {
    // let mut tun = create(10).expect("---");
    let mut tun = tunlinux::alloc_tun().expect("");
    println!("ifname: {}", tun.ifname);

    let mut data = [0 as u8, 255];

    while match tun.handle.read(&mut data) {
        Ok(size) => {
            println!("size: {}", size);
            true
        }
        Err(_) => false,
    } {}

    // unsafe { sleep(100) };
    // println!("Hello, world!");
}
