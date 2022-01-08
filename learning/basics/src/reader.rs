pub fn hello() {
    println!("Hello Nachiket");
}

#[allow(dead_code)]
pub fn get_line(line: &mut String) {
    // reads a line from stdin and returns to the caller
    use std::io;

    io::stdin().read_line(line).expect("unable to read line");
}

fn works() -> Result<i32, std::num::ParseIntError> {
    let x: i32 = "21".to_string().parse::<i32>()?;
    Ok(x)
}

fn check() {
    let mut a = Vec::new();
    let first = 20;
    let second = 10;

    if 2 * second == first {
        a.push(20);
    } else {
        a.push("nachiket");
    }

    a.push("hello");
}

// this is separate module hence compiler warns about it being unused
#[allow(dead_code)]
fn main() {
    use std::io;

    check();

    let mut my_line = String::new();

    io::stdin()
        .read_line(&mut my_line)
        .expect("unable to read line from stdin");

    println!("{}", my_line);
}
