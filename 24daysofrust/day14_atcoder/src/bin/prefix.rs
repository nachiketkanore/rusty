use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut heap = BinaryHeap::<Reverse<usize>>::new();
    for val in a {
        heap.push(Reverse(val));
        if heap.len() > k {
            heap.pop();
        }
        if heap.len() >= k {
            println!("{}", heap.peek().unwrap().0);
        }
    }
}
