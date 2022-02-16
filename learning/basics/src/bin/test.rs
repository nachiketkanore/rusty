#![allow(dead_code)]

#[derive(Debug)]
struct MyError {
    err: &'static str,
}
impl MyError {
    fn new(err: &'static str) -> Self {
        MyError { err }
    }
}

fn print_type_name<T>(_value: &T) {
    let types = dbg!(std::any::type_name::<T>());
    dbg!(types);
}

struct Pair<A, B, C, D> {
    first: A,
    second: B,
    third: C,
    fourth: D,
}

fn main() -> Result<(), MyError> {
    // let it = 0..20;
    // it.rev().for_each(|val| println!("{}", val));
    let var = (1, 2, 3);
    print_type_name(&var);
    let var = Pair {
        first: vec![1, 2, 3],
        second: String::from("nachiket"),
        third: (1, 2),
        fourth: "hello",
    };
    print_type_name(&var);
    Ok(())
    // Err(MyError::new("Some error occured"))
}
