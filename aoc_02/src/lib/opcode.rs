pub fn compute(code: i32, left: i32, right: i32) -> Option<i32> {
    match code {
        1 => Some(left + right),
        2 => Some(left * right),
        // Consider anything else as None, even 99
        _ => None,
    }
}
