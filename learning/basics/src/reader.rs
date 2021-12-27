pub fn hello() {
    println!("Hello Nachiket");
}

#[allow(dead_code)]
pub fn get_line(line: &mut String) {
    // reads a line from stdin and returns to the caller
    use std::io;

    io::stdin().read_line(line).expect("unable to read line");
}

// this is separate module hence compiler warns about it being unused
#[allow(dead_code)]
fn main() {
    use std::io;

    let mut my_line = String::new();

    io::stdin()
        .read_line(&mut my_line)
        .expect("unable to read line from stdin");

    println!("{}", my_line);
}
