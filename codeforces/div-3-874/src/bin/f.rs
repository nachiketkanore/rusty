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
        let mut prefix_frequencies = vec![0u64; n + 1];
        for (id, val) in unique_values.iter().enumerate() {
            prefix_frequencies[id] = if let Some(value) = frequencies.get(val) {
                *value as u64 % MOD
            } else {
                0
            };
            if id > 0 {
                prefix_frequencies[id] *= prefix_frequencies[id - 1];
                prefix_frequencies[id] %= MOD;
            }
        }

        let get_index = |r: u32| -> Option<usize> {
            let mut lo = 0;
            let mut hi = unique_values.len() - 1;
            let mut ans: i32 = -1;

            while lo <= hi {
                let mid = (lo + hi) / 2;
                if unique_values[mid] <= r {
                    ans = mid as i32;
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }
            if ans >= 0 {
                Some(ans as usize)
            } else {
                None
            }
        };

        fn power(mut a: u64, mut b: u64) -> u64 {
            let mut ans = 1;
            while b > 0 {
                if b % 2 == 1 {
                    ans = ans * a % MOD;
                }
                a = a * a % MOD;
                b /= 2;
            }
            ans
        }

        fn inverse_mod(val: u64) -> u64 {
            power(val, MOD - 2)
        }

        let get_answer = |l: u32, r: u32| -> u64 {
            let mut prod: u64 = 0;
            if let Some(idr) = get_index(r) {
                prod = prefix_frequencies[idr];
                if let Some(idl) = get_index(l) {
                    if idl >= 1 {
                        prod *= inverse_mod(prefix_frequencies[idl - 1]);
                        prod %= MOD;
                    }
                }
            }
            prod
        };

        for val in &unique_values {
            // let mut prod: u64 = 1;
            // for j in val..(val + m as u32) {
            //     if let Some(value) = frequencies.get(&j) {
            //         prod *= *value as u64;
            //     } else {
            //         prod = 0;
            //     }
            //     prod %= MOD;
            // }
            ans += get_answer(*val, val + m as u32 - 1);
            if ans >= MOD {
                ans -= MOD;
            }
        }

        writeln!(out, "{}", ans).unwrap();
    }
}
