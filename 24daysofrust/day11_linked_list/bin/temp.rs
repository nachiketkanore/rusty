fn hello_nachiket() -> i32 {
    (0..100)
        .rev()
        .take_while(|val| val % 10 != 0)
        .last()
        .unwrap()
}

fn main() {
    test();
    dbg!(hello_nachiket());
}

fn test() {
    let x = (1..1000)
        .map(|x| x * x)
        .filter(|x| x < &1000)
        .filter(|x| x < &500)
        .max();
    let y: u32 = (1u32..=10u32).map(|x| x * x).max().unwrap();
    let t = (1..=10)
        .map(|x| x * x)
        .filter(|x| x < &50)
        .map(|x| x + 10)
        .max()
        .unwrap();
    let z = get_val(20);
    dbg!(x, y, z, t);
}

fn get_val(a: i32) -> Vec<i32> {
    let mut ret = Vec::new();
    ret.push(a);
    ret
}
