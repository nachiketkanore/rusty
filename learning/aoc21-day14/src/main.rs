use std::collections::HashMap;

fn main() {
    let s: Vec<&str> = include_str!("sample")
        .split('\n')
        .filter(|line| line.len() > 0)
        .collect();
    let initial = s[0];
    let s: Vec<String> = s.iter().skip(1).map(|line| line.to_string()).collect();
    let mut mappings = HashMap::<String, String>::new();
    s.iter().for_each(|line| {
        let mut splits = line.split(" -> ");
        let first = splits.next().unwrap().to_string();
        let second = splits.next().unwrap().to_string();
        mappings.insert(first, second);
    });
    let mut cnt = HashMap::<String, u32>::new();
    for i in 0..initial.len() - 1 {
        let add = String::from(&initial[i..=i + 1]).clone();
        *cnt.entry(add).or_insert(0) += 1;
    }
    dbg!(&cnt);
    dbg!(initial, s);
    for step_id in 0..5 {
        let mut ncnt = HashMap::<String, u32>::new();
        dbg!(step_id, &cnt);
        for (curr, freq) in &cnt {
            assert!(curr.len() == 2);
            assert!(mappings.contains_key(curr));
            let becomes = mappings.get(curr).unwrap();
            assert!(becomes.len() == 1);
            let mut first = String::new();
            first.push_str(&curr[0..0]);
            first.push_str(becomes);
            let mut second = String::new();
            second.push_str(becomes);
            second.push_str(&curr[1..1]);

            *ncnt.entry(first).or_insert(0) += freq;
            *ncnt.entry(second).or_insert(0) += freq;
        }

        cnt = ncnt;
    }
}
