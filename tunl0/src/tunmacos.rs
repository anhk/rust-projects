use libc::*;
use std::fs;
use std::io;
use std::os::fd::{AsRawFd, FromRawFd};

#[cfg(target_os = "macos")]
const UTUN_CONTROL_NAME: &'static str = "com.apple.net.utun_control";
#[cfg(target_os = "macos")]
const AF_SYSTEM: u8 = 32;
#[cfg(target_os = "macos")]
const AF_SYS_CONTROL: u16 = 2;

#[cfg(target_os = "macos")]
#[repr(C)]
pub struct CtlInfo {
    pub ctl_id: u32,
    pub ctl_name: [u8; 96],
}

#[cfg(target_os = "macos")]
pub struct Tun {
    pub handle: fs::File,
    pub ifname: String,
}

#[cfg(target_os = "macos")]
pub fn alloc_tun() -> Result<Tun, io::Error> {
    use std::mem;

    let fd = unsafe { socket(PF_SYSTEM, SOCK_DGRAM, SYSPROTO_CONTROL) };
    if fd < 0 {
        return Err(io::Error::last_os_error());
    }

    let handle = unsafe { fs::File::from_raw_fd(fd) };

    let mut info = CtlInfo {
        ctl_id: 0,
        ctl_name: {
            let mut buffer = [0u8; 96];
            buffer[..UTUN_CONTROL_NAME.len()].copy_from_slice(UTUN_CONTROL_NAME.as_bytes());
            buffer
        },
    };

    let res = unsafe { ioctl(handle.as_raw_fd(), CTLIOCGINFO, &mut info) };
    if res != 0 {
        return Err(io::Error::last_os_error());
    }
    let addr = sockaddr_ctl {
        sc_id: info.ctl_id,
        sc_len: mem::size_of::<sockaddr_ctl>() as u8,
        sc_family: AF_SYSTEM,
        ss_sysaddr: AF_SYS_CONTROL,
        sc_unit: 0 as u32,
        sc_reserved: [0; 5],
    };

    // If connect() is successful, a tun%d device will be created, where "%d"
    // is our sc_unit-1
    let res = unsafe {
        let addr_ptr = &addr as *const sockaddr_ctl;
        connect(
            handle.as_raw_fd(),
            addr_ptr as *const sockaddr,
            mem::size_of_val(&addr) as socklen_t,
        )
    };

    if res != 0 {
        return Err(io::Error::last_os_error());
    }

    let mut name_buf = [0u8; 64];
    let mut name_length: socklen_t = 64;
    let res = unsafe {
        getsockopt(
            handle.as_raw_fd(),
            SYSPROTO_CONTROL,
            UTUN_OPT_IFNAME,
            &mut name_buf as *mut _ as *mut c_void,
            &mut name_length as *mut socklen_t,
        )
    };
    if res != 0 {
        return Err(io::Error::last_os_error());
    }

    let res = unsafe { fcntl(handle.as_raw_fd(), F_SETFL, O_NONBLOCK) };
    if res == -1 {
        return Err(io::Error::last_os_error());
    }

    let res = unsafe { fcntl(handle.as_raw_fd(), F_SETFD, FD_CLOEXEC) };
    if res == -1 {
        return Err(io::Error::last_os_error());
    }

    let tun = Tun {
        handle: handle,
        ifname: {
            let len = name_buf.iter().position(|&r| r == 0).unwrap();
            String::from_utf8(name_buf[..len].to_vec()).unwrap()
        },
    };
    Ok(tun)
}
