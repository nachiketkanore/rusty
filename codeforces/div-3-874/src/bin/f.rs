use std::{
    collections::{HashMap, HashSet},
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
    // brute solution
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());
    const MOD: u64 = 1e9 as u64 + 7;

    let tcs: usize = scan.next();
    for _ in 0..tcs {
        let (n, m): (usize, usize) = (scan.next(), scan.next());
        let mut a: Vec<u32> = (0..n).map(|_| scan.next()).collect();
        a.sort();
        // dbg!(&a);
        // compress values
        // {
        //     let mut value_map: HashMap<u32, u32> = HashMap::new();
        //     let mut compressed_values: Vec<u32> = Vec::new();
        //     let mut compressed_value = 1;
        //
        //     for &value in a.iter() {
        //         if !value_map.contains_key(&value) {
        //             value_map.insert(value, compressed_value);
        //             compressed_value += 1;
        //         }
        //         compressed_values.push(*value_map.get(&value).unwrap());
        //     }
        //     a = compressed_values;
        // }
        let mut frequencies = HashMap::<u32, usize>::new();
        for &val in &a {
            *frequencies.entry(val).or_insert(0) += 1;
        }
        let mut unique_values: Vec<_> = a.into_iter().collect::<HashSet<_>>().into_iter().collect();
        let mut ans = 0;
        unique_values.sort();

        for val in unique_values {
            let mut prod: u64 = 1;
            for j in val..(val + m as u32) {
                if let Some(value) = frequencies.get(&j) {
                    prod *= *value as u64;
                } else {
                    prod = 0;
                }
                prod %= MOD;
            }
            ans += prod;
            if ans >= MOD {
                ans -= MOD;
            }
        }

        writeln!(out, "{}", ans).unwrap();
    }
}
