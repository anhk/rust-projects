use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1{
        println!("{} ip:port", args[0]);
    }
    println!("args: {}", args[0]);
}
