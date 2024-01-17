#[cfg(target_os = "macos")]
use libc::*;
#[cfg(target_os = "macos")]
use std::fs;
#[cfg(target_os = "macos")]
use std::io;
#[cfg(target_os = "macos")]
use std::os::fd::FromRawFd;
use std::process;

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
        println!("error on {}:{}", file!(), line!());
        return Err(io::Error::last_os_error());
    }

    let mut ctl_name = [0u8; 96];
    ctl_name[..UTUN_CONTROL_NAME.len()].copy_from_slice(UTUN_CONTROL_NAME.as_bytes());
    let mut info = CtlInfo {
        ctl_id: 0,
        ctl_name: ctl_name,
    };

    let res = unsafe { ioctl(fd, CTLIOCGINFO, &mut info) };
    if res != 0 {
        println!("error on {}:{}", file!(), line!());
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
    let addr_ptr = &addr as *const sockaddr_ctl;
    let res = unsafe {
        connect(
            fd,
            addr_ptr as *const sockaddr,
            mem::size_of_val(&addr) as socklen_t,
        )
    };

    if res != 0 {
        println!("error on {}:{}", file!(), line!());
        return Err(io::Error::last_os_error());
    }

    let mut name_buf = [0u8; 64];
    let mut name_length: socklen_t = 64;
    let res = unsafe {
        getsockopt(
            fd,
            SYSPROTO_CONTROL,
            UTUN_OPT_IFNAME,
            &mut name_buf as *mut _ as *mut c_void,
            &mut name_length as *mut socklen_t,
        )
    };
    if res != 0 {
        println!("error on {}:{}", file!(), line!());
        return Err(io::Error::last_os_error());
    }

    // if unsafe { fcntl(fd, F_SETFL, O_NONBLOCK) } == -1 {
    //     println!("error on {}:{}", file!(), line!());
    //     return Err(io::Error::last_os_error());
    // }

    if unsafe { fcntl(fd, F_SETFD, FD_CLOEXEC) } == -1 {
        println!("error on {}:{}", file!(), line!());
        return Err(io::Error::last_os_error());
    }

    let tun = Tun {
        handle: unsafe { fs::File::from_raw_fd(fd) },
        ifname: {
            let len = name_buf.iter().position(|&r| r == 0).unwrap();
            String::from_utf8(name_buf[..len].to_vec()).unwrap()
        },
    };

    tun.up();
    // let s = unsafe {
    //     let mut buffer = [0u8; 4096];
    //     read(fd, buffer.as_mut_ptr() as *mut c_void, 4096)
    // };
    // println!("read size: {}", s);
    Ok(tun)
}

impl Tun {
    #[cfg(target_os = "macos")]
    pub fn up(&self) {
        let status = process::Command::new("ifconfig")
            .arg(self.ifname.clone())
            .arg("10.10.10.253")
            .arg("10.10.10.1")
            .arg("up")
            .status()
            .unwrap();
        assert!(status.success());

        let status = process::Command::new("ifconfig")
            .arg(self.ifname.clone())
            .arg("mtu")
            .arg("1500")
            .arg("up")
            .status()
            .unwrap();
        assert!(status.success());
    }
}
