use std::io;

fn read_int() -> i32 {
    let mut buff = String::new();
    io::stdin()
        .read_line(&mut buff)
        .expect("Unable to read line");
    buff.trim().parse().unwrap()
}

fn main() -> io::Result<()> {
    (0..read_int())
        .map(|_| read_int())
        .map(|k| {
            (1..)
                .filter(|num| num % 3 != 0 && num % 10 != 3)
                .nth((k - 1) as usize)
                .unwrap()
        })
        .for_each(|ans| println!("{}", ans));
    Ok(())
}

#[test]
fn samples() {
    fn parse_input(path: &str) -> Vec<i32> {
        path.split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.trim().parse().unwrap())
            .collect()
    }
    let input: Vec<i32> = parse_input(include_str!("../data/sample.in"));
    let output: Vec<i32> = parse_input(include_str!("../data/sample.out"));
    dbg!(input, output);
    // assert_eq!(solve(input[1..].to_vec()), output);
}
