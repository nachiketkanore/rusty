use std::fs::File;
use std::io::prelude::*;
use std::io::{self};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("foo.txt")?;
    // file.write_all(b"Hello, world!")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    for (id, line) in contents.split('\n').enumerate() {
        println!("{} {}", id, line);
    }
    {
        if let Ok(lines) = read_lines("./test.txt") {
            for (pos, line) in lines.enumerate() {
                print!("{} --> ", pos);
                if let Ok(my_line) = line {
                    println!("{}", my_line);
                }
            }
        }
    }
    Ok(())
}
