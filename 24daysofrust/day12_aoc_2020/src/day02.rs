#[derive(Debug)]
struct Data {
    atleast: usize,
    atmost: usize,
    which: char,
    value: String,
}

pub fn solve() {
    // let data: Vec<String> = include_str!("../data/02.sample")
    let data: Vec<String> = include_str!("../data/02.input")
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| line.to_string())
        .collect();

    let mut clean_data: Vec<Data> = Vec::new();

    data.iter()
        .map(|line| line.split(' ').collect())
        .for_each(|item: Vec<_>| {
            let (from, to) = item[0].split_once('-').unwrap();
            let ch: char = item[1].as_bytes()[0].into();
            let val: String = item[2].to_string();
            clean_data.push(Data {
                atleast: from.parse().unwrap(),
                atmost: to.parse().unwrap(),
                which: ch,
                value: val,
            });
        });
    // dbg!(&clean_data);
    let ans1: usize = clean_data.iter().fold(0, |acc, data| {
        let cnt = data.value.matches(data.which).count();
        match data.atleast <= cnt && cnt <= data.atmost {
            true => acc + 1,
            false => acc,
        }
    });
    println!("Part 1: {}", ans1);

    let ans2: usize = clean_data.iter().fold(0, |acc, data| {
        let (want, id1, id2, look) = (data.which, data.atleast, data.atmost, &data.value);
        let one = look.as_bytes()[id1 - 1] as char == want;
        let two = look.as_bytes()[id2 - 1] as char == want;
        acc + (one ^ two) as usize
    });
    println!("Part 2: {}", ans2);
}
