use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i128
    }
    let mut mat = vec![vec![]; n];
    for i in 0..n {
        input! {
            len: usize,
            row: [i32; len]
        }
        mat[i] = row;
    }

    fn dfs(id: usize, prod: i128, n: usize, x: i128, mat: &Vec<Vec<i32>>) -> i64 {
        let mut ans = 0;
        if prod > x {
            return 0;
        }
        if id == n {
            return (prod == x) as i64;
        }
        for &take in &mat[id] {
            ans += dfs(id + 1, prod * (take as i128), n, x, mat);
        }
        return ans;
    }

    println!("{}", dfs(0, 1, n, x, &mat));
}
