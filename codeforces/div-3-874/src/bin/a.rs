use std::{
    collections::HashSet,
    io::{stdin, stdout, BufWriter, Write},
};

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
        let _n: usize = scan.next();
        let util = scan.next::<String>().chars().collect::<Vec<char>>();
        writeln!(
            out,
            "{}",
            util.windows(2)
                .fold(HashSet::<&[char]>::new(), |mut set, window| {
                    set.insert(window);
                    set
                })
                .len()
        )
        .unwrap();
    }
}
