pub fn solve() -> i64 {
    let mut vals = Vec::<i64>::new();
    vals.push(1);
    vals.push(2);
    while vals[vals.len() - 1] <= 4e6 as i64 {
        let len = vals.len();
        vals.push(vals[len - 1] + vals[len - 2]);
    }
    let ans = vals.iter().filter(|x| *x % 2 == 0).sum();
    return ans;
}
