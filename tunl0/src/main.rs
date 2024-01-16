mod tunlinux;

// Reference: git@github.com:changlan/kytan.git
use std::io::{Read, Write};

fn main() {
    // let mut tun = create(10).expect("---");
    let mut tun = tunlinux::alloc_tun().expect("");
    println!("ifname: {}", tun.ifname);

    let mut data = [0 as u8; 2048]; // mtu=1500 by default

    while match tun.handle.read(&mut data) {
        Ok(size) => {
            println!("size: {}", size);
            // if size < 20 {
            //     return;
            // }
            // let mut srcip = [0 as u8; 4];
            // let mut dstip = [0 as u8; 4];
            // srcip.copy_from_slice(&data[12..16]);
            // dstip.copy_from_slice(&data[16..20]);
            // data[12..16].copy_from_slice(&dstip);
            // data[16..20].copy_from_slice(&srcip);
            // match tun.handle.write(&data[0..20]) { // EBADF 9 Bad file descriptor
            //     Ok(size) => println!("write size: {}", size),
            //     Err(err) => println!("write error: {}", err),
            // }
            true
        }
        Err(_) => false,
    } {}

    // unsafe { sleep(100) };
    // println!("Hello, world!");
}
