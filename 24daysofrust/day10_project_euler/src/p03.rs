pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

#[allow(dead_code)]
pub fn solve2() {
    println!("here");
    let num = 600851475143i64;
    let divisors: Vec<i64> = (2..)
        .take_while(move |val| val * val <= num)
        .filter(move |val| num % val == 0)
        .collect::<Vec<i64>>();
    println!("{:?}", divisors);
}

pub fn solve() -> i64 {
    let mut num = 600851475143;
    let mut ans = 0;
    for d in (2..).take_while(move |val| val * val <= num) {
        if num % d == 0 {
            ans = max(ans, d);
            while num % d == 0 {
                num /= d;
            }
        }
    }
    if num != 1 {
        ans = max(ans, num);
        num /= num;
    }
    assert!(num == 1);
    assert!(ans > 0);
    ans
}
