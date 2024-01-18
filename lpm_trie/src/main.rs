use ipnet::Ipv4Net;
use prefix_trie::PrefixMap;

fn main() {
    let mut pm: PrefixMap<Ipv4Net, i32> = PrefixMap::new();
    pm.insert("192.168.0.0/22".parse().unwrap(), 1);
    pm.insert("192.168.0.0/23".parse().unwrap(), 2);
    pm.insert("192.168.0.0/24".parse().unwrap(), 3);
    pm.insert("192.168.1.0/25".parse().unwrap(), 4);
    pm.insert("192.168.2.0/25".parse().unwrap(), 5);

    let prefix: Ipv4Net = "192.168.1.123/32".parse().unwrap();
    let p = match pm.get_lpm(&prefix) {
        Some((_, b)) => b,
        None => return,
    };
    println!("p: {}", p);

    let p2 = match pm.get_lpm_mut(&("192.168.2.0/25".parse().unwrap())) {
        Some((_, b)) => b,
        None => return,
    };
    println!("p2: {}", p2);
}
