use std::fs::OpenOptions;
use std::io::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Name
    name: String,
    /// College Name
    college: String,
    /// College Reg. ID
    id: u32,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("data.txt")
        .unwrap();

    let content = format!("{:?}\n", opt);

    file.write_all(content.as_bytes())?;

    /*
       TODO:
        - collect all data from data.txt into struct
        - validate whether this entry already exists
        - notify user if it already exists
        - multiple entries in single cli possible?
    */

    Ok(())
}
