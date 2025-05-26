fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase()
            } else if c.is_uppercase() {
                c.to_lowercase()
            } else {
                std::iter::once(c)
            }
        })
        .flatten()
        .collect()
}