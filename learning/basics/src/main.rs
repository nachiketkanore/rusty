const LIMIT: i32 = 50;

fn print_value(a: i32) {
    println!("{}", a);
}

fn get_val() -> (i32, i32) {
    let x = 20;
    let y = 21;
    return (x, y);
}

#[allow(dead_code, unused)]
fn just_riding() {
    let x = vec![1, 2, 3];
    for &val in x.iter() {
    }
    for val in x.iter() {
    }
    for val in x.iter() {
    }
}

pub fn reverse(s: &str) -> String {
    let mut ans = String::new();
    for i in (0..s.len()).rev() {
        ans.push_str(&s[i..i+1]);
    }
    println!("{}", s);
    return ans;
}

fn main() {
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
