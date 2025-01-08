use std::{fs::File, io::Read, process};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// name of the file to print
    #[arg(short, long)]
    file_name: String,

    /// starting line range
    /// 1-based indexing
    #[arg(short, long, default_value_t = 1)]
    start: usize,

    /// ending line range
    /// 1-based indexing
    #[arg(short, long)]
    end: usize,

    /// Flag to print file metadata
    /// default = Falsej
    #[arg(short, long, default_value_t = false)]
    print_meta: bool,
}

fn main() {
    let args = Arguments::parse();
    let file_name = args.file_name;

    let mut data = String::new();
    let mut f = File::open(&file_name).expect("unable to open file: {}");
    f.read_to_string(&mut data)
        .expect("unable to read from the file: {}");

    let lines: Vec<&str> = data.lines().collect();

    if !(1 <= args.start && args.start <= args.end && args.end <= lines.len()) {
        println!("Invalid ranges provided {}-{}", args.start, args.end);
        println!("{} contains {} lines", file_name, lines.len());
        println!("Provide file number ranges between 1 and {}", lines.len());
        process::exit(0);
    }
    if args.print_meta {
        println!("{} contains {} lines", file_name, lines.len());
    }
    for i in args.start..=args.end {
        println!("{}", lines[i - 1]);
    }
}
