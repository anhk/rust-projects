use std::{
    fs,
    io::{self, Read},
    os::fd::AsRawFd,
    path,
};

use libc::ioctl;

const IFNAMSIZ: usize = 16;
const IFF_TUN: i16 = 0x0001;
const IFF_NO_PI: i16 = 0x1000;
const TUNSETIFF: u64 = 0x400454ca; // TODO: use _IOW('T', 202, int)

pub struct IoctlFlagsData {
    pub ifr_name: [u8; IFNAMSIZ],
    pub ifr_flags: i16,
}

struct Tun {
    handle: fs::File,
    if_name: String,
}

#[cfg(target_os = "linux")]
fn create(name: u8) -> Result<Tun, io::Error> {
    let path = path::Path::new("/dev/net/tun");
    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path)
        .expect("open /dev/net/tun failed");

    let mut req = IoctlFlagsData {
        ifr_name: {
            let mut buffer = [0u8; IFNAMSIZ];
            let full_name = format!("tun{}", name);
            buffer[..full_name.len()].clone_from_slice(full_name.as_bytes());
            buffer
        },
        ifr_flags: IFF_TUN | IFF_NO_PI,
    };

    let res = unsafe { ioctl(file.as_raw_fd(), TUNSETIFF, &mut req) };
    if res < 0 {
        return Err(io::Error::last_os_error());
    }

    let size = req.ifr_name.iter().position(|&r| r == 0).unwrap();

    let tun = Tun {
        handle: file,
        if_name: String::from_utf8(req.ifr_name[..size].to_vec()).unwrap(),
    };
    Ok(tun)
}

#[cfg(target_os = "macos")]
fn create(name: u8) -> Result<Tun, io::Error> {
    Err(io::Error::last_os_error())
}

// Reference: git@github.com:changlan/kytan.git

fn main() {
    let mut tun = create(10).expect("---");
    println!("ifname: {}", tun.if_name);

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
