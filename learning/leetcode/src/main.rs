fn array_sign(nums: Vec<i32>) -> i32 {
    nums.iter()
        .map(|val| match *val {
            val if val > 0 => 1,
            val if val < 0 => -1,
            _ => 0,
        })
        .fold(1, |res, val| res * val)
}

fn main() {
    let v = vec![-1, -2, -3, -4, 3, 2, 1];
    dbg!(array_sign(v));
}

#[test]
fn test_array_sign() {
    assert_eq!(array_sign(vec![-1, -2, -3, -4, 3, 2, 1]), 1);
    assert_eq!(array_sign(vec![1, 5, 0, 2, -3]), 0);
    assert_eq!(array_sign(vec![-1, 1, -1, 1, -1]), -1);
    assert_eq!(array_sign(vec![0, 0, 0]), 0);
}
