use rand::Rng;
use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self, JoinHandle},
    time::Instant,
};

struct ThreadInfo {
    thread: JoinHandle<()>,
    chunk_sender: Sender<String>,
    result_receiver: Receiver<ChunkResult>,
}

struct ChunkResult {
    balance: i64,
    min_balance: i64,
}

fn process_chunk(chunk: String) -> ChunkResult {
    let mut balance = 0;
    let mut min_balance: i64 = i64::max_value();

    for c in chunk.chars() {
        match c {
            '(' => balance += 1,
            ')' => balance -= 1,
            _ => {
                panic!("invalid parenthesis given as input")
            }
        }

        if balance < min_balance {
            min_balance = balance;
        }
    }

    ChunkResult {
        balance,
        min_balance,
    }
}

fn worker_thread(chunk_receiver: Receiver<String>, result_sender: Sender<ChunkResult>) {
    for chunk in chunk_receiver {
        result_sender.send(process_chunk(chunk)).unwrap();
    }
}

fn check_valid_parentheses(input: &str) -> bool {
    const CHUNK_SIZE: usize = 1000000; // Adjust the chunk size as needed
    const NUM_THREADS: usize = 8; // Adjust the number of threads as needed

    let chunks: Vec<_> = input.chars().collect();

    let threads: Vec<ThreadInfo> = (0..NUM_THREADS)
        .map(|_| {
            let (chunk_sender, chunk_receiver): (Sender<String>, Receiver<String>) = channel();
            let (result_sender, result_receiver): (Sender<ChunkResult>, Receiver<ChunkResult>) =
                channel();

            ThreadInfo {
                thread: thread::spawn(move || {
                    worker_thread(chunk_receiver, result_sender);
                }),
                chunk_sender,
                result_receiver,
            }
        })
        .collect();

    let mut global_balance: i64 = 0;
    let mut global_min_balance: i64 = i64::max_value();

    for chunk_batch in chunks.chunks(CHUNK_SIZE * NUM_THREADS) {
        let mut batch_size = 0;
        chunk_batch
            .chunks(CHUNK_SIZE)
            .map(|chunk| chunk.iter().collect::<String>())
            .enumerate()
            .for_each(|(id, chunk)| {
                batch_size += 1;
                threads[id].chunk_sender.send(chunk).unwrap();
            });
        for i in 0..batch_size {
            // preserving the same order in which we sent the chunks
            let chunk_result = threads[i].result_receiver.recv().unwrap();
            global_min_balance = global_min_balance.min(global_balance + chunk_result.min_balance);
            global_balance += chunk_result.balance;
        }
    }

    for thread_info in threads {
        // Notify worker threads that no more chunks will be sent
        drop(thread_info.chunk_sender);
        thread_info.thread.join().unwrap();
    }

    global_balance == 0 && global_min_balance >= 0
}
fn main() {
    for id in 0..10 {
        let s = generate_random_parentheses();

        let start_time = Instant::now();
        let correct = brute_checker(&s);
        let brute_time = start_time.elapsed();

        let start_time = Instant::now();
        let solution = check_valid_parentheses(&s);
        let my_time = start_time.elapsed();

        println!("Brute: {:?}, My: {:?}", brute_time, my_time);
        assert_eq!(solution, correct);
        println!("OK test {}", id);
    }
}

fn brute_checker(s: &str) -> bool {
    let (mut bal, mut min_bal) = (0, i64::MAX);
    for ch in s.chars() {
        if ch == '(' {
            bal += 1;
        } else if ch == ')' {
            bal -= 1;
        } else {
            panic!("bad input for string");
        }
        min_bal = min_bal.min(bal);
    }
    bal == 0 && min_bal >= 0
}

fn generate_random_parentheses() -> String {
    let mut rng = rand::thread_rng();
    let mut parentheses = String::new();
    let mut balance = 0;
    let length = 1e8 as usize;

    while parentheses.len() < length {
        let random_char = if balance > 0 && rng.gen::<f64>() < 0.5 {
            balance -= 1;
            ')'
        } else {
            balance += 1;
            '('
        };

        parentheses.push(random_char);
    }

    while balance > 0 {
        parentheses.push(')');
        balance -= 1;
    }

    parentheses
}

#[cfg(test)]
mod tests {
    use crate::{check_valid_parentheses, generate_random_parentheses};

    #[test]
    fn test_valid_parentheses() {
        let tests = [
            ("()()()()()()()()()()()()", true),
            ("(((((((((((((((((((((((((())))))))))))))))))))))))))", true),
            ("((((((()))))))(((((((())))))))(((((((())))))))", true),
        ];
        for (s, answer) in tests {
            let result = check_valid_parentheses(s);
            assert_eq!(answer, result);
        }
    }

    #[test]
    fn test_invalid_parentheses() {
        let tests = [
            ("))))(((())))", false),
            ("((()))((", false),
            ("())))", false),
        ];
        for (s, answer) in tests {
            let result = check_valid_parentheses(s);
            assert_eq!(answer, result);
        }
    }

    #[test]
    fn test_edge_cases() {
        let tests = [("", true), ("(", false), ("((()))", true)];
        for (s, answer) in tests {
            let result = check_valid_parentheses(s);
            assert_eq!(answer, result);
        }
    }

    #[test]
    fn test_large_input_valid() {
        let mut s = "(".repeat(10000);
        s.push_str(&")".repeat(10000));
        assert_eq!(check_valid_parentheses(s.as_str()), true);
    }

    #[test]
    fn test_large_input_invalid() {
        let s = ")".repeat(1000000);
        assert_eq!(check_valid_parentheses(s.as_str()), false);
    }

    #[test]
    fn test_random_large_input_valid() {
        for _ in 0..5 {
            let s = generate_random_parentheses();
            assert_eq!(check_valid_parentheses(s.as_str()), true);
        }
    }
}
// Benchmark logs:
//
// Parameters:
// const CHUNK_SIZE: usize = 1000000; // Adjust the chunk size as needed
// const NUM_THREADS: usize = 8; // Adjust the number of threads as needed
// let length = 1e8 as usize;
//
/*
Brute: 1.143071289s, My: 1.059856668s
OK test 0
Brute: 1.140490921s, My: 1.087086417s
OK test 1
Brute: 1.142042704s, My: 1.087540431s
OK test 2
Brute: 1.140149592s, My: 1.088812247s
OK test 3
Brute: 1.138979688s, My: 1.090767189s
OK test 4
Brute: 1.138598026s, My: 1.121822518s
OK test 5
Brute: 1.141424134s, My: 1.102247529s
OK test 6
Brute: 1.141956544s, My: 1.106900144s
OK test 7
Brute: 1.139567871s, My: 1.088665193s
OK test 8
Brute: 1.140854629s, My: 1.087422395s
OK test 9
*/
