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

fn test_deferencing() {
    struct S {
        data: i32,
    }
    let x = S { data: 20 };
    let y = &x;
    let z = &y;
    let a = &z;
    let val = 20;
    // The * operator can be used to dereference, but you don't need to do that to access fields or call methods
    // It doesn't work similar with primitive types? Not sure

    if a.data == val {
        println!("it works");
    }
}

fn test_closures() {
    // closures are just functions of type Fn, FnMut, FnOnce
    fn this_function_takes_a_function<F>(my_func: F)
    where
        F: Fn(&'static str),
    {
        my_func("nachiket");
        my_func("kanore");
        my_func("pict pune");
    }

    this_function_takes_a_function(|stuff| {
        println!("{}", stuff);
    });
}

fn test_overflow() {
    // rust literally panic!() at overflow
    // how wonderful
    let a = if false { 1e9 } else { 1e4 } as i32;
    let b = if false { 1e9 } else { 1e4 } as i32;
    dbg!(a * b);
}

fn main() -> Result<(), MyError> {
    // let it = 0..20;
    // it.rev().for_each(|val| println!("{}", val));
    let var = (1, 2, 3);
    print_type_name(&var);
    test_overflow();
    test_deferencing();
    test_closures();
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
