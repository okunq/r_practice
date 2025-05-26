const H: usize = 6; 

fn main() {
    let mut output = String::new();

    
    for i in 0..=H {
        let spaces = H - i;
        let stars = 2 * i + 1;

        output.push_str(&" ".repeat(spaces));
        output.push_str(&"*".repeat(stars));
        output.push('\n');
    }

    
    for i in (0..H).rev() {
        let spaces = H - i;
        let stars = 2 * i + 1;

        output.push_str(&" ".repeat(spaces));
        output.push_str(&"*".repeat(stars));
        output.push('\n');
    }

    print!("{}", output); 
}
