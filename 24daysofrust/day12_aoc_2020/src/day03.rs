pub fn solve() {
    let mat: Vec<Vec<char>> = include_str!("../data/03.input")
        .split('\n')
        .map(|line| line.chars().collect())
        .filter(|item: &Vec<char>| item.len() > 0)
        .collect();
    // dbg!(&mat);
    let (n, m) = (mat.len(), mat[0].len());
    let slopes = vec![[1, 1], [1, 3], [1, 5], [1, 7], [2, 1]];
    let compute = |slope: &[usize; 2]| -> i32 {
        let (mut i, mut j) = (0usize, 0usize);
        let mut ans = 0;
        loop {
            if (0..n).contains(&i) {
                if mat[i][j % m] == '#' {
                    ans += 1;
                }
                i += slope[0];
                j += slope[1];
            } else {
                break;
            }
        }
        ans
    };
    let ans1 = compute(&slopes[1]);
    let ans2: u64 = slopes
        .iter()
        .map(|slope| compute(&slope))
        .collect::<Vec<i32>>()
        .iter()
        .fold(1u64, |res, score| res * (*score as u64));
    println!("Day 3: Part 1 = {}", ans1);
    println!("Day 3: Part 1 = {}", ans2);
    // Answers:
    // Day 3: Part 1 = 286
    // Day 3: Part 1 = 3638606400
}
