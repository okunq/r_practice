fn main() {
    let levels = 5; 
    let mut output = String::new();

    for level in 1..=levels {
        for i in 1..=level {
            let spaces = levels - i;
            let stars = 2 * i - 1;

            output.extend(std::iter::repeat(' ').take(spaces));
            output.extend(std::iter::repeat('*').take(stars));
            output.push('\n');
        }
    }

    print!("{}", output);
}