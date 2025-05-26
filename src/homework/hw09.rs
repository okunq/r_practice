fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s;
    }

    // обмежуємо зсув до межі довжини рядка
    let shift = ((n % len) + len) % len;
    let split_at = shift as usize;

    let (left, right) = s.split_at(split_at);
    format!("{}{}", right, left)
}
