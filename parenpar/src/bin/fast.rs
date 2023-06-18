use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self, ScopedJoinHandle},
    time::Instant,
};

struct ThreadInfo<'s, 'i> {
    thread: ScopedJoinHandle<'s, ()>,
    chunk_sender: Sender<&'i str>,
    result_receiver: Receiver<ChunkResult>,
}

struct ChunkResult {
    balance: i64,
    min_balance: i64,
}

fn process_chunk(chunk: &str) -> ChunkResult {
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

fn worker_thread(chunk_receiver: Receiver<&str>, result_sender: Sender<ChunkResult>) {
    for chunk in chunk_receiver {
        result_sender.send(process_chunk(chunk)).unwrap();
    }
}

fn str_chunks(input: &str, chunk_size: usize) -> impl Iterator<Item = &str> {
    (0..(input.len() + chunk_size - 1) / chunk_size).map(move |i| {
        let chunk = input
            .get(i * chunk_size..)
            .expect("invalid parenthesis given as input");
        chunk
            .get(..std::cmp::min(chunk_size, chunk.len()))
            .expect("invalid parenthesis given as input")
    })
}

fn check_valid_parentheses(input: &str) -> bool {
    std::thread::scope(|s| {
        const CHUNK_SIZE: usize = 30000000; // Adjust the chunk size as needed

        let num_threads = thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(10);
        dbg!(num_threads);

        let threads: Vec<ThreadInfo> = (0..num_threads)
            .map(|_| {
                let (chunk_sender, chunk_receiver): (Sender<&str>, Receiver<&str>) = channel();
                let (result_sender, result_receiver): (Sender<ChunkResult>, Receiver<ChunkResult>) =
                    channel();

                ThreadInfo {
                    thread: s.spawn(move || {
                        worker_thread(chunk_receiver, result_sender);
                    }),
                    chunk_sender,
                    result_receiver,
                }
            })
            .collect();

        let mut global_balance: i64 = 0;
        let mut global_min_balance: i64 = i64::max_value();

        for chunk_batch in str_chunks(input, CHUNK_SIZE * num_threads) {
            let mut batch_size = 0;
            for (i, chunk) in str_chunks(chunk_batch, CHUNK_SIZE).enumerate() {
                batch_size += 1;
                threads[i].chunk_sender.send(chunk).unwrap();
            }

            for i in 0..batch_size {
                // preserving the same order in which we sent the chunks
                let chunk_result = threads[i].result_receiver.recv().unwrap();
                global_min_balance =
                    global_min_balance.min(global_balance + chunk_result.min_balance);
                global_balance += chunk_result.balance;
            }
        }

        for thread_info in threads {
            // Notify worker threads that no more chunks will be sent
            drop(thread_info.chunk_sender);
            thread_info.thread.join().unwrap();
        }

        global_balance == 0 && global_min_balance >= 0
    })
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
    let mut parentheses = String::new();
    let length = 5e8 as usize;

    for _ in 0..length {
        parentheses.push('(');
    }
    for _ in 0..length {
        parentheses.push(')');
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
            assert_eq!(answer, result, "{}", s);
        }
    }

    #[test]
    fn test_edge_cases() {
        let tests = [("", true), ("(", false), ("((()))", true)];
        for (s, answer) in tests {
            let result = check_valid_parentheses(s);
            assert_eq!(answer, result, "{}", s);
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

// benchmarks on running
/*
[src/bin/fast.rs:66] num_threads = 8
Brute: 1.506208492s, My: 545.329027ms
OK test 0
[src/bin/fast.rs:66] num_threads = 8
Brute: 1.506578678s, My: 471.840766ms
OK test 1
[src/bin/fast.rs:66] num_threads = 8
Brute: 1.506094206s, My: 513.329575ms
OK test 2
[src/bin/fast.rs:66] num_threads = 8
Brute: 1.509122298s, My: 320.703203ms
OK test 3
[src/bin/fast.rs:66] num_threads = 8
Brute: 1.50760297s, My: 319.286005ms
OK test 4
[src/bin/fast.rs:66] num_threads = 8
Brute: 1.506348399s, My: 318.853544ms
OK test 5
[src/bin/fast.rs:66] num_threads = 8
Brute: 1.506928909s, My: 319.498627ms
OK test 6
[src/bin/fast.rs:66] num_threads = 8
Brute: 1.506651525s, My: 334.447186ms
OK test 7
[src/bin/fast.rs:66] num_threads = 8
Brute: 3.184727097s, My: 699.577349ms
OK test 8
[src/bin/fast.rs:66] num_threads = 8
Brute: 2.002453219s, My: 669.600844ms
OK test 9
*/
