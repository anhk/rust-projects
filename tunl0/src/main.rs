mod tunlinux;

// Reference: git@github.com:changlan/kytan.git
use std::io::{Read, Write};

fn main() {
    // let mut tun = create(10).expect("---");
    let mut tun = tunlinux::alloc_tun().expect("");
    println!("ifname: {}", tun.ifname);

    let mut data = [0 as u8; 2048]; // mtu=1500 by default

    'l: while match tun.handle.read(&mut data) {
        Ok(size) => {
            println!("size: {}", size);
            if data[0] != 0x45 || data[9] != 1 {
                /* not ipv4 or icmp*/
                continue 'l;
            }
            println!("data: {:x?}", &data[..size]);
            let mut srcip = [0 as u8; 4];
            let mut dstip = [0 as u8; 4];
            dstip.copy_from_slice(&data[12..16]);
            srcip.copy_from_slice(&data[16..20]);

            data[12..16].copy_from_slice(&srcip);
            data[16..20].copy_from_slice(&dstip);

            data[20] = 0; // icmp echo

            // *((unsigned short *)&buffer[22]) += 8; // checksum

            println!("data: {:x?}", &data[..size]);

            // tun.handle.write(&data[0..20]).unwrap();
            true
        }
        Err(_) => false,
    } {}

    // unsafe { sleep(100) };
    // println!("Hello, world!");
}
