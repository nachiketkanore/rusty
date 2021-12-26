use std::collections::HashMap;

fn test_fn() {
    let mut mymap = HashMap::new();
    {
        let x = vec![1, 2, 3];
        let y = 100;
        mymap.insert(x, y);
    }
    {
        let x = vec![4, 5, 6];
        let y = 200;
        mymap.insert(x, y);
    }

    let result = mymap.iter().fold(Vec::<i32>::new(), |mut acc, (key, _)| {
        println!("{:?}", key);
        for k in key {
            acc.push(*k);
        }
        acc
    });

    println!("{:?}", result);
}

fn main() -> Result<(), ()> {
    test_fn();
    Ok(())
}
