mod tunlinux;
mod tunmacos;

// Reference: git@github.com:changlan/kytan.git
use std::io::{Read, Write};

fn main() {
    // let mut tun = create(10).expect("---");

    #[cfg(target_os = "linux")]
    let mut tun = tunlinux::alloc_tun().expect("");
    #[cfg(target_os = "macos")]
    let mut tun = tunmacos::alloc_tun().expect("");

    println!("ifname: {}", tun.ifname);

    let mut data = [0 as u8; 2048]; // mtu=1500 by default

    'l: while match tun.handle.read(&mut data) {
        Ok(size) => {
            println!("size: {}", size);
            if data[0] != 0x45 || data[9] != 1 {
                /* not ipv4 or icmp*/
                continue 'l;
            }
            let mut srcip = [0 as u8; 4];
            let mut dstip = [0 as u8; 4];
            dstip.copy_from_slice(&data[12..16]);
            srcip.copy_from_slice(&data[16..20]);

            // swap src and dst ip address
            data[12..16].copy_from_slice(&srcip);
            data[16..20].copy_from_slice(&dstip);

            data[20] = 0; // icmp echo

            let mut csum = ((data[22] as u16) << 8) | data[23] as u16;
            csum += 8;

            data[22] = (csum >> 8) as u8;
            data[23] = (csum & 0xF) as u8;

            tun.handle.write(&data[0..size]).unwrap();
            true
        }
        Err(err) => {
            println!("error: {}", err);
            false
        }
    } {}

    // unsafe { sleep(100) };
    // println!("Hello, world!");
}
