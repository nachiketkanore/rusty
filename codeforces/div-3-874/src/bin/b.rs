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
        let (n, _k): (usize, usize) = (scan.next(), scan.next());
        let mut a: Vec<(i32, usize)> = (0..n).map(|id| (scan.next::<i32>(), id)).collect();
        let mut b: Vec<i32> = (0..n).map(|_| scan.next::<i32>()).collect();
        a.sort();
        b.sort();
        let mut ans = vec![0i32; n];
        for i in 0..n {
            ans[a[i].1] = b[i];
        }
        for x in ans {
            write!(out, "{} ", x).unwrap();
        }
        writeln!(out, "").unwrap();
    }
}
