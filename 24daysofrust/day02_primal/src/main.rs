use primal;
use primal::Sieve;

fn main() {
    // get N-th prime
    let n = 1000;
    let p = primal::Primes::all().nth(n - 1);
    println!("{}-th prime = {:?}", n, p);

    // some fiddling around with primal

    // ns goes upto 100_00
    let ns = (1..=100).map(|x| x * 1000).collect::<Vec<_>>();

    let sum = ns
        .iter()
        .map(|n| primal::StreamingSieve::nth_prime(*n))
        .fold(0, |acc, val| acc + val);

    println!("The sum (weird) is = {}", sum);

    // Project Euler task:
    // Count divisors using formula with prime factorization

    let sieve = Sieve::new(10000);
    let divisors = count_divisors(2610, &sieve);

    println!("Divisors of {} is = {:?}", 2610, divisors);
}

fn count_divisors(n: usize, sieve: &Sieve) -> Option<usize> {
    match sieve.factor(n) {
        Ok(factors) => Some(
            factors
                .iter()
                .fold(1, |acc, (_prime, expo)| acc * (expo + 1)),
        ),
        Err(_) => panic!("factors not received"),
    }
}
