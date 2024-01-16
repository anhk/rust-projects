use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
};

fn main() {
    // 删除目录
    fs::remove_dir_all("./test_dir").unwrap_or(());

    // 创建目录
    fs::create_dir("./test_dir").expect("create directory");
    fs::create_dir_all("./test_dir/a/b/c/").expect("create directory all");

    // 创建文件并写入内容
    fs::write("./test_dir/hello.txt", "Hello World.").unwrap();

    // 追加内容
    let mut f2 = OpenOptions::new()
        .append(true)
        .open("./test_dir/hello.txt")
        .unwrap();
    f2.write(b"Show me the world").unwrap();

    // 读内容
    let mut buf = [0u8; 16];
    let mut f = fs::File::open("./test_dir/hello.txt").unwrap();
    f.read(&mut buf).unwrap();
    println!("{:?}", buf);

    // 读并写
    let mut f3 = OpenOptions::new()
        .read(true)
        .write(true) // 这是从头写的.
        .open("./test_dir/hello.txt")
        .unwrap();
    f3.read(&mut buf).unwrap();
    f3.write(&buf).unwrap();
}
