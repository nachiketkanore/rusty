use anyhow::Result;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() -> Result<()> {
    println!("Guessing game\nGuess the integer from 1 to 100");

    let target_num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Output your guess.");

        let mut user_guess = String::new();

        io::stdin()
            .read_line(&mut user_guess)
            .expect("failed to read line from stdin");

        let guess: u32 = match user_guess.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        // other way to parse with expect?
        // let guess2: u32 = user_guess.trim().parse()?;

        println!("You guessed {}", guess);

        match guess.cmp(&target_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("Correct guess\nTarget number = {}", target_num);
                break;
            }
        }
    }

    println!("Thank you for playing");
    Ok(())
}
