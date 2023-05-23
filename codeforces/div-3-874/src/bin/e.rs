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
        let n: usize = scan.next();
        let mut adj: Vec<HashSet<usize>> = vec![HashSet::default(); n];
        for u in 0..n {
            let v = scan.next::<usize>() - 1;
            adj[u].insert(v);
            adj[v].insert(u);
        }
        let mut visited = vec![false; n];
        let mut b = 0;
        let mut c = 0;

        fn dfs(
            curr: usize,
            visited: &mut Vec<bool>,
            adj: &Vec<HashSet<usize>>,
            component: &mut Vec<usize>,
        ) {
            if !visited[curr] {
                visited[curr] = true;
                component.push(curr);
                for child in &adj[curr] {
                    dfs(*child, visited, adj, component);
                }
            }
        }

        for i in 0..n {
            if !visited[i] {
                let mut component = Vec::new();
                dfs(i, &mut visited, &adj, &mut component);
                let mut cycle = true;
                for c in &component {
                    cycle &= adj[*c].len() >= 2;
                }

                if cycle {
                    c += 1;
                } else {
                    b += 1;
                }
            }
        }
        writeln!(out, "{} {}", (b > 0) as usize + c, b + c).unwrap();
    }
}
