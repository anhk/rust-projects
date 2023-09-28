mod user;

fn a() {
    let s: &str = "hello, world";
    println!("a() s={}", s);
}

fn b() {
    let mut s = String::from("hello");
    s.push_str(" world");
    s.push('!');
    println!("b() s={}", s)
}

type File = String;
fn c(_: &mut File) -> bool {
    true
}

fn greet(name: String) {
    println!("Hello, {}!", name);
}

fn _if(cond: bool, a: String, b: String) -> String {
    if cond {
        return a;
    }
    return b;
}

fn string() {
    a();
    b();

    let mut f = File::from("a");
    println!("c={}", c(&mut f));

    let my = "Pascal";
    greet(my.to_string());

    let mut s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..s.len()];
    println!("{} {} -> {}", hello, world, &s[..]);
    s.clear();
    // println!("s:{}", _if(s.len() == 0, String::from("nil"), s));
    println!("s:{}", if s.len() == 0 { String::from("nil") } else { s });

    let s = "中国人"; // 9个字节
    println!("len: {}", s.len())

    // let a = &s[0..2];
    // println!("{}",a);
}

fn array() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    for i in a.iter() {
        print!("{} ", i)
    }
    println!("")
}

fn _loop() {
    for i in 0..5 {
        print!("{} ", i + 1)
    }
    println!("")
}

fn _match(i: i32) {
    match i {
        1 => println!("> {}", i),
        3 => println!("> {}", i),
        _ => (),
    }
}

enum Number {
    Zero,
    One,
    Two,
}

fn number(i: Number) {
    match i {
        Number::One => println!("{}", "one"),
        _ => (),
    }
}

fn main() {
    string();
    array();
    _loop();
    _match(3);
    let user = user::User {
        active: true,
        username: String::from("张三丰"),
        email: String::from("abc@good.com"),
        sign_in_count: 64,
    };

    println!("{}", user.string());
    number(Number::One);
    number(Number::Zero);
    number(Number::Two);
}
