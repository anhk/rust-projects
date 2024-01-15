use libc::*;
use std::fs;
use std::io;
use std::os::fd::AsRawFd;

pub struct Tun {
    pub handle: fs::File,
    pub ifname: String,
}

#[cfg(target_os = "linux")]
const IFNAMSIZ: usize = 16;
#[cfg(target_os = "linux")]
const IFF_TUN: i16 = 0x0001;
#[cfg(target_os = "linux")]
const IFF_NO_PI: i16 = 0x1000;
#[cfg(target_os = "linux")]
const TUNSETIFF: u64 = 0x400454ca; // TODO: use _IOW('T', 202, int)

pub struct IoctlFlagsData {
    pub ifr_name: [u8; IFNAMSIZ],
    pub ifr_flags: i16,
}

#[cfg(target_os = "linux")]
pub fn alloc_tun() -> Result<Tun, io::Error> {
    let file = fs::File::open("/dev/net/tun").expect("");
    let mut req = IoctlFlagsData {
        ifr_name: [0u8; IFNAMSIZ],
        ifr_flags: IFF_TUN | IFF_NO_PI,
    };
    if unsafe { ioctl(file.as_raw_fd(), TUNSETIFF, &mut req) } < 0 {
        return Err(io::Error::last_os_error());
    }

    let size = req.ifr_name.iter().position(|&r| r == 0).unwrap();

    let tun = Tun {
        handle: file,
        ifname: String::from_utf8(req.ifr_name[..size].to_vec()).unwrap(),
    };
    Ok(tun)
}
