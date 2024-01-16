use std::os::fd::AsRawFd;

use smoltcp::{
    iface::{Config, SocketSet},
    phy::TunTapInterface,
    time::Instant,
    wire::{HardwareAddress, IpAddress, IpCidr, Ipv4Address},
};
fn main() {
    let mut device = TunTapInterface::new("tun0", smoltcp::phy::Medium::Ip).unwrap();
    let fd = device.as_raw_fd();
    println!("fd: {}", fd);

    let config = Config::new(HardwareAddress::Ip);
    let mut iface = smoltcp::iface::Interface::new(config, &mut device, Instant::now());
    iface.update_ip_addrs(|ipaddrs| {
        ipaddrs
            .push(IpCidr::new(IpAddress::v4(10, 0, 0, 1), 24))
            .unwrap();
        println!("update it .");
    });

    iface
        .routes_mut()
        .add_default_ipv4_route(Ipv4Address::new(10, 0, 0, 10))
        .unwrap();

    loop {
        let timestamp = Instant::now();
        let mut sockets = SocketSet::new(vec![]);
        iface.poll(timestamp, &mut device, &mut sockets);
        std::thread::park();
    }
}
