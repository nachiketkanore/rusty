#![allow(dead_code)]
const LIMIT: i32 = 50;
// mod reader

mod math {
    pub fn prime(n: u32) -> bool {
        match n {
            0 | 1 => false,
            n => {
                for d in 2..(n as f64).sqrt() as u32 {
                    if n % d == 0 {
                        return false;
                    }
                }
                return true;
            }
        }
    }
}

fn factorial(n: u32) -> u32 {
    match n {
        0 => 1,
        n => n * factorial(n - 1),
    }
}

fn playground() {
    let x = 20;
    let y: &i32 = &x.clone();
    dbg!(x, y);
}

fn nachiket2() -> i32 {
    for (id, val) in (1..100).enumerate().map(|(id, val)| (id + 1, val)) {
        println!("{} {}", id, val);
    }
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("hello".to_string(), "greeting".to_string());
    let _x = (1..20)
        .map(|x| x * x)
        .filter(|x| x < &500)
        .filter(|x| x < &300)
        .filter(|x| x < &100)
        .max()
        .unwrap();

    let answer = (1..1000000)
        .filter(|x| x * x < 1000000)
        .map(|x| x * x)
        .collect::<Vec<i32>>();
    *answer.last().unwrap()
}

fn other_file() {
    dbg!(factorial(5));
    // reader::hello();
    let line = String::new();
    // reader::get_line(&mut line);
    println!("got line from stdin:\n{}", line);
}

fn print_value(a: i32) {
    println!("{}", a);

    let x = vec![1, 2, 3, 4];
    for item in &x {
        println!("{}", item);
    }
    for item in x {
        println!("{}", item);
    }
}

fn get_val() -> (i32, i32) {
    let x = 20;
    let y = 21;
    return (x, y);
}

#[allow(dead_code, unused)]
fn just_riding() {
    let x = vec![1, 2, 3];
    for &val in x.iter() {}
    for val in x.iter() {}
    for val in x.iter() {}
}

pub fn reverse(s: &str) -> String {
    let mut ans = String::new();
    for i in (0..s.len()).rev() {
        ans.push_str(&s[i..i + 1]);
    }
    println!("{}", s);
    return ans;
}

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MyError {
    code: usize,
}
impl Error for MyError {
    fn description(&self) -> &str {
        "Occurs when someone makes a mistake"
    }
}
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error code {:#X}", self.code)
    }
}
fn new_main() {
    println!("Display: {}", MyError { code: 1535 });
    println!("Debug: {:?}", MyError { code: 42 });
    // println!("Description: {:?}", (MyError { code: 42 }).description());
    let _val: i32 = "21".to_string().parse().expect("failed to parse");
}

fn nachiket() -> () {
    ()
}

fn main() {
    other_file();
    new_main();
    println!("{}", reverse("nachiket"));
    println!("{}", reverse("kanore"));
    println!("{}", reverse("asfda df a sdfads f as df a sfd"));
    let x = 21;
    let (a, b) = get_val();
    println!("{}, {}", a, b);
    print_value(x);
    print_value(x);
    println!("Welcome to RustWorld\n");
    for i in (1..10).step_by(2) {
        println!("{}", i);
    }
    println!("some iterator magic");
    for i in (1..10).map(|x| x * x).filter(|x| x < &LIMIT) {
        println!("{}", i);
    }
}
