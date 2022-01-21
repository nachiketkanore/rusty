use itertools::Itertools;

pub fn solve() {
    // Part one
    let (first, second) = include_str!("../data/01.input")
        .split('\n')
        .filter(|val| val.len() > 0)
        .map(|val| val.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
        .into_iter()
        .tuple_combinations::<(_, _)>()
        .find(|(first, second)| first + second == 2020)
        .unwrap();
    println!("Part 1: {}", first * second);

    // Part two
    let (first, second, third) = include_str!("../data/01.input")
        .split('\n')
        .filter(|val| val.len() > 0)
        .map(|val| val.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
        .into_iter()
        .tuple_combinations::<(_, _, _)>()
        .find(|(first, second, third)| first + second + third == 2020)
        .unwrap();
    println!("Part 2: {}", first * second * third);
}
