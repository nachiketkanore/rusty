use crate::p03::max;

pub fn solve() -> i64 {
    let mut ans = 0;
    for one in 100..999 {
        for two in 100..999 {
            let prod = one * two;
            let first: String = prod.to_string();
            let second: String = first.chars().rev().collect();
            if first == second {
                ans = max(ans, prod);
            }
        }
    }
    ans
}
