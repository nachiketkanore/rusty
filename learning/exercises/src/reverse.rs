pub fn test() {
    println!("test function");
}
pub fn reverse(input: &str) -> String {
    String::from(input).chars().rev().collect::<String>()
}
