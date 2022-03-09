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
    let s = scan.next::<String>().chars().collect::<Vec<char>>();
    let k = scan.next::<usize>();
    let n = s.len();
    let mut ans = 0;

    let (mut lo, mut hi) = (1, n);

    let exists = |window: usize| -> bool {
        let mut dots = 0;
        for i in 0..window {
            dots += (s[i] == '.') as usize;
        }
        match dots <= k {
            true => true,
            false => {
                for i in window..n {
                    dots += (s[i] == '.') as usize;
                    dots -= (s[i - window] == '.') as usize;
                    if dots <= k {
                        return true;
                    }
                }
                false
            }
        }
    };

    while lo <= hi {
        let mid = (lo + hi) / 2;
        if exists(mid) {
            ans = mid;
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }

    writeln!(out, "{}", ans).ok();
}
