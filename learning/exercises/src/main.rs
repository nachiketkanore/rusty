struct SimpleTasks {}

#[allow(dead_code)]
fn hello_world(name: String) -> String {
    name.chars().rev().collect::<String>()
}

use std::collections::HashMap;
use std::collections::HashSet;

#[allow(dead_code)]
fn use_test() {
    let mut map = HashMap::new();

    map.insert("name", "nachiket");
    map.insert("roll_no", "41231");
    map.insert("test", "1232423");
    map.insert("school", "PICT");
    map.insert("city", "pune");
    let values = map.iter().fold(String::new(), |mut result, (_, value)| {
        result.push_str(value);
        result.push_str(" ");
        result
    });
    let keys = map.iter().fold(String::new(), |mut result, (key, _)| {
        result.push_str(key);
        result.push_str(" ");
        result
    });
    println!("{}, {}", keys, values);
    let mut myset = HashSet::new();
    myset.insert("nachiket");
    myset.insert("kanore");
    myset.insert("nachiket");
    myset.insert("anant");
    let concat: String = myset.iter().fold(String::new(), |mut ret, value| {
        ret.push_str(value);
        ret.push_str(" ");
        ret
    });
    println!("concatenated elements: {}", concat);
}

#[derive(Debug)]
struct Test {
    inputs: Vec<i32>,
    outputs: Vec<i32>,
}

impl SimpleTasks {
    fn reverse_string(s: String) -> String {
        println!("{}", s);
        s.chars().rev().collect()
    }
    fn anagram_check(a: String, b: String) -> bool {
        println!("{} {}", a, b);
        let mut freq = vec![0; 26];
        for ch in a.chars() {
            freq[(ch as u8 - 'a' as u8) as usize] += 1;
        }
        for ch in b.chars() {
            freq[(ch as u8 - 'a' as u8) as usize] -= 1;
        }
        let mut anagram: bool = true;
        for f in freq {
            anagram &= f != 0;
        }
        anagram
    }
    fn array_product_except_self(a: Vec<i32>) -> Vec<i32> {
        let mut answer = Vec::new();
        let mut prefix = a.clone();
        for i in 1..prefix.len() {
            prefix[i] *= prefix[i - 1];
        }
        let mut suffix = a.clone();
        for i in (0..suffix.len() - 1).rev() {
            suffix[i] *= suffix[i + 1];
        }
        for i in 0..a.len() {
            let mut prod = 1;
            if i >= 1 {
                prod *= prefix[i - 1];
            }
            if i + 1 < a.len() {
                prod *= suffix[i + 1];
            }
            answer.push(prod);
        }
        return answer;
    }
    #[allow(dead_code)]
    fn gibberish_code() {
        let def = 20;
        let mat = vec![vec![def; 3]; 5];
        // same as C-style multidimensional array like: int mat[5][3];
        // Notice the reverse order of dimension sizes
        println!("{:?}", mat);
    }
}

pub fn reverse(input: &str) -> String {
    let ret = String::from(input);
    ret.chars().rev().collect::<String>()
}

fn main() {
    use_test();
    println!(
        "{}",
        SimpleTasks::reverse_string("nachiket kanore".to_string())
    );
    println!(
        "{}",
        SimpleTasks::anagram_check("nachiket".to_string(), "tekichan".to_string())
    );
    println!(
        "{:?}",
        SimpleTasks::array_product_except_self(vec![1, 3, 10])
    );
}
