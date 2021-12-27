pub fn hello() {
    println!("file1");
}

#[allow(dead_code)]
fn test() {
    let _x: String = "nachiket".to_string();
    let _y = _x.chars().filter(|ch| *ch == 'n' || *ch == 't');
}
