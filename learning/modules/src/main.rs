mod folder1;
use folder1::file1::hello as hello1;
use folder1::file2::hello as hello2;

fn main() {
    println!("Hello, world!");
    hello1();
    hello2();
}
