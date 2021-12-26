#[allow(dead_code)]
fn test() -> &'static str {
    "hello"
}

#[cfg(tests)]
mod tests {
    assert_eq!(test(), "hello");
}
