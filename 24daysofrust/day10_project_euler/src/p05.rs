use gcd::Gcd;

pub fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / a.gcd(b)
}

fn brute_solve() {
    let bound = 10;
    let ans = (1..)
        .take_while(|val| {
            let mut divisible = true;
            for d in 1..=bound {
                divisible &= val % d == 0;
            }
            !divisible
        })
        .max();

    if let Some(x) = ans {
        println!("bound = {}, answer = {}", bound, x + 1);
    } else {
        println!("Answer not found");
    }
}

pub fn solve() -> u64 {
    let bound = 20;
    // brute force
    // dbg!(brute_solve());

    // optimal: using LCM
    let mut ans: u64 = 1;
    for val in 1..=bound {
        ans = lcm(ans, val);
    }

    ans
}
