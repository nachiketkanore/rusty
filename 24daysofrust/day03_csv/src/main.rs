use std::error::Error;
use std::io;
use std::process;

#[allow(dead_code)]
fn example1() -> Result<(), Box<dyn Error>> {
    // stdin to stdout csv reading
    let mut reader = csv::Reader::from_reader(io::stdin());

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

use serde::Deserialize;

#[derive(Deserialize)]
struct Record {
    year: u16,
    make: String,
    model: String,
    description: String,
}

fn example2() -> Result<(), csv::Error> {
    // let csv = "year,make,model,description
    //     1948,Porsche,356,Luxury sports car
    //     1967,Ford,Mustang fastback 1967,American car";

    use std::fs;

    let doc = fs::read_to_string("src/data.csv")?;

    let mut reader = csv::Reader::from_reader(doc.as_bytes());

    for record in reader.deserialize() {
        let record: Record = record?;
        println!(
            "In {}, {} built the {} model. It is a {}.",
            record.year, record.make, record.model, record.description
        );
    }

    Ok(())
}

#[allow(unreachable_code)]
fn main() {
    // csv reading
    println!("Reading csv and printing");
    let csv = r"year,make,model,description
        1948,Porsche,356,Luxury sports car
        1967,Ford,Mustang fastback 1967,American car";

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    for line in reader.records() {
        let record = line.expect("parsing failed");

        println!(
            "In {}, {} built the {} model. It is a {}.",
            &record[0], &record[1], &record[2], &record[3]
        );
    }

    // // csv parser rust crates
    // if let Err(err) = example1() {
    //     println!("Error from example() function: {}", err);
    //     process::exit(1);
    // }

    println!("Deserialiation");
    // deserialize csv to rust struct
    if let Err(err) = example2() {
        println!("Error from example() function: {}", err);
        process::exit(1);
    }
}
