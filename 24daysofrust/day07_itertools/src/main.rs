use itertools::iproduct;
use itertools::Itertools;

#[allow(unstable_name_collisions)]
fn main() {
    // interleaving: take alternate values from first and second containers
    let it = (1..5)
        .interleave(vec![1, 2, 3, 4, 5, 6, 7])
        .collect::<Vec<i32>>();
    for val in it {
        println!("{}", val);
    }
    // foreach: similar to map(|closure|)
    let words = "nachiket kanore hello world how are you?".split(|ch| ch == ' ');
    words.for_each(|word| {
        println!("{}", word);
    });

    // interleave iterators -> collect to containers
    let even = (1..10).map(|x| 2 * x);
    let odd = (1..10).map(|x| 2 * x + 1);

    // type = Interleave {interface}
    // let result = even.interleave(odd);
    // type = Vec<i32> or anything we want
    let result = even.interleave(odd).collect::<Vec<u32>>();
    for val in result {
        println!("{}", val);
    }

    // single liner ;)
    const RANGE: i32 = 10;
    let one_liner_result = (1..RANGE)
        .map(|x| 2 * x)
        .interleave((1..RANGE).map(|x| 2 * x + 1))
        .collect::<Vec<i32>>();

    println!("{:?}", one_liner_result);
    // intersperse: alternate with a single value
    // basically adding X in between each adjacent elements in container
    let result = (1..10).intersperse(21).collect::<Vec<u32>>();
    print!("{:?} ", result);

    // iproduct! : similar to cartesian product?
    let first = 1..4;
    let second = vec!["nachiket", "kanore", "hello"];

    for (id, (one, two)) in iproduct!(first, second).enumerate() {
        println!("{} {} {}", id, one, two);
    }

    let tp = vec![20, 30, 40];

    for (id, val) in tp.iter().enumerate() {
        println!("id = {}, val = {}", id, val);
    }
}
