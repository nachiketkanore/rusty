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

    let test_cases: usize = scan.next();
    (0..test_cases)
        .fold(Vec::<i64>::new(), |mut result, _| {
            let (n, q): (usize, usize) = (scan.next(), scan.next());
            let mut sum_matrix = [[0i64; 1001]; 1001];
            (0..n)
                .map(|_| (scan.next::<usize>(), scan.next::<usize>()))
                .for_each(|(h, w)| {
                    sum_matrix[h][w] += h as i64 * w as i64;
                });

            for i in 1..1000 {
                for j in 1..1000 {
                    sum_matrix[i][j] += sum_matrix[i - 1][j];
                    sum_matrix[i][j] += sum_matrix[i][j - 1];
                    sum_matrix[i][j] -= sum_matrix[i - 1][j - 1];
                }
            }

            let submatrix_sum = |h1: usize, w1: usize, h2: usize, w2: usize| {
                sum_matrix[h2][w2] - sum_matrix[h2][w1 - 1] - sum_matrix[h1 - 1][w2]
                    + sum_matrix[h1 - 1][w1 - 1]
            };

            result.append(
                &mut (0..q)
                    .map(|_| {
                        (
                            scan.next::<usize>(),
                            scan.next::<usize>(),
                            scan.next::<usize>(),
                            scan.next::<usize>(),
                        )
                    })
                    .map(|(h1, w1, h2, w2)| submatrix_sum(h1 + 1, w1 + 1, h2 - 1, w2 - 1))
                    .collect(),
            );
            result
        })
        .iter()
        .for_each(|answer| {
            writeln!(out, "{}", answer).ok();
        });
}
