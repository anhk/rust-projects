#[cfg(target_os = "linux")]
use libc::*;
#[cfg(target_os = "linux")]
use std::fs;
#[cfg(target_os = "linux")]
use std::io;
#[cfg(target_os = "linux")]
use std::io::{Read, Write};
#[cfg(target_os = "linux")]
use std::os::fd::AsRawFd;

#[cfg(target_os = "linux")]
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

#[cfg(target_os = "linux")]
pub struct IoctlFlagsData {
    pub ifr_name: [u8; IFNAMSIZ],
    pub ifr_flags: i16,
}

#[cfg(target_os = "linux")]
pub fn alloc_tun() -> Result<Tun, io::Error> {
    use std::fs::OpenOptions;

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/net/tun")
        .expect("");
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

#[cfg(target_os = "linux")]
impl Read for Tun {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.handle.read(buf)
    }
}
#[cfg(target_os = "linux")]
impl Write for Tun {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        self.handle.write(buf)
    }
    fn flush(&mut self) -> Result<(), std::io::Error> {
        self.handle.flush()
    }
}
