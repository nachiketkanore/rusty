#[allow(unused_imports)]
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
    let n = scan.next::<usize>();
    let m = scan.next::<usize>();
    let mut mat = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            mat[i][j] = scan.next::<i32>();
        }
    }
    for j in 0..m {
        for i in 0..n {
            write!(out, "{} ", mat[i][j]).ok();
        }
        writeln!(out).ok();
    }
}
