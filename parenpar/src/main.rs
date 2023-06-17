use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self, JoinHandle},
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
    let mut min_balance = 0;

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
    const CHUNK_SIZE: usize = 5; // Adjust the chunk size as needed
    const NUM_THREADS: usize = 10; // Adjust the number of threads as needed

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
    let s = "()(())()()(((())))";
    println!("string = {}, valid = {}", s, check_valid_parentheses(s));
}

#[cfg(test)]
mod tests {
    use crate::check_valid_parentheses;

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
}
