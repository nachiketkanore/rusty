use std::io::{stdin, stdout, BufWriter, Write};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let tcs: usize = scan.next();
    for _ in 0..tcs {
        let n: usize = scan.next();
        let mut even = 0;
        let mut odd = 0;
        let mut a = Vec::new();
        for _ in 0..n {
            let val: i32 = scan.next();
            a.push(val);
            if val % 2 == 1 {
                odd += 1;
            } else {
                even += 1;
            }
        }
        if even == n
            || odd == n
            || *a.iter().filter(|&val| val % 2 == 0).min().unwrap()
                > *a.iter().filter(|&val| val % 2 == 1).min().unwrap()
        {
            writeln!(out, "YES").unwrap();
        } else {
            writeln!(out, "NO").unwrap();
        }
    }
}
