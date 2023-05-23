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
        let mut adj: Vec<Vec<(usize, usize)>> = vec![Vec::default(); n];
        for eid in 1..n {
            let u = scan.next::<usize>() - 1;
            let v = scan.next::<usize>() - 1;
            adj[u].push((v, eid));
            adj[v].push((u, eid));
        }
        let mut to_delete = Vec::new();

        fn dfs(
            curr: usize,
            par: usize,
            adj: &Vec<Vec<(usize, usize)>>,
            to_delete: &mut Vec<usize>,
        ) -> usize {
            let mut subsize = 1;

            for (child, edge_id) in &adj[curr] {
                if *child == par {
                    continue;
                }
                let get = dfs(*child, curr, adj, to_delete);
                subsize += get;

                if get % 3 == 0 {
                    to_delete.push(*edge_id);
                }
            }

            return subsize;
        }
        dfs(0, 0, &adj, &mut to_delete);

        // edges to remove
        // n = 3, remove = 0
        // n = 6, remove = 1
        // n = 9, remove = 2
        // remove = n / 3 - 1
        // 3 * (to_delete.len() + 1) should be n
        // this also ensure that n % 3 == 0

        if 3 * (to_delete.len() + 1) == n {
            writeln!(out, "{}", to_delete.len()).unwrap();
            for del in to_delete {
                write!(out, "{} ", del).unwrap();
            }
            writeln!(out, "").unwrap();
        } else {
            writeln!(out, "-1").unwrap();
        }
    }
}
