use interfaces::Interface;
fn main() {
    let inf = match Interface::get_by_name("enp0s1") {
        Ok(Some(i)) => i,
        Ok(None) => {
            println!("None");
            return;
        }
        Err(e) => {
            println!("error: {}", e);
            return;
        }
    };

    for addr in inf.addresses.iter() {
        let raddr = match addr.addr {
            Some(a) => a,
            None => continue,
        };
        println!("{}", raddr);
    }
}
